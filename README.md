# Ingen SDK

[Merak](https://github.com/futurewei-cloud/merak) is a Large-scale Cloud Emulator.

# How to update wit?

## One-time initialization

1. (Windows only) Install [Git for Windows](https://gitforwindows.org/). It will give us `sh`, which could be used for running our build scripts.
2. (Windows only) Add the `sh` command path into the PATH environment variable. The binary is by default located here: `C:\Program Files\Git\bin\sh.exe`.
3. Run `just init` to get `wit-bindgen` CLI installed.

## Update wit

1. Update wit definiitons in `./wit` folder.
2. Run `just gen-wasm` to update the bindings in `./wasm_sdk` folder.
3. Check how the new bindings looks like and run `just build` to make sure things builds.
4. Submit PR for review and merge.
