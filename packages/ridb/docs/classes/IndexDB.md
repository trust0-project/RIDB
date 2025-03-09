[**@trust0/ridb**](../README.md)

***

[@trust0/ridb](../README.md) / IndexDB

# Class: IndexDB\<T\>

Defined in: ridb-core/pkg/ridb\_core.d.ts:217

Represents an IndexDB storage system extending the base storage functionality.

## Extends

- [`BaseStorage`](BaseStorage.md)\<`T`\>

## Type Parameters

• **T** *extends* [`SchemaTypeRecord`](../type-aliases/SchemaTypeRecord.md)

The schema type.

## Constructors

### new IndexDB()

> **new IndexDB**\<`T`\>(`dbName`, `schemas`, `options`?): [`IndexDB`](IndexDB.md)\<`T`\>

Defined in: ridb-core/pkg/ridb\_core.d.ts:166

#### Parameters

##### dbName

`string`

##### schemas

`T`

##### options?

[`BaseStorageOptions`](../type-aliases/BaseStorageOptions.md)

#### Returns

[`IndexDB`](IndexDB.md)\<`T`\>

#### Inherited from

[`BaseStorage`](BaseStorage.md).[`constructor`](BaseStorage.md#constructors)

## Properties

### core

> `readonly` **core**: [`CoreStorage`](CoreStorage.md)

Defined in: ridb-core/pkg/ridb\_core.d.ts:174

#### Inherited from

[`BaseStorage`](BaseStorage.md).[`core`](BaseStorage.md#core)

***

### dbName

> `readonly` **dbName**: `string`

Defined in: ridb-core/pkg/ridb\_core.d.ts:171

#### Inherited from

[`BaseStorage`](BaseStorage.md).[`dbName`](BaseStorage.md#dbname-1)

***

### options

> `readonly` **options**: [`BaseStorageOptions`](../type-aliases/BaseStorageOptions.md)

Defined in: ridb-core/pkg/ridb\_core.d.ts:173

#### Inherited from

[`BaseStorage`](BaseStorage.md).[`options`](BaseStorage.md#options-1)

***

### schemas

> `readonly` **schemas**: `Record`\<keyof `T`, [`Schema`](Schema.md)\<`T`\[keyof `T`\]\>\>

Defined in: ridb-core/pkg/ridb\_core.d.ts:172

#### Inherited from

[`BaseStorage`](BaseStorage.md).[`schemas`](BaseStorage.md#schemas-1)

## Methods

### addIndexSchemas()

> **addIndexSchemas**(): `null`

Defined in: ridb-core/pkg/ridb\_core.d.ts:184

#### Returns

`null`

#### Inherited from

[`BaseStorage`](BaseStorage.md).[`addIndexSchemas`](BaseStorage.md#addindexschemas)

***

### close()

> **close**(): `Promise`\<`void`\>

Defined in: ridb-core/pkg/ridb\_core.d.ts:176

#### Returns

`Promise`\<`void`\>

#### Inherited from

[`BaseStorage`](BaseStorage.md).[`close`](BaseStorage.md#close)

***

### count()

> **count**(`colectionName`, `query`, `options`?): `Promise`\<`number`\>

Defined in: ridb-core/pkg/ridb\_core.d.ts:177

#### Parameters

##### colectionName

keyof `T`

##### query

[`QueryType`](../type-aliases/QueryType.md)\<`T`\[keyof `T`\]\>

##### options?

[`QueryOptions`](../type-aliases/QueryOptions.md)

#### Returns

`Promise`\<`number`\>

#### Inherited from

[`BaseStorage`](BaseStorage.md).[`count`](BaseStorage.md#count)

***

### find()

> **find**(`collectionName`, `query`, `options`?): `Promise`\<[`Doc`](../type-aliases/Doc.md)\<`T`\[keyof `T`\]\>[]\>

Defined in: ridb-core/pkg/ridb\_core.d.ts:179

#### Parameters

##### collectionName

keyof `T`

##### query

[`QueryType`](../type-aliases/QueryType.md)\<`T`\[keyof `T`\]\>

##### options?

[`QueryOptions`](../type-aliases/QueryOptions.md)

#### Returns

`Promise`\<[`Doc`](../type-aliases/Doc.md)\<`T`\[keyof `T`\]\>[]\>

#### Inherited from

[`BaseStorage`](BaseStorage.md).[`find`](BaseStorage.md#find)

***

### findDocumentById()

> **findDocumentById**(`collectionName`, `id`): `Promise`\<`null` \| [`Doc`](../type-aliases/Doc.md)\<`T`\[keyof `T`\]\>\>

Defined in: ridb-core/pkg/ridb\_core.d.ts:178

#### Parameters

##### collectionName

keyof `T`

##### id

`string`

#### Returns

`Promise`\<`null` \| [`Doc`](../type-aliases/Doc.md)\<`T`\[keyof `T`\]\>\>

#### Inherited from

[`BaseStorage`](BaseStorage.md).[`findDocumentById`](BaseStorage.md#finddocumentbyid)

***

### free()

> **free**(): `void`

Defined in: ridb-core/pkg/ridb\_core.d.ts:221

Frees the resources used by the in-memory storage.

#### Returns

`void`

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

[`BaseStorage`](BaseStorage.md).[`getOption`](BaseStorage.md#getoption)

***

### getSchema()

> **getSchema**(`name`): [`Schema`](Schema.md)\<`any`\>

Defined in: ridb-core/pkg/ridb\_core.d.ts:182

#### Parameters

##### name

`string`

#### Returns

[`Schema`](Schema.md)\<`any`\>

#### Inherited from

[`BaseStorage`](BaseStorage.md).[`getSchema`](BaseStorage.md#getschema)

***

### start()

> **start**(): `Promise`\<`void`\>

Defined in: ridb-core/pkg/ridb\_core.d.ts:175

#### Returns

`Promise`\<`void`\>

#### Inherited from

[`BaseStorage`](BaseStorage.md).[`start`](BaseStorage.md#start)

***

### write()

> **write**(`op`): `Promise`\<[`Doc`](../type-aliases/Doc.md)\<`T`\[keyof `T`\]\>\>

Defined in: ridb-core/pkg/ridb\_core.d.ts:180

#### Parameters

##### op

[`Operation`](../type-aliases/Operation.md)\<`T`\[keyof `T`\]\>

#### Returns

`Promise`\<[`Doc`](../type-aliases/Doc.md)\<`T`\[keyof `T`\]\>\>

#### Inherited from

[`BaseStorage`](BaseStorage.md).[`write`](BaseStorage.md#write)

***

### create()

> `static` **create**\<`SchemasCreate`\>(`dbName`, `schemas`): `Promise`\<[`IndexDB`](IndexDB.md)\<`SchemasCreate`\>\>

Defined in: ridb-core/pkg/ridb\_core.d.ts:223

#### Type Parameters

• **SchemasCreate** *extends* [`SchemaTypeRecord`](../type-aliases/SchemaTypeRecord.md)

#### Parameters

##### dbName

`string`

##### schemas

`SchemasCreate`

#### Returns

`Promise`\<[`IndexDB`](IndexDB.md)\<`SchemasCreate`\>\>

#### Overrides

[`BaseStorage`](BaseStorage.md).[`create`](BaseStorage.md#create)
