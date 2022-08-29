FROM debian:stable
ARG DEBIAN_FRONTEND=noninteractive

COPY containerscripts/common.sh containerscripts/lib.sh /
RUN chmod +x common.sh 
RUN /common.sh

COPY containerscripts/cmake.sh /
RUN chmod +x cmake.sh 
RUN /cmake.sh

COPY containerscripts/xargo.sh /
RUN chmod +x xargo.sh 
RUN /xargo.sh

RUN apt-get update && apt-get install --assume-yes --no-install-recommends \
  g++-powerpc64-linux-gnu \
  libc6-dev-ppc64-cross

COPY containerscripts/deny-debian-packages.sh /
RUN chmod +x deny-debian-packages.sh
RUN TARGET_ARCH=ppc64 /deny-debian-packages.sh \
  binutils \
  binutils-powerpc64-linux-gnu

COPY containerscripts/qemu.sh /
RUN chmod +x qemu.sh
RUN /qemu.sh ppc64 softmmu

COPY containerscripts/dropbear.sh /
RUN chmod +x dropbear.sh
RUN /dropbear.sh

COPY containerscripts/linux-image.sh /
RUN chmod +x linux-image.sh
RUN /linux-image.sh powerpc64

COPY containerscripts/linux-runner containerscripts/base-runner.sh /
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
