FROM debian:latest

ENV RTE_SDK=/usr/local/share/dpdk/
ENV RTE_TARGET=x86_64-native-linuxapp-gcc

RUN apt update -y && apt dist-upgrade -y && apt autoremove -y && apt autoclean -y
RUN apt install -y build-essential libnuma-dev git linux-headers-$(uname -r)

RUN git clone -b releases "https://gitlab.kaist.ac.kr/3rdparty/dpdk" /dpdk

WORKDIR /dpdk

RUN echo "${RTE_TARGET}" > RTE_TARGET_EXPECTED
RUN make defconfig | sed -r 's/(.*)\s(\w+)/\2/g' > RTE_TARGET
RUN diff -w -q RTE_TARGET RTE_TARGET_EXPECTED
RUN make -j$(nproc)
RUN make -j$(nproc) install

WORKDIR /
RUN rm -rf /dpdk

# For rustup
ENV RUSTUP_HOME=/usr/local/rustup
ENV CARGO_HOME=/usr/local/cargo
ENV PATH=/usr/local/cargo/bin:$PATH
RUN apt install -y curl
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | bash -s -- -y --no-modify-path
RUN chmod -R a+w ${RUSTUP_HOME} ${CARGO_HOME}

# Recover env and verify
RUN rustup --version

# For rust-dpdk
RUN apt install -y libclang-dev clang
# For rust-mtcp
RUN apt install -y libgmp-dev