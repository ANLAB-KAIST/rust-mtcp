extern crate bindgen;

use std::env;
use std::fs::*;
use std::path::*;
use std::process::Command;

#[derive(Default)]
struct State {
    project_path: Option<PathBuf>,
    include_path: Option<PathBuf>,
    library_path: Option<PathBuf>,
}

fn check_os(_: &mut State) {
    #[cfg(not(unix))]
    panic!("Currently, only xnix OS is supported.");
}

fn find_mtcp(state: &mut State) {
    if let (Ok(_), Ok(_)) = (env::var("RTE_SDK"), env::var("RTE_TARGET")) {
        let dir_path = Path::new(state.project_path.as_ref().unwrap()).join("3rdparty");
        if !dir_path.exists() {
            create_dir(&dir_path).ok();
        }
        assert!(dir_path.exists());
        let git_path = dir_path.join("mtcp");
        if !git_path.exists() {
            Command::new("git")
                .args(&[
                    "clone",
                    "-b",
                    "portable",
                    "https://github.com/leeopop/mtcp.git",
                    git_path.to_str().unwrap(),
                ])
                .output()
                .expect("failed to run git command");
        }
        Command::new("make")
            .args(&["-C", git_path.to_str().unwrap()])
            .output()
            .expect("failed to run make command");
        state.include_path = Some(git_path.join("include").join("mtcp"));
        state.library_path = Some(git_path.join("lib"));
    } else {
        panic!("mTCP requires RTE_SDK and RTE_TARGET env var to be built");
    }

    assert!(state.include_path.clone().unwrap().exists());
    assert!(state.library_path.clone().unwrap().exists());
}

fn generate_rust_def(state: &mut State) {
    let include_path = state.include_path.clone().unwrap();
    let project_path = state.project_path.clone().unwrap();
    let src_path = project_path.join("src").join("mtcp.rs");
    bindgen::builder()
        .header(project_path.join("mtcp_all.h").to_str().unwrap())
        .clang_arg(format!("-I{}", include_path.to_str().unwrap()))
        .clang_arg("-march=native")
        .clang_arg("-Wno-everything")
        .rustfmt_bindings(true)
        .generate()
        .unwrap()
        .write_to_file(src_path)
        .ok();
}
fn compile(state: &mut State) {
    let lib_path = state.library_path.clone().unwrap();
    println!(
        "cargo:rustc-link-search=native={}",
        lib_path.to_str().unwrap()
    );

    let additional_libs = vec!["mtcp", "gmp", "dpdk", "numa"];
    for lib in &additional_libs {
        println!("cargo:rustc-link-lib={}", lib);
    }
}
fn main() {
    let mut state: State = Default::default();
    state.project_path = Some(PathBuf::from(".").canonicalize().unwrap());
    check_os(&mut state);
    find_mtcp(&mut state);
    generate_rust_def(&mut state);
    compile(&mut state);
}
