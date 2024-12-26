<p align="center">
  <img src="https://cdn.jsdelivr.net/gh/trust0-project/ridb@latest/docs/logo.svg" alt="JavaScript Database" />
  <br />
  <br />
  <h3 align="center">A secure light-weight and dependency free database wrapper for the web.</h3>
</p>


<p align="center">
    <a href="https://github.com/trust0-project/RIDB/releases"><img src="https://img.shields.io/github/v/release/trust0-project/ridb?color=%23ff00a0&include_prereleases&label=version&sort=semver&style=flat-square"></a>
    &nbsp;
    <a href="#"><img src="https://img.shields.io/npm/types/@trust0/ridb?style=flat-square"></a>
    &nbsp;
    <a href="https://raw.githubusercontent.com/trust0-project/RIDB/refs/heads/main/LICENSE"><img src="https://img.shields.io/github/license/trust0-project/ridb?style=flat-square"></a>
    &nbsp;
    <a href="https://www.npmjs.com/package/@trust0/ridb"><img src="https://img.shields.io/npm/dm/@trust0/ridb?color=c63a3b&style=flat-square"></a>   
</p>

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
| **IndexDB Storage**           | Robust type safe replacement for Dexie          

## Documentation
For detailed API documentation, please visit the [RIDB Documentation](docs/README.md).

## Install
In order to install simply run the following command
npm:
``` 
npm i @trust0/ridb --save
```

yarn:

``` 
yarn add @trust0/ridb
```

## Usage
Creating your own database is pretty straight forward.

```javascript
import {
    RIDB,
    SchemaFieldType
} from '@trust0/ridb';

(async () => {
    const db =  new RIDB({
        demo: {
            version: 0,
            primaryKey: 'id',
            type: SchemaFieldType.object,
            properties: {
                id: {
                    type: SchemaFieldType.string,
                    maxLength: 60
                }
            }
        }
    });
    console.log("Starting the database");
    await db.start({dbName: "demo"});
    console.log("Ok :)");
})()
```

Use with custom storage (IndexDB and InMemory)

```javascript
import {
    RIDB,
    SchemaFieldType,
    StorageType
} from '@trust0/ridb';

(async () => {
    const db =  new RIDB({
        demo: {
            version: 0,
            primaryKey: 'id',
            type: SchemaFieldType.object,
            properties: {
                id: {
                    type: SchemaFieldType.string,
                    maxLength: 60
                }
            }
        }
    });
    console.log("Starting the database");
    await db.start({dbName: "demo", storage: StorageType.IndexDB //or StorageType.InMemory});
    console.log("Ok :)");
})()
```

## Specification

### Storage
A valid storage must extend [BaseStorage class](https://github.com/trust0-project/RIDB/blob/main/docs/namespaces/RIDBTypes/classes/BaseStorage.md)
here's some example:

```typescript
export class InMemory<T extends SchemaTypeRecord>  extends BaseStorage<T> {

    static create<SchemasCreate extends SchemaTypeRecord>(
        name: string,
        schemas: SchemasCreate,
        options: any
    ): Promise<
        BaseStorage<
            SchemasCreate
        >
    > {
        throw new Error("Method not implemented.");
    }

    constructor(name: string, schemas: T, options: any) {
        super(name, schemas, options);
    }

    findDocumentById(collectionName: keyof T, id: string): Promise<Doc<T[keyof T]> | null> {
        throw new Error("Method not implemented.");
    }
    find(collectionName: keyof T, query: QueryType<T[keyof T]>): Promise<Doc<T[keyof T]>[]> {
        throw new Error("Method not implemented.");
    }
    write(op: Operation<T[keyof T]>): Promise<Doc<T[keyof T]>> {
        throw new Error("Method not implemented.");
    }
    count(collectionName: keyof T, query: QueryType<T[keyof T]>): Promise<number> {
        throw new Error("Method not implemented.");
    }
    start(): Promise<void> {
        throw new Error("Method not implemented.");
    }
    close(): Promise<void> {
        throw new Error("Method not implemented.");
    }

}
```

### Plugins
Plugins extend the functionality of the database by hooking into the database lifecycle.

```typescript
/**
 * A simple plugin that overrides the docCreateHook and docRecoverHook methods.
 */
class MySimplePlugin extends BasePlugin {
    constructor() {
        super();
        this.docCreateHook = (
            schema,
            migration,
            docs
        ) => docs;
        this.docRecoverHook = (
            schema,
            migration,
            docs
        ) => docs;
    }
}
```