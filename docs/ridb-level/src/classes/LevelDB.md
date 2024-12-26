[**trust0**](../../../README.md)

***

[trust0](../../../README.md) / [ridb-level/src](../README.md) / LevelDB

# Class: LevelDB\<T\>

## Extends

- `BaseStorage`\<`T`\>

## Type Parameters

• **T** *extends* `SchemaTypeRecord`

## Constructors

### new LevelDB()

> **new LevelDB**\<`T`\>(`Level`, `name`, `schemas`, `options`): [`LevelDB`](LevelDB.md)\<`T`\>

#### Parameters

##### Level

*typeof* `ClassicLevel`

##### name

`string`

##### schemas

`T`

##### options

`any`

#### Returns

[`LevelDB`](LevelDB.md)\<`T`\>

#### Overrides

`BaseStorage<T>.constructor`

#### Defined in

[packages/ridb-level/src/index.ts:44](https://github.com/elribonazo/RIDB/blob/7a38590bb34c3fed47a6348e8f9a6eb9c1d2b091/packages/ridb-level/src/index.ts#L44)

## Properties

### core

> `readonly` **core**: `CoreStorage`

#### Inherited from

`BaseStorage.core`

#### Defined in

node\_modules/@trust0/ridb-wasm/ridb\_wasm.d.ts:403

***

### db

> `private` **db**: `Level`

#### Defined in

[packages/ridb-level/src/index.ts:34](https://github.com/elribonazo/RIDB/blob/7a38590bb34c3fed47a6348e8f9a6eb9c1d2b091/packages/ridb-level/src/index.ts#L34)

***

### dbName

> `readonly` **dbName**: `string`

#### Inherited from

`BaseStorage.dbName`

#### Defined in

node\_modules/@trust0/ridb-wasm/ridb\_wasm.d.ts:400

***

### options

> `readonly` **options**: `BaseStorageOptions`

#### Inherited from

`BaseStorage.options`

#### Defined in

node\_modules/@trust0/ridb-wasm/ridb\_wasm.d.ts:402

***

### schemas

> `readonly` **schemas**: `Record`\<keyof `T`, `Schema`\<`T`\[keyof `T`\]\>\>

#### Inherited from

`BaseStorage.schemas`

#### Defined in

node\_modules/@trust0/ridb-wasm/ridb\_wasm.d.ts:401

## Methods

### close()

> **close**(): `Promise`\<`void`\>

Close the database

#### Returns

`Promise`\<`void`\>

#### Overrides

`BaseStorage.close`

#### Defined in

[packages/ridb-level/src/index.ts:53](https://github.com/elribonazo/RIDB/blob/7a38590bb34c3fed47a6348e8f9a6eb9c1d2b091/packages/ridb-level/src/index.ts#L53)

***

### count()

> **count**(`collectionName`, `query`): `Promise`\<`number`\>

Count documents matching a query

#### Parameters

##### collectionName

keyof `T`

##### query

`QueryType`\<`T`\[keyof `T`\]\>

#### Returns

`Promise`\<`number`\>

#### Overrides

`BaseStorage.count`

#### Defined in

[packages/ridb-level/src/index.ts:99](https://github.com/elribonazo/RIDB/blob/7a38590bb34c3fed47a6348e8f9a6eb9c1d2b091/packages/ridb-level/src/index.ts#L99)

***

### find()

> **find**(`collectionName`, `query`): `Promise`\<`Doc`\<`T`\[keyof `T`\]\>[]\>

Find documents matching a query

#### Parameters

##### collectionName

keyof `T`

##### query

`QueryType`\<`T`\[keyof `T`\]\>

#### Returns

`Promise`\<`Doc`\<`T`\[keyof `T`\]\>[]\>

#### Overrides

`BaseStorage.find`

#### Defined in

[packages/ridb-level/src/index.ts:119](https://github.com/elribonazo/RIDB/blob/7a38590bb34c3fed47a6348e8f9a6eb9c1d2b091/packages/ridb-level/src/index.ts#L119)

***

### findDocumentById()

> **findDocumentById**(`collectionName`, `id`): `Promise`\<`Doc`\<`T`\[keyof `T`\]\>\>

Find a document by its ID

#### Parameters

##### collectionName

keyof `T`

##### id

`string`

#### Returns

`Promise`\<`Doc`\<`T`\[keyof `T`\]\>\>

#### Overrides

`BaseStorage.findDocumentById`

#### Defined in

[packages/ridb-level/src/index.ts:57](https://github.com/elribonazo/RIDB/blob/7a38590bb34c3fed47a6348e8f9a6eb9c1d2b091/packages/ridb-level/src/index.ts#L57)

***

### getOption()

> **getOption**(`name`): `string` \| `number` \| `boolean`

#### Parameters

##### name

`string`

#### Returns

`string` \| `number` \| `boolean`

#### Inherited from

`BaseStorage.getOption`

#### Defined in

node\_modules/@trust0/ridb-wasm/ridb\_wasm.d.ts:411

***

### getSchema()

> **getSchema**(`name`): `Schema`\<`any`\>

#### Parameters

##### name

`string`

#### Returns

`Schema`\<`any`\>

#### Inherited from

`BaseStorage.getSchema`

#### Defined in

node\_modules/@trust0/ridb-wasm/ridb\_wasm.d.ts:412

***

### matchesQuery()

> **matchesQuery**(`doc`, `query`): `boolean`

#### Parameters

##### doc

`Doc`\<`T`\[keyof `T`\]\>

##### query

`Query`\<`any`\>

#### Returns

`boolean`

#### Defined in

[packages/ridb-level/src/index.ts:138](https://github.com/elribonazo/RIDB/blob/7a38590bb34c3fed47a6348e8f9a6eb9c1d2b091/packages/ridb-level/src/index.ts#L138)

***

### start()

> **start**(): `Promise`\<`void`\>

Start the database

#### Returns

`Promise`\<`void`\>

#### Overrides

`BaseStorage.start`

#### Defined in

[packages/ridb-level/src/index.ts:49](https://github.com/elribonazo/RIDB/blob/7a38590bb34c3fed47a6348e8f9a6eb9c1d2b091/packages/ridb-level/src/index.ts#L49)

***

### write()

> **write**(`op`): `Promise`\<`Doc`\<`T`\[keyof `T`\]\>\>

Write an operation (insert, update, delete)

#### Parameters

##### op

`Operation`\<`T`\[keyof `T`\]\>

#### Returns

`Promise`\<`Doc`\<`T`\[keyof `T`\]\>\>

#### Overrides

`BaseStorage.write`

#### Defined in

[packages/ridb-level/src/index.ts:75](https://github.com/elribonazo/RIDB/blob/7a38590bb34c3fed47a6348e8f9a6eb9c1d2b091/packages/ridb-level/src/index.ts#L75)

***

### create()

> `static` **create**\<`SchemasCreate`\>(`name`, `schemas`, `options`): `Promise`\<`BaseStorage`\<`SchemasCreate`\>\>

#### Type Parameters

• **SchemasCreate** *extends* `SchemaTypeRecord`

#### Parameters

##### name

`string`

##### schemas

`SchemasCreate`

##### options

`any`

#### Returns

`Promise`\<`BaseStorage`\<`SchemasCreate`\>\>

#### Overrides

`BaseStorage.create`

#### Defined in

[packages/ridb-level/src/index.ts:35](https://github.com/elribonazo/RIDB/blob/7a38590bb34c3fed47a6348e8f9a6eb9c1d2b091/packages/ridb-level/src/index.ts#L35)
