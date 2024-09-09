# borealis-rs
rust reimplement of https://github.com/natinusala/borealis

```shell
cargo run --example window_borealis
```

## mingw64

```shell
pacman -S --needed base-devel mingw-w64-x86_64-toolchain
pacman -S mingw-w64-x86_64-SDL2
```

```shell
export CC=gcc  
export CXX=g++
```

## android
```shell
cargo install cargo-sdl-apk
rustup target add aarch64-linux-android

export ANDROID_HOME=
export ANDROID_NDK_ROOT=
export PATH=JAVA_HOME/bin
export SDL=

# tested on linux
cargo sdl-apk run --example android_borealis
```