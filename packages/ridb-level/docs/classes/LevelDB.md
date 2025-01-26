[**@trust0/ridb-level**](../README.md)

***

[@trust0/ridb-level](../README.md) / LevelDB

# Class: LevelDB\<T\>

Defined in: [ridb-level/src/index.ts:37](https://github.com/trust0-project/RIDB/blob/99c65071b9fa26908fb7a847098f8c5c3969f24b/packages/ridb-level/src/index.ts#L37)

## Extends

- `BaseStorage`\<`T`\>

## Type Parameters

• **T** *extends* `SchemaTypeRecord`

## Constructors

### new LevelDB()

> **new LevelDB**\<`T`\>(`Level`, `name`, `schemas`, `options`): [`LevelDB`](LevelDB.md)\<`T`\>

Defined in: [ridb-level/src/index.ts:48](https://github.com/trust0-project/RIDB/blob/99c65071b9fa26908fb7a847098f8c5c3969f24b/packages/ridb-level/src/index.ts#L48)

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

## Properties

### core

> `readonly` **core**: `CoreStorage`

Defined in: ridb-core/pkg/ridb\_core.d.ts:115

#### Inherited from

`BaseStorage.core`

***

### db

> `private` **db**: `Level`

Defined in: [ridb-level/src/index.ts:38](https://github.com/trust0-project/RIDB/blob/99c65071b9fa26908fb7a847098f8c5c3969f24b/packages/ridb-level/src/index.ts#L38)

***

### dbName

> `readonly` **dbName**: `string`

Defined in: ridb-core/pkg/ridb\_core.d.ts:112

#### Inherited from

`BaseStorage.dbName`

***

### options

> `readonly` **options**: `BaseStorageOptions`

Defined in: ridb-core/pkg/ridb\_core.d.ts:114

#### Inherited from

`BaseStorage.options`

***

### schemas

> `readonly` **schemas**: `Record`\<keyof `T`, `Schema`\<`T`\[keyof `T`\]\>\>

Defined in: ridb-core/pkg/ridb\_core.d.ts:113

#### Inherited from

`BaseStorage.schemas`

## Methods

### addIndexSchemas()

> **addIndexSchemas**(): `null`

Defined in: ridb-core/pkg/ridb\_core.d.ts:125

#### Returns

`null`

#### Inherited from

`BaseStorage.addIndexSchemas`

***

### close()

> **close**(): `Promise`\<`void`\>

Defined in: [ridb-level/src/index.ts:57](https://github.com/trust0-project/RIDB/blob/99c65071b9fa26908fb7a847098f8c5c3969f24b/packages/ridb-level/src/index.ts#L57)

Close the database

#### Returns

`Promise`\<`void`\>

#### Overrides

`BaseStorage.close`

***

### count()

> **count**(`collectionName`, `query`): `Promise`\<`number`\>

Defined in: [ridb-level/src/index.ts:125](https://github.com/trust0-project/RIDB/blob/99c65071b9fa26908fb7a847098f8c5c3969f24b/packages/ridb-level/src/index.ts#L125)

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

***

### find()

> **find**(`collectionName`, `query`): `Promise`\<`Doc`\<`T`\[keyof `T`\]\>[]\>

Defined in: [ridb-level/src/index.ts:145](https://github.com/trust0-project/RIDB/blob/99c65071b9fa26908fb7a847098f8c5c3969f24b/packages/ridb-level/src/index.ts#L145)

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

***

### findDocumentById()

> **findDocumentById**(`collectionName`, `id`): `Promise`\<`undefined` \| `Doc`\<`T`\[keyof `T`\]\>\>

Defined in: [ridb-level/src/index.ts:61](https://github.com/trust0-project/RIDB/blob/99c65071b9fa26908fb7a847098f8c5c3969f24b/packages/ridb-level/src/index.ts#L61)

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

***

### getOption()

> **getOption**(`name`): `undefined` \| `string` \| `number` \| `boolean`

Defined in: ridb-core/pkg/ridb\_core.d.ts:122

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

Defined in: ridb-core/pkg/ridb\_core.d.ts:123

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

Defined in: [ridb-level/src/index.ts:53](https://github.com/trust0-project/RIDB/blob/99c65071b9fa26908fb7a847098f8c5c3969f24b/packages/ridb-level/src/index.ts#L53)

Start the database

#### Returns

`Promise`\<`void`\>

#### Overrides

`BaseStorage.start`

***

### write()

> **write**(`op`): `Promise`\<`Doc`\<`T`\[keyof `T`\]\>\>

Defined in: [ridb-level/src/index.ts:82](https://github.com/trust0-project/RIDB/blob/99c65071b9fa26908fb7a847098f8c5c3969f24b/packages/ridb-level/src/index.ts#L82)

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

> `static` **create**\<`SchemasCreate`\>(`name`, `schemas`, `options`): `Promise`\<`BaseStorage`\<`SchemasCreate`\>\>

Defined in: [ridb-level/src/index.ts:39](https://github.com/trust0-project/RIDB/blob/99c65071b9fa26908fb7a847098f8c5c3969f24b/packages/ridb-level/src/index.ts#L39)

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
