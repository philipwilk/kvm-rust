FROM debian:stable
ARG DEBIAN_FRONTEND=noninteractive

COPY cross/debian-scripts/common.sh cross/debian-scripts/lib.sh /
RUN chmod +x common.sh 
RUN /common.sh

COPY cross/debian-scripts/cmake.sh /
RUN chmod +x cmake.sh 
RUN /cmake.sh

COPY cross/debian-scripts/xargo.sh /
RUN chmod +x xargo.sh 
RUN /xargo.sh

RUN apt-get update && apt-get install --assume-yes --no-install-recommends \
  g++-powerpc64-linux-gnu \
  libc6-dev-ppc64-cross

COPY cross/debian-scripts/deny-debian-packages.sh /
RUN chmod +x deny-debian-packages.sh
RUN TARGET_ARCH=ppc64 /deny-debian-packages.sh \
  binutils \
  binutils-powerpc64-linux-gnu

COPY cross/debian-scripts/qemu.sh /
RUN chmod +x qemu.sh
RUN /qemu.sh ppc64 softmmu

COPY cross/debian-scripts/dropbear.sh /
RUN chmod +x dropbear.sh
RUN /dropbear.sh

COPY cross/debian-scripts/linux-image.sh /
RUN chmod +x linux-image.sh
RUN /linux-image.sh powerpc64

COPY cross/debian-scripts/linux-runner cross/debian-scripts/base-runner.sh /
RUN chmod +x linux-runner
RUN chmod +x base-runner.sh

ENV CARGO_TARGET_POWERPC64_UNKNOWN_LINUX_GNU_LINKER=powerpc64-linux-gnu-gcc \
  CARGO_TARGET_POWERPC64_UNKNOWN_LINUX_GNU_RUNNER="/linux-runner powerpc64" \
  CC_powerpc64_unknown_linux_gnu=powerpc64-linux-gnu-gcc \
  CXX_powerpc64_unknown_linux_gnu=powerpc64-linux-gnu-g++ \
  BINDGEN_EXTRA_CLANG_ARGS_powerpc64_unknown_linux_gnu="--sysroot=/usr/powerpc64-linux-gnu" \
  QEMU_LD_PREFIX=/usr/powerpc64-linux-gnu \
  RUST_TEST_THREADS=1 \
  PKG_CONFIG_PATH="/usr/lib/powerpc64-linux-gnu/pkgconfig/:${PKG_CONFIG_PATH}" \
  OPENSSL_INCLUDE_DIR=/usr/include/powerpc64-linux-gnu/openssl/ \
  OPENSSL_LIB_DIR=/usr/lib/powerpc64-linux-gnu/ \
  OPENSSL_DIR=/usr/bin
