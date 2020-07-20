# Build WebAssembly from Rust and run it on Deno

This is a project companion to the blog post [How to write WebAssembly in Rust and use it in Deno](https://startfunction.com/rust-webassembly-deno/)

## Install

Install the tool to compile Rust into WebAssembly with:

```
curl https://raw.githubusercontent.com/second-state/ssvmup/master/installer/init.sh -sSf | sh
```

## Build

Build the Rust library into WASM with:

```
ssvmup build --target deno
```

## Run in Deno

To run in Deno, use:

```
deno run --allow-read mod.ts
```

-----

Made by [StartFunction](https://startfunction.com)