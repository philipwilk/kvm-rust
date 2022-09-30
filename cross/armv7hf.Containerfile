
FROM debian:stable
ARG DEBIAN_FRONTEND=noninteractive

COPY cross/debian-scripts/common.sh cross/debian-scripts/lib.sh /
RUN /common.sh

COPY cross/debian-scripts/cmake.sh /
RUN /cmake.sh

COPY cross/debian-scripts/xargo.sh /
RUN /xargo.sh

RUN apt-get update && apt-get install --assume-yes --no-install-recommends \
  g++-arm-linux-gnueabihf \
  libc6-dev-armhf-cross

COPY cross/debian-scripts/deny-debian-packages.sh /
RUN TARGET_ARCH=armhf /deny-debian-packages.sh \
  binutils \
  binutils-arm-linux-gnueabihf

COPY cross/debian-scripts/qemu.sh /
RUN /qemu.sh arm softmmu

COPY cross/debian-scripts/dropbear.sh /
RUN /dropbear.sh

COPY cross/debian-scripts/linux-image.sh /
RUN /linux-image.sh armv7

COPY cross/debian-scripts/linux-runner cross/debian-scripts/base-runner.sh /

ENV CARGO_TARGET_ARMV7_UNKNOWN_LINUX_GNUEABIHF_LINKER=arm-linux-gnueabihf-gcc \
  CARGO_TARGET_ARMV7_UNKNOWN_LINUX_GNUEABIHF_RUNNER="/linux-runner armv7hf" \
  CC_armv7_unknown_linux_gnueabihf=arm-linux-gnueabihf-gcc \
  CXX_armv7_unknown_linux_gnueabihf=arm-linux-gnueabihf-g++ \
  BINDGEN_EXTRA_CLANG_ARGS_armv7_unknown_linux_gnueabihf="--sysroot=/usr/arm-linux-gnueabihf" \
  QEMU_LD_PREFIX=/usr/arm-linux-gnueabihf \
  RUST_TEST_THREADS=1 \
  PKG_CONFIG_PATH="/usr/lib/arm-linux-gnueabihf/pkgconfig/:${PKG_CONFIG_PATH}" \
  OPENSSL_INCLUDE_DIR=/usr/include/arm-linux-gnueabihf/openssl/ \
  OPENSSL_LIB_DIR=/usr/lib/arm-linux-gnueabihf/ \
  OPENSSL_DIR=/usr/bin
