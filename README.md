# SputnikVM in Browser

This compiles the `no_std` version of SputnikVM and run it in browser,
using Rust's `wasm32-unknown-emscripten` target.

## Get Started

Installing a custom target for Rust is not yet an easy task so it is
recommended to use
[rustup](https://www.rust-lang.org/en-US/install.html). The `no_std`
version needs Rust nightly due to its use of `alloc` library.

```
rustup install nightly
rustup default nightly
```

After that, install the `wasm32-unknown-emscripten` target.

```
rustup target add wasm32-unknown-emscripten
```

You will need to install emscripten next. Note that the build can take
quite a long time.

```
wget https://s3.amazonaws.com/mozilla-games/emscripten/releases/emsdk-portable.tar.gz
tar -xvf emsdk-portable.tar.gz
cd emsdk-portable
./emsdk update
./emsdk install sdk-incoming-64bit
source ./emsdk-env.sh
```

After that, you can compile this project.

```
npm run compile
npm run serve
```

Go to `http://127.0.0.1:8080` and open the web console. The example
will execute a same EVM program using SputnikVM, and if no error is
reported, you will see a `0` printed.
