# borealis-rs
rust reimplement of https://github.com/natinusala/borealis

```shell
cargo run --example window_borealis
```

## windows mingw64

```shell
pacman -S --needed base-devel mingw-w64-x86_64-toolchain
pacman -S mingw-w64-x86_64-SDL2
pacman -S mingw-w64-x86_64-mpv

export CC=gcc  
export CXX=g++
```

## ubuntu
```shell
apt install libssl-dev
apt install libglu1-mesa-dev freeglut3-dev mesa-common-dev
apt install libmpv-dev
apt install libsdl2-dev
```

## macos
```shell
brew install mpv
brew install sdl2
```

## android
```shell

rustup default 1.67.1
cargo install cargo-sdl-apk --locked
rustup target add aarch64-linux-android armv7-linux-androideabi i686-linux-android
# https://julhe.github.io/posts/building-an-android-app-with-rust-and-sdl2/

git clone -b release-2.26.x https://github.com/libsdl-org/SDL.git

export ANDROID_HOME=/root/Android/Sdk
export ANDROID_NDK_HOME=/root/Android/Sdk/ndk/21.4.7075529
export ANDROID_NDK_ROOT=/root/Android/Sdk/ndk/21.4.7075529
export PATH=JAVA_HOME/bin:$PATH
export SDL=/root/git/SDL
export PATH=/root/Android/Sdk/ndk/21.4.7075529/toolchains/aarch64-linux-android-4.9/prebuilt/linux-x86_64/bin:$PATH

# tested on linux
cargo sdl-apk build --release --example android_borealis
```

vim ~/.cargo/config.toml
```toml
[target.aarch64-linux-android]
ar = "/root/Android/Sdk/ndk/21.4.7075529/toolchains/llvm/prebuilt/linux-x86_64/bin/aarch64-linux-android-ar"
linker ="/root/Android/Sdk/ndk/21.4.7075529/toolchains/llvm/prebuilt/linux-x86_64/bin/aarch64-linux-android26-clang"
```

```shell
export PATH=/root/Android/Sdk/ndk/21.4.7075529/toolchains/llvm/prebuilt/linux-x86_64/bin/:$PATH
export CC_aarch64_linux_android=/root/Android/Sdk/ndk/21.4.7075529/toolchains/llvm/prebuilt/linux-x86_64/bin/aarch64-linux-android26-clang
export AR_aarch64_linux_android=/root/Android/Sdk/ndk/21.4.7075529/toolchains/llvm/prebuilt/linux-x86_64/bin/aarch64-linux-android-ar
```