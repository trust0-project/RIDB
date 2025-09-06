[**Documentation**](../../../README.md)

***

[Documentation](../../../README.md) / [@trust0/ridb-core](../README.md) / Database

# Class: Database\<T\>

Defined in: ridb\_core.d.ts:302

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

### T

`T` *extends* [`SchemaTypeRecord`](../type-aliases/SchemaTypeRecord.md)

A record of schema types.

## Constructors

### Constructor

> **new Database**\<`T`\>(): `Database`\<`T`\>

#### Returns

`Database`\<`T`\>

## Properties

### collections

> `readonly` **collections**: \{ \[name in string \| number \| symbol\]: Collection\<Schema\<T\[name\]\>\> \}

Defined in: ridb\_core.d.ts:332

The collections in the database.

This is a read-only property where the key is the name of the collection and the value is a `Collection` instance.

***

### started

> `readonly` **started**: `boolean`

Defined in: ridb\_core.d.ts:336

## Methods

### authenticate()

> **authenticate**(`password`): `Promise`\<`boolean`\>

Defined in: ridb\_core.d.ts:325

#### Parameters

##### password

`string`

#### Returns

`Promise`\<`boolean`\>

***

### close()

> **close**(): `Promise`\<`void`\>

Defined in: ridb\_core.d.ts:350

Closes the database.

#### Returns

`Promise`\<`void`\>

A promise that resolves when the database is closed.

***

### start()

> **start**(): `Promise`\<`void`\>

Defined in: ridb\_core.d.ts:343

Starts the database.

#### Returns

`Promise`\<`void`\>

A promise that resolves when the database is started.

***

### create()

> `static` **create**\<`TS`\>(`db_name`, `schemas`, `migrations`, `plugins`, `options`, `password?`, `storage?`): `Promise`\<`Database`\<`TS`\>\>

Defined in: ridb\_core.d.ts:315

Creates a new `Database` instance with the provided schemas and storage module.

#### Type Parameters

##### TS

`TS` *extends* [`SchemaTypeRecord`](../type-aliases/SchemaTypeRecord.md)

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

`Promise`\<`Database`\<`TS`\>\>

A promise that resolves to the created `Database` instance.
