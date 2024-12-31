[**@trust0/ridb**](../README.md)

***

[@trust0/ridb](../README.md) / BaseStorage

# Class: BaseStorage\<Schemas\>

## Extends

- [`StorageInternal`](StorageInternal.md)\<`Schemas`\>

## Extended by

- [`IndexDB`](IndexDB.md)
- [`InMemory`](InMemory.md)

## Type Parameters

• **Schemas** *extends* [`SchemaTypeRecord`](../type-aliases/SchemaTypeRecord.md)

## Constructors

### new BaseStorage()

> **new BaseStorage**\<`Schemas`\>(`dbName`, `schemas`, `options`?): [`BaseStorage`](BaseStorage.md)\<`Schemas`\>

#### Parameters

##### dbName

`string`

##### schemas

`Schemas`

##### options?

[`BaseStorageOptions`](../type-aliases/BaseStorageOptions.md)

#### Returns

[`BaseStorage`](BaseStorage.md)\<`Schemas`\>

#### Overrides

[`StorageInternal`](StorageInternal.md).[`constructor`](StorageInternal.md#constructors)

#### Defined in

ridb-wasm/pkg/ridb\_wasm.d.ts:190

## Properties

### core

> `readonly` **core**: [`CoreStorage`](CoreStorage.md)

#### Defined in

ridb-wasm/pkg/ridb\_wasm.d.ts:199

***

### dbName

> `readonly` **dbName**: `string`

#### Defined in

ridb-wasm/pkg/ridb\_wasm.d.ts:196

***

### options

> `readonly` **options**: [`BaseStorageOptions`](../type-aliases/BaseStorageOptions.md)

#### Defined in

ridb-wasm/pkg/ridb\_wasm.d.ts:198

***

### schemas

> `readonly` **schemas**: `Record`\<keyof `Schemas`, [`Schema`](Schema.md)\<`Schemas`\[keyof `Schemas`\]\>\>

#### Defined in

ridb-wasm/pkg/ridb\_wasm.d.ts:197

## Methods

### close()

> **close**(): `Promise`\<`void`\>

#### Returns

`Promise`\<`void`\>

#### Overrides

[`StorageInternal`](StorageInternal.md).[`close`](StorageInternal.md#close)

#### Defined in

ridb-wasm/pkg/ridb\_wasm.d.ts:201

***

### count()

> **count**(`colectionName`, `query`): `Promise`\<`number`\>

#### Parameters

##### colectionName

keyof `Schemas`

##### query

[`QueryType`](../type-aliases/QueryType.md)\<`Schemas`\[keyof `Schemas`\]\>

#### Returns

`Promise`\<`number`\>

#### Overrides

[`StorageInternal`](StorageInternal.md).[`count`](StorageInternal.md#count)

#### Defined in

ridb-wasm/pkg/ridb\_wasm.d.ts:202

***

### find()

> **find**(`collectionName`, `query`): `Promise`\<[`Doc`](../type-aliases/Doc.md)\<`Schemas`\[keyof `Schemas`\]\>[]\>

#### Parameters

##### collectionName

keyof `Schemas`

##### query

[`QueryType`](../type-aliases/QueryType.md)\<`Schemas`\[keyof `Schemas`\]\>

#### Returns

`Promise`\<[`Doc`](../type-aliases/Doc.md)\<`Schemas`\[keyof `Schemas`\]\>[]\>

#### Overrides

[`StorageInternal`](StorageInternal.md).[`find`](StorageInternal.md#find)

#### Defined in

ridb-wasm/pkg/ridb\_wasm.d.ts:204

***

### findDocumentById()

> **findDocumentById**(`collectionName`, `id`): `Promise`\<`undefined` \| `null` \| [`Doc`](../type-aliases/Doc.md)\<`Schemas`\[keyof `Schemas`\]\>\>

#### Parameters

##### collectionName

keyof `Schemas`

##### id

`string`

#### Returns

`Promise`\<`undefined` \| `null` \| [`Doc`](../type-aliases/Doc.md)\<`Schemas`\[keyof `Schemas`\]\>\>

#### Overrides

[`StorageInternal`](StorageInternal.md).[`findDocumentById`](StorageInternal.md#finddocumentbyid)

#### Defined in

ridb-wasm/pkg/ridb\_wasm.d.ts:203

***

### getOption()

> **getOption**(`name`): `undefined` \| `string` \| `number` \| `boolean`

#### Parameters

##### name

`string`

#### Returns

`undefined` \| `string` \| `number` \| `boolean`

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

#### Defined in

ridb-wasm/pkg/ridb\_wasm.d.ts:208

***

### start()

> **start**(): `Promise`\<`void`\>

#### Returns

`Promise`\<`void`\>

#### Overrides

[`StorageInternal`](StorageInternal.md).[`start`](StorageInternal.md#start)

#### Defined in

ridb-wasm/pkg/ridb\_wasm.d.ts:200

***

### write()

> **write**(`op`): `Promise`\<[`Doc`](../type-aliases/Doc.md)\<`Schemas`\[keyof `Schemas`\]\>\>

#### Parameters

##### op

[`Operation`](../type-aliases/Operation.md)\<`Schemas`\[keyof `Schemas`\]\>

#### Returns

`Promise`\<[`Doc`](../type-aliases/Doc.md)\<`Schemas`\[keyof `Schemas`\]\>\>

#### Overrides

[`StorageInternal`](StorageInternal.md).[`write`](StorageInternal.md#write)

#### Defined in

ridb-wasm/pkg/ridb\_wasm.d.ts:205

***

### create()

> `static` **create**\<`SchemasCreate`\>(`dbName`, `schemas`, `options`?): `Promise`\<[`BaseStorage`](BaseStorage.md)\<`SchemasCreate`\>\>

#### Type Parameters

• **SchemasCreate** *extends* [`SchemaTypeRecord`](../type-aliases/SchemaTypeRecord.md)

#### Parameters

##### dbName

`string`

##### schemas

`SchemasCreate`

##### options?

[`BaseStorageOptions`](../type-aliases/BaseStorageOptions.md)

#### Returns

`Promise`\<[`BaseStorage`](BaseStorage.md)\<`SchemasCreate`\>\>

#### Defined in

ridb-wasm/pkg/ridb\_wasm.d.ts:180
