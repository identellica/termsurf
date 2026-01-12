#--------------------------------------------------------------------
# Alacritty since it has no build in Ubuntu
#--------------------------------------------------------------------
FROM rust AS alacritty

RUN apt-get update && apt-get install -y \
    cmake pkg-config libfreetype6-dev libfontconfig1-dev \
    libxcb-xfixes0-dev python3 git

RUN git clone https://github.com/alacritty/alacritty.git /tmp/alacritty

WORKDIR /tmp/alacritty

RUN git checkout tags/$(git describe --tags $(git rev-list --tags --max-count=1))

RUN RUSTFLAGS='-C link-arg=-s' cargo build --release

#--------------------------------------------------------------------
# Wraptest
#--------------------------------------------------------------------
FROM ubuntu:22.04 AS wraptest

RUN apt-get update && apt-get install -y \
    build-essential git

RUN git clone https://github.com/mattiase/wraptest.git /tmp/wraptest

WORKDIR /tmp/wraptest

RUN gcc -o wraptest wraptest.c

#--------------------------------------------------------------------
# Main Runner
#--------------------------------------------------------------------
# Note: we used to use Alpine, but we need to use an OS that is more
# glibc friendly because on Linux we still require glibc (active todo to
# support musl).
FROM ubuntu:22.04

# Base deps
RUN apt-get update && apt-get install -y \
  fonts-inconsolata \
  i3 \
  imagemagick \
  libgl1-mesa-dev \
  libxcursor1 \
  patchelf \
  software-properties-common \
  xdotool \
  xvfb \
  vttest

# Terminals
RUN apt-get install -y \
  xterm

RUN rm -rf /var/lib/apt/lists/*

COPY --from=alacritty /tmp/alacritty/target/release/alacritty /usr/bin/alacritty
COPY --from=wraptest /tmp/wraptest/wraptest /usr/bin/wraptest

COPY ./run.sh /entrypoint.sh

COPY ./ghostty /usr/bin/ghostty

ENTRYPOINT ["/bin/bash"]
