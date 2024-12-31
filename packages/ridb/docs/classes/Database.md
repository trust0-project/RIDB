[**@trust0/ridb**](../README.md)

***

[@trust0/ridb](../README.md) / Database

# Class: Database\<T\>

Represents a database containing collections of documents.
RIDB extends from this class and is used to expose collections.

So if you specify:
```typescript
const db = new RIDB(
    {
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

The collection will be available as `db.collections.demo` and all the methods for the collection (find, count, findById, update, create, delete) will be available.

## Type Parameters

• **T** *extends* [`SchemaTypeRecord`](../type-aliases/SchemaTypeRecord.md)

A record of schema types.

## Constructors

### new Database()

> **new Database**\<`T`\>(): [`Database`](Database.md)\<`T`\>

#### Returns

[`Database`](Database.md)\<`T`\>

## Properties

### collections

> `readonly` **collections**: \{ \[name in string \| number \| symbol\]: Collection\<Schema\<T\[name\]\>\> \}

The collections in the database.

This is a read-only property where the key is the name of the collection and the value is a `Collection` instance.

#### Defined in

ridb-wasm/pkg/ridb\_wasm.d.ts:397

***

### started

> `readonly` **started**: `boolean`

#### Defined in

ridb-wasm/pkg/ridb\_wasm.d.ts:401

## Methods

### close()

> **close**(): `Promise`\<`void`\>

Closes the database.

#### Returns

`Promise`\<`void`\>

A promise that resolves when the database is closed.

#### Defined in

ridb-wasm/pkg/ridb\_wasm.d.ts:415

***

### start()

> **start**(): `Promise`\<`void`\>

Starts the database.

#### Returns

`Promise`\<`void`\>

A promise that resolves when the database is started.

#### Defined in

ridb-wasm/pkg/ridb\_wasm.d.ts:408

***

### create()

> `static` **create**\<`TS`\>(`db_name`, `schemas`, `migrations`, `plugins`, `options`, `password`?, `storage`?): `Promise`\<[`Database`](Database.md)\<`TS`\>\>

Creates a new `Database` instance with the provided schemas and storage module.

#### Type Parameters

• **TS** *extends* [`SchemaTypeRecord`](../type-aliases/SchemaTypeRecord.md)

A record of schema types.

#### Parameters

##### db\_name

`string`

##### schemas

`TS`

The schemas to use for the collections.

##### migrations

[`MigrationPathsForSchemas`](../type-aliases/MigrationPathsForSchemas.md)\<`TS`\> | [`MigrationPathsForSchema`](../type-aliases/MigrationPathsForSchema.md)\<`TS`\[`string`\]\>

##### plugins

*typeof* [`BasePlugin`](BasePlugin.md)[]

##### options

[`RIDBModule`](../type-aliases/RIDBModule.md)

##### password?

`string`

##### storage?

[`BaseStorage`](BaseStorage.md)\<`TS`\>

#### Returns

`Promise`\<[`Database`](Database.md)\<`TS`\>\>

A promise that resolves to the created `Database` instance.

#### Defined in

ridb-wasm/pkg/ridb\_wasm.d.ts:382
