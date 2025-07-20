<p align="center">
  <img src="../../resources/ridb-dark.svg" alt="JavaScript Database" />
  <br />
  <br />
  <h3 align="center">A secure light-weight and dependency free database wrapper for the web.</h3>
</p>
<p align="center">
    <a href="https://github.com/trust0-project/RIDB/releases"><img src="https://img.shields.io/github/v/release/trust0-project/ridb?color=%23ff00a0&include_prereleases&label=version&sort=semver&style=flat-square"></a>
    <a href="#"><img src="https://img.shields.io/npm/types/rxdb?style=flat-square"></a>
    <a href="https://raw.githubusercontent.com/trust0-project/RIDB/refs/heads/main/LICENSE"><img src="https://img.shields.io/github/license/trust0-project/ridb?style=flat-square"></a>
    <a href="https://www.npmjs.com/package/@trust0/ridb"><img src="https://img.shields.io/npm/dm/@trust0/ridb?color=c63a3b&style=flat-square"></a>   
</p>

## CORE Introduction
RIDB secure database wrapper for the web with multiple storage engines, written in rust.
This package serves as wasm wrapper for the [RIDB](https://github.com/trust0-project/RIDB) project, providing a secure and lightweight database solution for the web.

## Documentation
Complete type structure and sdk docs can be found [here](../../docs/README.md)

### Building the source code
Building the source code will compile the rust code into wasm and then bundle the Javascript code using esbuild, the output will be located in ./ts/build.
```bash
sh build.sh
```

### How to test the project
For now, we have enabled the implementation of the whole wasm + javascript integration.
In order to run it, write the following:

```bash
sh test.sh
```

### Security
We take security very seriously and have implemented robust measures to ensure data protection. Below are the specifications for our security features:

| Feature                        | Description                                                                 |
|--------------------------------|-----------------------------------------------------------------------------|
| **Password Hashing**           | We use PBKDF2 (Password-Based Key Derivation Function 2) with HMAC-SHA3-256 for password hashing. This method involves multiple iterations to enhance security against brute-force attacks. [Learn more about PBKDF2](https://tools.ietf.org/html/rfc8018#section-5.2) |
| **Encryption**                 | Data is encrypted using AES-256-GCM (Advanced Encryption Standard with Galois/Counter Mode), which provides both confidentiality and integrity. [Learn more about AES-GCM](https://nvlpubs.nist.gov/nistpubs/Legacy/SP/nistspecialpublication800-38d.pdf) |
| **Integrity**                  | We ensure data integrity by hashing data with SHA3-512 and comparing it with the stored hash to detect any tampering. [Learn more about SHA-3](https://nvlpubs.nist.gov/nistpubs/FIPS/NIST.FIPS.202.pdf) |

### Supported features
By default RIDB is bundled with a default InMemory storage with support for write, create, update, fetch one, remove, find and count operations.

| Feature                        | Description                                                                 |
|--------------------------------|-----------------------------------------------------------------------------|
| **Schemas**                    | Creation of declarative schemas with required fields, default and encrypted fields                        |
| **Validation**                 | Implement validation across all the flows extracting properties and required fields when needed |
| **Primary Key Management**     | Primary key and index management                                                    |
| **Plugin Engine**              | Extend the functionality of your Database implementation with wasm or Javascript plugins                                           |
| **Data Encryption Plugin**     | Secure data with encryption plugins                                         |
| **Migration Plugin**           | Support for data migrations                                                 |
| **Integrity Plugin**           | Support for data has not been tampered with                                              |
| **IndexDB Storage**           | Robust type safe replacement for Dexie                                            |

