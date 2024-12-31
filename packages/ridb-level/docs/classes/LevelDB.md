[**@trust0/ridb-level**](../README.md)

***

[@trust0/ridb-level](../README.md) / LevelDB

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

[ridb-level/src/index.ts:47](https://github.com/elribonazo/RIDB/blob/b28b0b719e467bd7324ebf7597b93fa48be5c6ad/packages/ridb-level/src/index.ts#L47)

## Properties

### core

> `readonly` **core**: `CoreStorage`

#### Inherited from

`BaseStorage.core`

#### Defined in

ridb-wasm/pkg/ridb\_wasm.d.ts:199

***

### db

> `private` **db**: `Level`

#### Defined in

[ridb-level/src/index.ts:37](https://github.com/elribonazo/RIDB/blob/b28b0b719e467bd7324ebf7597b93fa48be5c6ad/packages/ridb-level/src/index.ts#L37)

***

### dbName

> `readonly` **dbName**: `string`

#### Inherited from

`BaseStorage.dbName`

#### Defined in

ridb-wasm/pkg/ridb\_wasm.d.ts:196

***

### options

> `readonly` **options**: `BaseStorageOptions`

#### Inherited from

`BaseStorage.options`

#### Defined in

ridb-wasm/pkg/ridb\_wasm.d.ts:198

***

### schemas

> `readonly` **schemas**: `Record`\<keyof `T`, `Schema`\<`T`\[keyof `T`\]\>\>

#### Inherited from

`BaseStorage.schemas`

#### Defined in

ridb-wasm/pkg/ridb\_wasm.d.ts:197

## Methods

### close()

> **close**(): `Promise`\<`void`\>

Close the database

#### Returns

`Promise`\<`void`\>

#### Overrides

`BaseStorage.close`

#### Defined in

[ridb-level/src/index.ts:56](https://github.com/elribonazo/RIDB/blob/b28b0b719e467bd7324ebf7597b93fa48be5c6ad/packages/ridb-level/src/index.ts#L56)

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

[ridb-level/src/index.ts:124](https://github.com/elribonazo/RIDB/blob/b28b0b719e467bd7324ebf7597b93fa48be5c6ad/packages/ridb-level/src/index.ts#L124)

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

[ridb-level/src/index.ts:144](https://github.com/elribonazo/RIDB/blob/b28b0b719e467bd7324ebf7597b93fa48be5c6ad/packages/ridb-level/src/index.ts#L144)

***

### findDocumentById()

> **findDocumentById**(`collectionName`, `id`): `Promise`\<`undefined` \| `Doc`\<`T`\[keyof `T`\]\>\>

Find a document by its ID

#### Parameters

##### collectionName

keyof `T`

##### id

`string`

#### Returns

`Promise`\<`undefined` \| `Doc`\<`T`\[keyof `T`\]\>\>

#### Overrides

`BaseStorage.findDocumentById`

#### Defined in

[ridb-level/src/index.ts:60](https://github.com/elribonazo/RIDB/blob/b28b0b719e467bd7324ebf7597b93fa48be5c6ad/packages/ridb-level/src/index.ts#L60)

***

### getOption()

> **getOption**(`name`): `undefined` \| `string` \| `number` \| `boolean`

#### Parameters

##### name

`string`

#### Returns

`undefined` \| `string` \| `number` \| `boolean`

#### Inherited from

`BaseStorage.getOption`

#### Defined in

ridb-wasm/pkg/ridb\_wasm.d.ts:207

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

ridb-wasm/pkg/ridb\_wasm.d.ts:208

***

### start()

> **start**(): `Promise`\<`void`\>

Start the database

#### Returns

`Promise`\<`void`\>

#### Overrides

`BaseStorage.start`

#### Defined in

[ridb-level/src/index.ts:52](https://github.com/elribonazo/RIDB/blob/b28b0b719e467bd7324ebf7597b93fa48be5c6ad/packages/ridb-level/src/index.ts#L52)

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

[ridb-level/src/index.ts:81](https://github.com/elribonazo/RIDB/blob/b28b0b719e467bd7324ebf7597b93fa48be5c6ad/packages/ridb-level/src/index.ts#L81)

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

[ridb-level/src/index.ts:38](https://github.com/elribonazo/RIDB/blob/b28b0b719e467bd7324ebf7597b93fa48be5c6ad/packages/ridb-level/src/index.ts#L38)
