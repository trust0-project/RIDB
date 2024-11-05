<p align="center">
  <img src="https://cdn.jsdelivr.net/gh/trust0-project/ridb@0.4.3/docs/logo.svg" alt="JavaScript Database" />
  <br />
  <br />
  <h3 align="center">A secure light-weight and dependency free database wrapper for the web.</h3>
</p>

<p align="center">
    <a href="https://github.com/trust0-project/RIDB/releases"><img src="https://img.shields.io/github/v/release/trust0-project/ridb?color=%23ff00a0&include_prereleases&label=version&sort=semver&style=flat-square"></a>
    &nbsp;
    <a href="#"><img src="https://img.shields.io/npm/types/rxdb?style=flat-square"></a>
    &nbsp;
    <a href="https://raw.githubusercontent.com/trust0-project/RIDB/refs/heads/main/LICENSE"><img src="https://img.shields.io/github/license/trust0-project/ridb?style=flat-square"></a>
    &nbsp;
    <a href="https://www.npmjs.com/package/@trust0/ridb"><img src="https://img.shields.io/npm/dm/@trust0/ridb?color=c63a3b&style=flat-square"></a>   
</p>

## Introduction
RIDB secure database wrapper for the web with multiple storage engines, written in rust.
The project started after years of experience working with web projects in both browser and nodejs platforms, the project was born with some rules / objectives:
1. **Strong types + proper validation**
2. **Declarative schemas & documents**
3. **Configurable storages, inMemory, monogoDB, sqlite, indexdb**
4. **Secure encryption**
5. **Work seamlessly in browsers or nodejs applications.**

### Supported features
By default RIDB is bundled with a default InMemory storage with support for write, create, update, fetch one, remove, find and count operations.

Features:
1. Schemas: Creation of declararive schemas with required fields
2. Schemas: Implement validation across all the flows extracting properties and required fields when needed
3. Schemas: Manage Primary keys
4. Schemas: Manage required and encrypted fields
5. Plugin engine
6. Data Encryption plugin
7. Migration Plugin

## Build & Testing
RIDB is a project mainly developed in rust and exported to wasm, the project combines the execution of rust code with the plugins and storage engines defined by the user in JS code or other wasm packages.
Building the project is pretty straight forward.

### Requirements
* Bash
* Have Rust ([cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)) and [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)) installed.
* Node JS Version (20/LTS Recommended)

### Building the source code
Building the source code will compile the rust code into wasm and then bundle the Javascript code using esbuild, the output will be located in ./ts/build.
```bash
cd ts 
npm i
npm run build
```

### How to test the project
For now, we have enabled the implementation of the whole wasm + javascript integration.
In order to run it, write the following:

```bash
cd ts 
npm i
npm run test
```

## Contributing
I'm very welcome to contributions from anyone, feel free to always open an Issue or create a Pull request. 
I'll try to be as transparent as I can around the things that are needed for the project.

Main priorities:
* Improve documentation
* Adding more examples
* Testing and code coverage