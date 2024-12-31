<p align="center">
  <img src="https://cdn.jsdelivr.net/gh/trust0-project/ridb@latest/docs/logo.svg" alt="JavaScript Database" />
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

### Description
This package is a leveldb wrapper for the ridb database.

## Installation & usage (typescript)
```bash
npm install @trust0/ridb @trust0/ridb-level -S 
# or yarn add @trust0/ridb @trust0/ridb-level
```

## Usage

The following example demonstrates how to create a database with a leveldb storage engine and a simple schema with ID primary key as string.

```typescript
import { RIDB, SchemaFieldType, Doc } from '@trust0/ridb';
import { LevelDB } from '@trust0/ridb-level';
const db = new RIDB(
    {
        dbName: "test",
        schemas: {
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
        } as const
    }
)
await db.start({
    storageType: LevelDB,
    password: "test"
});
```

