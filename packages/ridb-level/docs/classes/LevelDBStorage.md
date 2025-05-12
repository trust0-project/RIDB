[**@trust0/ridb-level**](../README.md)

***

[@trust0/ridb-level](../README.md) / LevelDBStorage

# Class: LevelDBStorage\<T\>

Defined in: [index.ts:46](https://github.com/trust0-project/RIDB/blob/96bdd9e989f3b9d3bb9f1e9e2333148433a17232/packages/ridb-level/src/index.ts#L46)

LevelDB storage implementation class

## Extends

- `BaseStorage`\<`T`\>

## Type Parameters

### T

`T` *extends* `SchemaTypeRecord`

## Constructors

### Constructor

> **new LevelDBStorage**\<`T`\>(`level`, `name`, `schemas`, `options`): `LevelDBStorage`\<`T`\>

Defined in: [index.ts:47](https://github.com/trust0-project/RIDB/blob/96bdd9e989f3b9d3bb9f1e9e2333148433a17232/packages/ridb-level/src/index.ts#L47)

#### Parameters

##### level

[`Level`](../type-aliases/Level.md)

##### name

`string`

##### schemas

`T`

##### options

`any`

#### Returns

`LevelDBStorage`\<`T`\>

#### Overrides

`BaseStorageType<T>.constructor`

## Properties

### core

> `readonly` **core**: `CoreStorage`

Defined in: ../../ridb-core/build/ridb\_core.d.ts:199

#### Inherited from

`BaseStorageType.core`

***

### dbName

> `readonly` **dbName**: `string`

Defined in: ../../ridb-core/build/ridb\_core.d.ts:196

#### Inherited from

`BaseStorageType.dbName`

***

### level

> **level**: [`Level`](../type-aliases/Level.md)

Defined in: [index.ts:47](https://github.com/trust0-project/RIDB/blob/96bdd9e989f3b9d3bb9f1e9e2333148433a17232/packages/ridb-level/src/index.ts#L47)

***

### options

> `readonly` **options**: `BaseStorageOptions`

Defined in: ../../ridb-core/build/ridb\_core.d.ts:198

#### Inherited from

`BaseStorageType.options`

***

### schemas

> `readonly` **schemas**: `Record`\<keyof `T`, `Schema`\<`T`\[keyof `T`\]\>\>

Defined in: ../../ridb-core/build/ridb\_core.d.ts:197

#### Inherited from

`BaseStorageType.schemas`

## Methods

### addIndexSchemas()

> **addIndexSchemas**(): `null`

Defined in: ../../ridb-core/build/ridb\_core.d.ts:209

#### Returns

`null`

#### Inherited from

`BaseStorageType.addIndexSchemas`

***

### close()

> **close**(): `Promise`\<`void`\>

Defined in: [index.ts:76](https://github.com/trust0-project/RIDB/blob/96bdd9e989f3b9d3bb9f1e9e2333148433a17232/packages/ridb-level/src/index.ts#L76)

Close the database

#### Returns

`Promise`\<`void`\>

#### Overrides

`BaseStorageType.close`

***

### count()

> **count**(`collectionName`, `query`, `options?`): `Promise`\<`number`\>

Defined in: [index.ts:147](https://github.com/trust0-project/RIDB/blob/96bdd9e989f3b9d3bb9f1e9e2333148433a17232/packages/ridb-level/src/index.ts#L147)

Count documents matching a query (supports offset & limit)

#### Parameters

##### collectionName

keyof `T`

##### query

`QueryType`\<`T`\[keyof `T`\]\>

##### options?

`QueryOptions`

#### Returns

`Promise`\<`number`\>

#### Overrides

`BaseStorageType.count`

***

### find()

> **find**(`collectionName`, `query`, `options?`): `Promise`\<`Doc`\<`T`\[keyof `T`\]\>[]\>

Defined in: [index.ts:196](https://github.com/trust0-project/RIDB/blob/96bdd9e989f3b9d3bb9f1e9e2333148433a17232/packages/ridb-level/src/index.ts#L196)

Find documents matching a query with pagination

#### Parameters

##### collectionName

keyof `T`

##### query

`QueryType`\<`T`\[keyof `T`\]\>

##### options?

`QueryOptions`

#### Returns

`Promise`\<`Doc`\<`T`\[keyof `T`\]\>[]\>

#### Overrides

`BaseStorageType.find`

***

### findDocumentById()

> **findDocumentById**(`collectionName`, `id`): `Promise`\<`null` \| `Doc`\<`T`\[keyof `T`\]\>\>

Defined in: [index.ts:81](https://github.com/trust0-project/RIDB/blob/96bdd9e989f3b9d3bb9f1e9e2333148433a17232/packages/ridb-level/src/index.ts#L81)

Find a document by its ID

#### Parameters

##### collectionName

keyof `T`

##### id

`string`

#### Returns

`Promise`\<`null` \| `Doc`\<`T`\[keyof `T`\]\>\>

#### Overrides

`BaseStorageType.findDocumentById`

***

### getOption()

> **getOption**(`name`): `undefined` \| `string` \| `number` \| `boolean`

Defined in: ../../ridb-core/build/ridb\_core.d.ts:206

#### Parameters

##### name

`string`

#### Returns

`undefined` \| `string` \| `number` \| `boolean`

#### Inherited from

`BaseStorageType.getOption`

***

### getSchema()

> **getSchema**(`name`): `Schema`\<`any`\>

Defined in: ../../ridb-core/build/ridb\_core.d.ts:207

#### Parameters

##### name

`string`

#### Returns

`Schema`\<`any`\>

#### Inherited from

`BaseStorageType.getSchema`

***

### start()

> **start**(): `Promise`\<`void`\>

Defined in: [index.ts:71](https://github.com/trust0-project/RIDB/blob/96bdd9e989f3b9d3bb9f1e9e2333148433a17232/packages/ridb-level/src/index.ts#L71)

Start the database

#### Returns

`Promise`\<`void`\>

#### Overrides

`BaseStorageType.start`

***

### write()

> **write**(`op`): `Promise`\<`Doc`\<`T`\[keyof `T`\]\>\>

Defined in: [index.ts:103](https://github.com/trust0-project/RIDB/blob/96bdd9e989f3b9d3bb9f1e9e2333148433a17232/packages/ridb-level/src/index.ts#L103)

Write an operation (insert, update, delete)

#### Parameters

##### op

`Operation`\<`T`\[keyof `T`\]\>

#### Returns

`Promise`\<`Doc`\<`T`\[keyof `T`\]\>\>

#### Overrides

`BaseStorageType.write`

***

### create()

> `static` **create**\<`SchemasCreate`\>(`name`, `schemas`, `options?`): `Promise`\<`LevelDBStorage`\<`SchemasCreate`\>\>

Defined in: [index.ts:59](https://github.com/trust0-project/RIDB/blob/96bdd9e989f3b9d3bb9f1e9e2333148433a17232/packages/ridb-level/src/index.ts#L59)

Create a new LevelDB storage instance

#### Type Parameters

##### SchemasCreate

`SchemasCreate` *extends* `SchemaTypeRecord`

#### Parameters

##### name

`string`

Database name

##### schemas

`SchemasCreate`

Collection schemas

##### options?

`BaseStorageOptions`

Storage options

#### Returns

`Promise`\<`LevelDBStorage`\<`SchemasCreate`\>\>

A new Instance of LevelDB storage

#### Overrides

`BaseStorageType.create`
