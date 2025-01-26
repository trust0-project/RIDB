[**@trust0/ridb**](../README.md)

***

[@trust0/ridb](../README.md) / Database

# Class: Database\<T\>

Defined in: ridb-core/pkg/ridb\_core.d.ts:131

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

Defined in: ridb-core/pkg/ridb\_core.d.ts:159

The collections in the database.

This is a read-only property where the key is the name of the collection and the value is a `Collection` instance.

***

### started

> `readonly` **started**: `boolean`

Defined in: ridb-core/pkg/ridb\_core.d.ts:163

## Methods

### close()

> **close**(): `Promise`\<`void`\>

Defined in: ridb-core/pkg/ridb\_core.d.ts:177

Closes the database.

#### Returns

`Promise`\<`void`\>

A promise that resolves when the database is closed.

***

### start()

> **start**(): `Promise`\<`void`\>

Defined in: ridb-core/pkg/ridb\_core.d.ts:170

Starts the database.

#### Returns

`Promise`\<`void`\>

A promise that resolves when the database is started.

***

### create()

> `static` **create**\<`TS`\>(`db_name`, `schemas`, `migrations`, `plugins`, `options`, `password`?, `storage`?): `Promise`\<[`Database`](Database.md)\<`TS`\>\>

Defined in: ridb-core/pkg/ridb\_core.d.ts:144

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
