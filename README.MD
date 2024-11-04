<p align="center">
  <a href="https://rxdb.info/">
    <img src="./docs/logo.svg" alt="JavaScript Database" />
  </a>
  <br />
  <br />
  <h3 align="center">A secure light-weight and dependency free database wrapper for the web.</h3>
</p>


<p align="center">
    <a href="https://github.com/trust0-project/RIDB/releases"><img src="https://img.shields.io/github/v/release/pubkey/rxdb?color=%23ff00a0&include_prereleases&label=version&sort=semver&style=flat-square"></a>
    &nbsp;
    <a href="https://rxdb.info/tutorials/typescript.html"><img src="https://img.shields.io/npm/types/rxdb?style=flat-square"></a>
    &nbsp;
    <a href="https://raw.githubusercontent.com/trust0-project/RIDB/refs/heads/main/LICENSE"><img src="https://img.shields.io/github/license/pubkey/rxdb?style=flat-square"></a>
    &nbsp;
    <a href="https://www.npmjs.com/package/@trust0/ridb"><img src="https://img.shields.io/npm/dm/@trust0/ridb?color=c63a3b&style=flat-square"></a>   
</p>

# Documentation

## Package Description
RIDB secure database wrapper for the web with multiple storage engines, written in rust.
The project started after years of experience working with web projects in both browser and nodejs platforms, the project was born with some rules / objectives:
1. Strong types + proper validation
2. Declarative schemas & documents
3. Configurable storages, inMemory, monogoDB, sqlite, indexdb
4. Secure encryption
5. Work seamlessly in browsers or nodejs applications.

## Supported features for InMemory Storage
The inMemory storage is used by default and is currently supporting the following features:
1. Schemas: Creation of declararive schemas with required fields
2. Schemas: Implement validation across all the flows extracting properties and required fields when needed
3. Schemas: Manage Primary keys
4. Internal Storage: write operation, create, update, fetch one, remove, find and count
5. Internal Storage: Rust inMemory implementation
6. Database default InMemory plugged in
7. Plugin engine
8. Encryption


## Build & Testing

### How to build this project
Build requirements:
* Bash
* Have Rust ([cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)) and [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)) installed.
* Node JS Version (20/LTS Recommended)

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
