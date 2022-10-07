# Ingen SDK

Ingen (/ingen/, 隐元) is a cloud VM emulator based on WASM. It provides the environment for emulate the lifecycle of the VM and VM agents (processes running inside of VMs), and uses WASM runtime to run the emulated or real workloads, as long as the workload can be compiled into a WASM-WASI program.

To help our users to use Ingen, we provides 2 SDKs for integrations: 
- WASM SDK to help writing WASM programs.
- Ingen gRPC interfaces to help communicate with Ingen service.

# WASM SDK

Although [WASM-WASI has wide range of language support](https://www.fermyon.com/wasm-languages/webassembly-language-support), however there are few functionalities missing, for example - certain socket APIs, such as raw socket for ICMP ping, TCP connect / bind, setting socket options and etc. This SDK provides low level capability to help everyone writing programs.

## How to use?

For most of the users, we don't need to use this SDK directly, as we extended the WASI APIs and have also ported a list of widely used libs (below), so we can directly update the .toml file to points to these ported libs. Then the program can be compiled into WASM with very minor code changes and run in our environment.

- mio
- tokio
- h2
- hyper
- reqwest
- mysql_async

For certain things, we will have to use the SDK directly, as they cannot be done by using WASI capabilities, such as ICMP ping, which uses raw socket. In this case, we will have to call the SDK directly.

To include the WASM SDK in your project, please add this in your project:

```yaml
ingen-wasm-sdk = { git = "https://github.com/futurewei-cloud/ingen-sdk", branch = "main" }
```

# Ingen gRPC interfaces

Although Ingen is provided as a lib, but sometimes we might like to host it from other languages, such as golang, so instead of directly linking it, we could launch Ingen as a child process and use gRPC to communicate with it. In this case, Ingen will act as a gRPC server, and our hosting service can connect to it to send commands. To help this case, we provided all `.proto` files, so we can use them to generate the client in any language.

To include the `.proto` files in your repo, we can add this repo as submodule and use the proto files directly.

# Development

## How to update wit?

### One-time initialization

1. (Windows only) Install [Git for Windows](https://gitforwindows.org/). It will give us `sh`, which could be used for running our build scripts.
2. (Windows only) Add the `sh` command path into the PATH environment variable. The binary is by default located here: `C:\Program Files\Git\bin\sh.exe`.
3. Run `just init` to get `wit-bindgen` CLI installed.

### Update wit

1. Update wit definiitons in `./wit` folder.
2. Run `just gen-wasm` to update the bindings in `./wasm_sdk` folder.
3. Check how the new bindings looks like and run `just build` to make sure things builds.
4. Submit PR for review and merge.
