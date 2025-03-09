[**@trust0/ridb-level**](../README.md)

***

[@trust0/ridb-level](../README.md) / LevelDB

# Class: LevelDB\<T\>

Defined in: [ridb-level/src/index.ts:41](https://github.com/trust0-project/RIDB/blob/6136f852d442fa87a06ae11f9e9437473c5a16f1/packages/ridb-level/src/index.ts#L41)

## Extends

- `BaseStorage`\<`T`\>

## Type Parameters

• **T** *extends* `SchemaTypeRecord`

## Constructors

### new LevelDB()

> **new LevelDB**\<`T`\>(`db`, `name`, `schemas`, `options`): [`LevelDB`](LevelDB.md)\<`T`\>

Defined in: [ridb-level/src/index.ts:53](https://github.com/trust0-project/RIDB/blob/6136f852d442fa87a06ae11f9e9437473c5a16f1/packages/ridb-level/src/index.ts#L53)

#### Parameters

##### db

`Level`

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

## Properties

### core

> `readonly` **core**: `CoreStorage`

Defined in: ridb-core/pkg/ridb\_core.d.ts:174

#### Inherited from

`BaseStorage.core`

***

### db

> `private` **db**: `Level`

Defined in: [ridb-level/src/index.ts:53](https://github.com/trust0-project/RIDB/blob/6136f852d442fa87a06ae11f9e9437473c5a16f1/packages/ridb-level/src/index.ts#L53)

***

### dbName

> `readonly` **dbName**: `string`

Defined in: ridb-core/pkg/ridb\_core.d.ts:171

#### Inherited from

`BaseStorage.dbName`

***

### options

> `readonly` **options**: `BaseStorageOptions`

Defined in: ridb-core/pkg/ridb\_core.d.ts:173

#### Inherited from

`BaseStorage.options`

***

### schemas

> `readonly` **schemas**: `Record`\<keyof `T`, `Schema`\<`T`\[keyof `T`\]\>\>

Defined in: ridb-core/pkg/ridb\_core.d.ts:172

#### Inherited from

`BaseStorage.schemas`

## Methods

### addIndexSchemas()

> **addIndexSchemas**(): `null`

Defined in: ridb-core/pkg/ridb\_core.d.ts:184

#### Returns

`null`

#### Inherited from

`BaseStorage.addIndexSchemas`

***

### close()

> **close**(): `Promise`\<`void`\>

Defined in: [ridb-level/src/index.ts:61](https://github.com/trust0-project/RIDB/blob/6136f852d442fa87a06ae11f9e9437473c5a16f1/packages/ridb-level/src/index.ts#L61)

Close the database

#### Returns

`Promise`\<`void`\>

#### Overrides

`BaseStorage.close`

***

### count()

> **count**(`collectionName`, `query`, `options`?): `Promise`\<`number`\>

Defined in: [ridb-level/src/index.ts:129](https://github.com/trust0-project/RIDB/blob/6136f852d442fa87a06ae11f9e9437473c5a16f1/packages/ridb-level/src/index.ts#L129)

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

`BaseStorage.count`

***

### find()

> **find**(`collectionName`, `query`, `options`?): `Promise`\<`Doc`\<`T`\[keyof `T`\]\>[]\>

Defined in: [ridb-level/src/index.ts:177](https://github.com/trust0-project/RIDB/blob/6136f852d442fa87a06ae11f9e9437473c5a16f1/packages/ridb-level/src/index.ts#L177)

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

`BaseStorage.find`

***

### findDocumentById()

> **findDocumentById**(`collectionName`, `id`): `Promise`\<`null` \| `Doc`\<`T`\[keyof `T`\]\>\>

Defined in: [ridb-level/src/index.ts:65](https://github.com/trust0-project/RIDB/blob/6136f852d442fa87a06ae11f9e9437473c5a16f1/packages/ridb-level/src/index.ts#L65)

Find a document by its ID

#### Parameters

##### collectionName

keyof `T`

##### id

`string`

#### Returns

`Promise`\<`null` \| `Doc`\<`T`\[keyof `T`\]\>\>

#### Overrides

`BaseStorage.findDocumentById`

***

### getOption()

> **getOption**(`name`): `undefined` \| `string` \| `number` \| `boolean`

Defined in: ridb-core/pkg/ridb\_core.d.ts:181

#### Parameters

##### name

`string`

#### Returns

`undefined` \| `string` \| `number` \| `boolean`

#### Inherited from

`BaseStorage.getOption`

***

### getSchema()

> **getSchema**(`name`): `Schema`\<`any`\>

Defined in: ridb-core/pkg/ridb\_core.d.ts:182

#### Parameters

##### name

`string`

#### Returns

`Schema`\<`any`\>

#### Inherited from

`BaseStorage.getSchema`

***

### start()

> **start**(): `Promise`\<`void`\>

Defined in: [ridb-level/src/index.ts:57](https://github.com/trust0-project/RIDB/blob/6136f852d442fa87a06ae11f9e9437473c5a16f1/packages/ridb-level/src/index.ts#L57)

Start the database

#### Returns

`Promise`\<`void`\>

#### Overrides

`BaseStorage.start`

***

### write()

> **write**(`op`): `Promise`\<`Doc`\<`T`\[keyof `T`\]\>\>

Defined in: [ridb-level/src/index.ts:86](https://github.com/trust0-project/RIDB/blob/6136f852d442fa87a06ae11f9e9437473c5a16f1/packages/ridb-level/src/index.ts#L86)

Write an operation (insert, update, delete)

#### Parameters

##### op

`Operation`\<`T`\[keyof `T`\]\>

#### Returns

`Promise`\<`Doc`\<`T`\[keyof `T`\]\>\>

#### Overrides

`BaseStorage.write`

***

### create()

> `static` **create**\<`SchemasCreate`\>(`name`, `schemas`, `options`): `Promise`\<[`LevelDB`](LevelDB.md)\<`SchemasCreate`\>\>

Defined in: [ridb-level/src/index.ts:43](https://github.com/trust0-project/RIDB/blob/6136f852d442fa87a06ae11f9e9437473c5a16f1/packages/ridb-level/src/index.ts#L43)

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

`Promise`\<[`LevelDB`](LevelDB.md)\<`SchemasCreate`\>\>

#### Overrides

`BaseStorage.create`
