[**@trust0/ridb**](../README.md)

***

[@trust0/ridb](../README.md) / InMemory

# Class: InMemory\<T\>

Represents an in-memory storage system extending the base storage functionality.

## Extends

- [`BaseStorage`](BaseStorage.md)\<`T`\>

## Type Parameters

• **T** *extends* [`SchemaTypeRecord`](../type-aliases/SchemaTypeRecord.md)

The schema type.

## Constructors

### new InMemory()

> **new InMemory**\<`T`\>(`dbName`, `schemas`, `options`?): [`InMemory`](InMemory.md)\<`T`\>

#### Parameters

##### dbName

`string`

##### schemas

`T`

##### options?

[`BaseStorageOptions`](../type-aliases/BaseStorageOptions.md)

#### Returns

[`InMemory`](InMemory.md)\<`T`\>

#### Inherited from

[`BaseStorage`](BaseStorage.md).[`constructor`](BaseStorage.md#constructors)

#### Defined in

ridb-wasm/pkg/ridb\_wasm.d.ts:190

## Properties

### core

> `readonly` **core**: [`CoreStorage`](CoreStorage.md)

#### Inherited from

[`BaseStorage`](BaseStorage.md).[`core`](BaseStorage.md#core)

#### Defined in

ridb-wasm/pkg/ridb\_wasm.d.ts:199

***

### dbName

> `readonly` **dbName**: `string`

#### Inherited from

[`BaseStorage`](BaseStorage.md).[`dbName`](BaseStorage.md#dbname-1)

#### Defined in

ridb-wasm/pkg/ridb\_wasm.d.ts:196

***

### options

> `readonly` **options**: [`BaseStorageOptions`](../type-aliases/BaseStorageOptions.md)

#### Inherited from

[`BaseStorage`](BaseStorage.md).[`options`](BaseStorage.md#options-1)

#### Defined in

ridb-wasm/pkg/ridb\_wasm.d.ts:198

***

### schemas

> `readonly` **schemas**: `Record`\<keyof `T`, [`Schema`](Schema.md)\<`T`\[keyof `T`\]\>\>

#### Inherited from

[`BaseStorage`](BaseStorage.md).[`schemas`](BaseStorage.md#schemas-1)

#### Defined in

ridb-wasm/pkg/ridb\_wasm.d.ts:197

## Methods

### close()

> **close**(): `Promise`\<`void`\>

#### Returns

`Promise`\<`void`\>

#### Inherited from

[`BaseStorage`](BaseStorage.md).[`close`](BaseStorage.md#close)

#### Defined in

ridb-wasm/pkg/ridb\_wasm.d.ts:201

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

ridb-wasm/pkg/ridb\_wasm.d.ts:202

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

ridb-wasm/pkg/ridb\_wasm.d.ts:204

***

### findDocumentById()

> **findDocumentById**(`collectionName`, `id`): `Promise`\<`undefined` \| `null` \| [`Doc`](../type-aliases/Doc.md)\<`T`\[keyof `T`\]\>\>

#### Parameters

##### collectionName

keyof `T`

##### id

`string`

#### Returns

`Promise`\<`undefined` \| `null` \| [`Doc`](../type-aliases/Doc.md)\<`T`\[keyof `T`\]\>\>

#### Inherited from

[`BaseStorage`](BaseStorage.md).[`findDocumentById`](BaseStorage.md#finddocumentbyid)

#### Defined in

ridb-wasm/pkg/ridb\_wasm.d.ts:203

***

### free()

> **free**(): `void`

Frees the resources used by the in-memory storage.

#### Returns

`void`

#### Defined in

ridb-wasm/pkg/ridb\_wasm.d.ts:659

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

ridb-wasm/pkg/ridb\_wasm.d.ts:207

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

ridb-wasm/pkg/ridb\_wasm.d.ts:208

***

### start()

> **start**(): `Promise`\<`void`\>

#### Returns

`Promise`\<`void`\>

#### Inherited from

[`BaseStorage`](BaseStorage.md).[`start`](BaseStorage.md#start)

#### Defined in

ridb-wasm/pkg/ridb\_wasm.d.ts:200

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

ridb-wasm/pkg/ridb\_wasm.d.ts:205

***

### create()

> `static` **create**\<`SchemasCreate`\>(`dbName`, `schemas`): `Promise`\<[`InMemory`](InMemory.md)\<`SchemasCreate`\>\>

#### Type Parameters

• **SchemasCreate** *extends* [`SchemaTypeRecord`](../type-aliases/SchemaTypeRecord.md)

#### Parameters

##### dbName

`string`

##### schemas

`SchemasCreate`

#### Returns

`Promise`\<[`InMemory`](InMemory.md)\<`SchemasCreate`\>\>

#### Overrides

[`BaseStorage`](BaseStorage.md).[`create`](BaseStorage.md#create)

#### Defined in

ridb-wasm/pkg/ridb\_wasm.d.ts:661
