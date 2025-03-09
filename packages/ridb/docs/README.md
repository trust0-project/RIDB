**@trust0/ridb**

***

# @trust0/ridb

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
<h1>Introduction</h1>

### Usage
```typescript
const db = new RIDB(
    {
        dbName: "demo",
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
```

### Using with SharedWorker

```typescript
const db = new RIDB({
    dbName: "demo",
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
        } as const
    },
    worker: true
)
```

### Using with encryption plugin
You can also optionally specify storageType with a compatible storage of your choice and an optional password to enable encryption plugin
```typescript
await db.start({
    password: "my-password"
})
```

A compatible storage should be a class implementing [StorageInternal<SchemaType> ](_media/StorageInternal.md) and its methods.

### Using with migration plugin
The migration plugin will automatically migrate your documents for you as you upgrade and change your schemas over the time. 

```typescript
const db = new RIDB(
    {
        dbName: "demo",
        schemas: {
            demo: {
                version: 1,
                primaryKey: 'id',
                type: SchemaFieldType.object,
                required:['id', 'age'],
                properties: {
                    id: {
                        type: SchemaFieldType.string,
                        maxLength: 60
                    },
                    age: {
                        type: SchemaFieldType.number,
                    }
                }
            }
        } as const,
        migrations: {
            demo: {
                1: function (doc) {
                    return doc
                }
            }
        }
    }
)

await db.start()
```
# SDK Rerefence

## Enumerations

- [OpType](enumerations/OpType.md)
- [StorageType](enumerations/StorageType.md)

## Classes

- [BasePlugin](classes/BasePlugin.md)
- [BaseStorage](classes/BaseStorage.md)
- [Collection](classes/Collection.md)
- [CoreStorage](classes/CoreStorage.md)
- [Database](classes/Database.md)
- [IndexDB](classes/IndexDB.md)
- [InMemory](classes/InMemory.md)
- [Property](classes/Property.md)
- [Query](classes/Query.md)
- [RIDB](classes/RIDB.md)
- [RIDBError](classes/RIDBError.md)
- [Schema](classes/Schema.md)
- [StorageInternal](classes/StorageInternal.md)

## Type Aliases

- [AnyVersionGreaterThan1](type-aliases/AnyVersionGreaterThan1.md)
- [BasePluginOptions](type-aliases/BasePluginOptions.md)
- [BaseStorageOptions](type-aliases/BaseStorageOptions.md)
- [CreateStorage](type-aliases/CreateStorage.md)
- [Doc](type-aliases/Doc.md)
- [EnumerateFrom1To](type-aliases/EnumerateFrom1To.md)
- [EnumerateUpTo](type-aliases/EnumerateUpTo.md)
- [ExtractType](type-aliases/ExtractType.md)
- [Hook](type-aliases/Hook.md)
- [InOperator](type-aliases/InOperator.md)
- [InternalsRecord](type-aliases/InternalsRecord.md)
- [IsVersionGreaterThan0](type-aliases/IsVersionGreaterThan0.md)
- [LogicalOperators](type-aliases/LogicalOperators.md)
- [MigrationFunction](type-aliases/MigrationFunction.md)
- [MigrationPathsForSchema](type-aliases/MigrationPathsForSchema.md)
- [MigrationPathsForSchemas](type-aliases/MigrationPathsForSchemas.md)
- [MigrationsParameter](type-aliases/MigrationsParameter.md)
- [Operation](type-aliases/Operation.md)
- [OperatorOrType](type-aliases/OperatorOrType.md)
- [Operators](type-aliases/Operators.md)
- [QueryOptions](type-aliases/QueryOptions.md)
- [QueryType](type-aliases/QueryType.md)
- [RIDBModule](type-aliases/RIDBModule.md)
- [SchemaType](type-aliases/SchemaType.md)
- [SchemaTypeRecord](type-aliases/SchemaTypeRecord.md)
- [StartOptions](type-aliases/StartOptions.md)
- [StorageClass](type-aliases/StorageClass.md)

## Variables

- [SchemaFieldType](variables/SchemaFieldType.md)
- [WasmInternal](variables/WasmInternal.md)
