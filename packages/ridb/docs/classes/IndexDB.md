[**@trust0/ridb**](../README.md)

***

[@trust0/ridb](../README.md) / IndexDB

# Class: IndexDB\<T\>

Represents an IndexDB storage system extending the base storage functionality.

## Extends

- [`BaseStorage`](BaseStorage.md)\<`T`\>

## Type Parameters

• **T** *extends* [`SchemaTypeRecord`](../type-aliases/SchemaTypeRecord.md)

The schema type.

## Constructors

### new IndexDB()

> **new IndexDB**\<`T`\>(`dbName`, `schemas`, `options`?): [`IndexDB`](IndexDB.md)\<`T`\>

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

#### Defined in

node\_modules/@trust0/ridb-wasm/ridb\_wasm.d.ts:394

## Properties

### core

> `readonly` **core**: [`CoreStorage`](CoreStorage.md)

#### Inherited from

[`BaseStorage`](BaseStorage.md).[`core`](BaseStorage.md#core)

#### Defined in

node\_modules/@trust0/ridb-wasm/ridb\_wasm.d.ts:403

***

### dbName

> `readonly` **dbName**: `string`

#### Inherited from

[`BaseStorage`](BaseStorage.md).[`dbName`](BaseStorage.md#dbname-1)

#### Defined in

node\_modules/@trust0/ridb-wasm/ridb\_wasm.d.ts:400

***

### options

> `readonly` **options**: [`BaseStorageOptions`](../type-aliases/BaseStorageOptions.md)

#### Inherited from

[`BaseStorage`](BaseStorage.md).[`options`](BaseStorage.md#options-1)

#### Defined in

node\_modules/@trust0/ridb-wasm/ridb\_wasm.d.ts:402

***

### schemas

> `readonly` **schemas**: `Record`\<keyof `T`, [`Schema`](Schema.md)\<`T`\[keyof `T`\]\>\>

#### Inherited from

[`BaseStorage`](BaseStorage.md).[`schemas`](BaseStorage.md#schemas-1)

#### Defined in

node\_modules/@trust0/ridb-wasm/ridb\_wasm.d.ts:401

## Methods

### close()

> **close**(): `Promise`\<`void`\>

#### Returns

`Promise`\<`void`\>

#### Inherited from

[`BaseStorage`](BaseStorage.md).[`close`](BaseStorage.md#close)

#### Defined in

node\_modules/@trust0/ridb-wasm/ridb\_wasm.d.ts:405

***

### count()

> **count**(`colectionName`, `query`): `Promise`\<`number`\>

#### Parameters

##### colectionName

keyof `T`

##### query

[`QueryType`](../type-aliases/QueryType.md)\<`T`\[keyof `T`\]\>

#### Returns

`Promise`\<`number`\>

#### Inherited from

[`BaseStorage`](BaseStorage.md).[`count`](BaseStorage.md#count)

#### Defined in

node\_modules/@trust0/ridb-wasm/ridb\_wasm.d.ts:406

***

### find()

> **find**(`collectionName`, `query`): `Promise`\<[`Doc`](../type-aliases/Doc.md)\<`T`\[keyof `T`\]\>[]\>

#### Parameters

##### collectionName

keyof `T`

##### query

[`QueryType`](../type-aliases/QueryType.md)\<`T`\[keyof `T`\]\>

#### Returns

`Promise`\<[`Doc`](../type-aliases/Doc.md)\<`T`\[keyof `T`\]\>[]\>

#### Inherited from

[`BaseStorage`](BaseStorage.md).[`find`](BaseStorage.md#find)

#### Defined in

node\_modules/@trust0/ridb-wasm/ridb\_wasm.d.ts:408

***

### findDocumentById()

> **findDocumentById**(`collectionName`, `id`): `Promise`\<`null` \| [`Doc`](../type-aliases/Doc.md)\<`T`\[keyof `T`\]\>\>

#### Parameters

##### collectionName

keyof `T`

##### id

`string`

#### Returns

`Promise`\<`null` \| [`Doc`](../type-aliases/Doc.md)\<`T`\[keyof `T`\]\>\>

#### Inherited from

[`BaseStorage`](BaseStorage.md).[`findDocumentById`](BaseStorage.md#finddocumentbyid)

#### Defined in

node\_modules/@trust0/ridb-wasm/ridb\_wasm.d.ts:407

***

### free()

> **free**(): `void`

Frees the resources used by the in-memory storage.

#### Returns

`void`

#### Defined in

node\_modules/@trust0/ridb-wasm/ridb\_wasm.d.ts:75

***

### getOption()

> **getOption**(`name`): `undefined` \| `string` \| `number` \| `boolean`

#### Parameters

##### name

`string`

#### Returns

`undefined` \| `string` \| `number` \| `boolean`

#### Inherited from

[`BaseStorage`](BaseStorage.md).[`getOption`](BaseStorage.md#getoption)

#### Defined in

node\_modules/@trust0/ridb-wasm/ridb\_wasm.d.ts:411

***

### getSchema()

> **getSchema**(`name`): [`Schema`](Schema.md)\<`any`\>

#### Parameters

##### name

`string`

#### Returns

[`Schema`](Schema.md)\<`any`\>

#### Inherited from

[`BaseStorage`](BaseStorage.md).[`getSchema`](BaseStorage.md#getschema)

#### Defined in

node\_modules/@trust0/ridb-wasm/ridb\_wasm.d.ts:412

***

### start()

> **start**(): `Promise`\<`void`\>

#### Returns

`Promise`\<`void`\>

#### Inherited from

[`BaseStorage`](BaseStorage.md).[`start`](BaseStorage.md#start)

#### Defined in

node\_modules/@trust0/ridb-wasm/ridb\_wasm.d.ts:404

***

### write()

> **write**(`op`): `Promise`\<[`Doc`](../type-aliases/Doc.md)\<`T`\[keyof `T`\]\>\>

#### Parameters

##### op

[`Operation`](../type-aliases/Operation.md)\<`T`\[keyof `T`\]\>

#### Returns

`Promise`\<[`Doc`](../type-aliases/Doc.md)\<`T`\[keyof `T`\]\>\>

#### Inherited from

[`BaseStorage`](BaseStorage.md).[`write`](BaseStorage.md#write)

#### Defined in

node\_modules/@trust0/ridb-wasm/ridb\_wasm.d.ts:409

***

### create()

> `static` **create**\<`SchemasCreate`\>(`dbName`, `schemas`): `Promise`\<[`IndexDB`](IndexDB.md)\<`SchemasCreate`\>\>

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

#### Defined in

node\_modules/@trust0/ridb-wasm/ridb\_wasm.d.ts:77
