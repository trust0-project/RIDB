[**Documentation**](../../../README.md)

***

[Documentation](../../../README.md) / [@trust0/ridb-level](../README.md) / LevelDBStorage

# Class: LevelDBStorage\<T\>

Defined in: [index.ts:47](https://github.com/trust0-project/RIDB/blob/2a07066072231c925f10d0ad0c5af414f1bfe85b/packages/ridb-level/src/index.ts#L47)

LevelDB storage implementation class

## Extends

- `BaseStorage`\<`T`\>

## Type Parameters

### T

`T` *extends* [`SchemaTypeRecord`](https://github.com/trust0-project/RIDB/blob/main/docs/%40trust0/ridb-core/type-aliases/SchemaTypeRecord.md)

## Constructors

### Constructor

> **new LevelDBStorage**\<`T`\>(`level`, `name`, `schemas`, `options`): `LevelDBStorage`\<`T`\>

Defined in: [index.ts:48](https://github.com/trust0-project/RIDB/blob/2a07066072231c925f10d0ad0c5af414f1bfe85b/packages/ridb-level/src/index.ts#L48)

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

Defined in: ../../ridb-core/build/ridb\_core.d.ts:368

#### Inherited from

`BaseStorageType.core`

***

### dbName

> `readonly` **dbName**: `string`

Defined in: ../../ridb-core/build/ridb\_core.d.ts:365

#### Inherited from

`BaseStorageType.dbName`

***

### level

> **level**: [`Level`](../type-aliases/Level.md)

Defined in: [index.ts:48](https://github.com/trust0-project/RIDB/blob/2a07066072231c925f10d0ad0c5af414f1bfe85b/packages/ridb-level/src/index.ts#L48)

***

### options

> `readonly` **options**: `BaseStorageOptions`

Defined in: ../../ridb-core/build/ridb\_core.d.ts:367

#### Inherited from

`BaseStorageType.options`

***

### schemas

> `readonly` **schemas**: `Record`\<keyof `Schemas`, `Schema`\<`Schemas`\[keyof `Schemas`\]\>\>

Defined in: ../../ridb-core/build/ridb\_core.d.ts:366

#### Inherited from

`BaseStorageType.schemas`

## Methods

### addIndexSchemas()

> **addIndexSchemas**(): `null`

Defined in: ../../ridb-core/build/ridb\_core.d.ts:378

#### Returns

`null`

#### Inherited from

`BaseStorageType.addIndexSchemas`

***

### close()

> **close**(): `Promise`\<`void`\>

Defined in: [index.ts:77](https://github.com/trust0-project/RIDB/blob/2a07066072231c925f10d0ad0c5af414f1bfe85b/packages/ridb-level/src/index.ts#L77)

Close the database

#### Returns

`Promise`\<`void`\>

#### Overrides

`BaseStorageType.close`

***

### count()

> **count**(`collectionName`, `query`, `options?`): `Promise`\<`number`\>

Defined in: [index.ts:148](https://github.com/trust0-project/RIDB/blob/2a07066072231c925f10d0ad0c5af414f1bfe85b/packages/ridb-level/src/index.ts#L148)

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

Defined in: [index.ts:197](https://github.com/trust0-project/RIDB/blob/2a07066072231c925f10d0ad0c5af414f1bfe85b/packages/ridb-level/src/index.ts#L197)

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

Defined in: [index.ts:82](https://github.com/trust0-project/RIDB/blob/2a07066072231c925f10d0ad0c5af414f1bfe85b/packages/ridb-level/src/index.ts#L82)

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

Defined in: ../../ridb-core/build/ridb\_core.d.ts:375

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

Defined in: ../../ridb-core/build/ridb\_core.d.ts:376

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

Defined in: [index.ts:72](https://github.com/trust0-project/RIDB/blob/2a07066072231c925f10d0ad0c5af414f1bfe85b/packages/ridb-level/src/index.ts#L72)

Start the database

#### Returns

`Promise`\<`void`\>

#### Overrides

`BaseStorageType.start`

***

### write()

> **write**(`op`): `Promise`\<`Doc`\<`T`\[keyof `T`\]\>\>

Defined in: [index.ts:104](https://github.com/trust0-project/RIDB/blob/2a07066072231c925f10d0ad0c5af414f1bfe85b/packages/ridb-level/src/index.ts#L104)

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

Defined in: [index.ts:60](https://github.com/trust0-project/RIDB/blob/2a07066072231c925f10d0ad0c5af414f1bfe85b/packages/ridb-level/src/index.ts#L60)

Create a new LevelDB storage instance

#### Type Parameters

##### SchemasCreate

`SchemasCreate` *extends* [`SchemaTypeRecord`](https://github.com/trust0-project/RIDB/blob/main/docs/%40trust0/ridb-core/type-aliases/SchemaTypeRecord.md)

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
