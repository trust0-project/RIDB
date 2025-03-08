[**@trust0/ridb**](../README.md)

***

[@trust0/ridb](../README.md) / BaseStorage

# Class: BaseStorage\<Schemas\>

Defined in: node\_modules/@trust0/ridb-core/pkg/ridb\_core.d.ts:251

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

Defined in: node\_modules/@trust0/ridb-core/pkg/ridb\_core.d.ts:261

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

## Properties

### core

> `readonly` **core**: [`CoreStorage`](CoreStorage.md)

Defined in: node\_modules/@trust0/ridb-core/pkg/ridb\_core.d.ts:269

***

### dbName

> `readonly` **dbName**: `string`

Defined in: node\_modules/@trust0/ridb-core/pkg/ridb\_core.d.ts:266

***

### options

> `readonly` **options**: [`BaseStorageOptions`](../type-aliases/BaseStorageOptions.md)

Defined in: node\_modules/@trust0/ridb-core/pkg/ridb\_core.d.ts:268

***

### schemas

> `readonly` **schemas**: `Record`\<keyof `Schemas`, [`Schema`](Schema.md)\<`Schemas`\[keyof `Schemas`\]\>\>

Defined in: node\_modules/@trust0/ridb-core/pkg/ridb\_core.d.ts:267

## Methods

### addIndexSchemas()

> **addIndexSchemas**(): `null`

Defined in: node\_modules/@trust0/ridb-core/pkg/ridb\_core.d.ts:279

#### Returns

`null`

***

### close()

> **close**(): `Promise`\<`void`\>

Defined in: node\_modules/@trust0/ridb-core/pkg/ridb\_core.d.ts:271

#### Returns

`Promise`\<`void`\>

#### Overrides

[`StorageInternal`](StorageInternal.md).[`close`](StorageInternal.md#close)

***

### count()

> **count**(`colectionName`, `query`, `options`?): `Promise`\<`number`\>

Defined in: node\_modules/@trust0/ridb-core/pkg/ridb\_core.d.ts:272

#### Parameters

##### colectionName

keyof `Schemas`

##### query

[`QueryType`](../type-aliases/QueryType.md)\<`Schemas`\[keyof `Schemas`\]\>

##### options?

[`QueryOptions`](../type-aliases/QueryOptions.md)

#### Returns

`Promise`\<`number`\>

#### Overrides

[`StorageInternal`](StorageInternal.md).[`count`](StorageInternal.md#count)

***

### find()

> **find**(`collectionName`, `query`, `options`?): `Promise`\<[`Doc`](../type-aliases/Doc.md)\<`Schemas`\[keyof `Schemas`\]\>[]\>

Defined in: node\_modules/@trust0/ridb-core/pkg/ridb\_core.d.ts:274

#### Parameters

##### collectionName

keyof `Schemas`

##### query

[`QueryType`](../type-aliases/QueryType.md)\<`Schemas`\[keyof `Schemas`\]\>

##### options?

[`QueryOptions`](../type-aliases/QueryOptions.md)

#### Returns

`Promise`\<[`Doc`](../type-aliases/Doc.md)\<`Schemas`\[keyof `Schemas`\]\>[]\>

#### Overrides

[`StorageInternal`](StorageInternal.md).[`find`](StorageInternal.md#find)

***

### findDocumentById()

> **findDocumentById**(`collectionName`, `id`): `Promise`\<`null` \| [`Doc`](../type-aliases/Doc.md)\<`Schemas`\[keyof `Schemas`\]\>\>

Defined in: node\_modules/@trust0/ridb-core/pkg/ridb\_core.d.ts:273

#### Parameters

##### collectionName

keyof `Schemas`

##### id

`string`

#### Returns

`Promise`\<`null` \| [`Doc`](../type-aliases/Doc.md)\<`Schemas`\[keyof `Schemas`\]\>\>

#### Overrides

[`StorageInternal`](StorageInternal.md).[`findDocumentById`](StorageInternal.md#finddocumentbyid)

***

### getOption()

> **getOption**(`name`): `undefined` \| `string` \| `number` \| `boolean`

Defined in: node\_modules/@trust0/ridb-core/pkg/ridb\_core.d.ts:276

#### Parameters

##### name

`string`

#### Returns

`undefined` \| `string` \| `number` \| `boolean`

***

### getSchema()

> **getSchema**(`name`): [`Schema`](Schema.md)\<`any`\>

Defined in: node\_modules/@trust0/ridb-core/pkg/ridb\_core.d.ts:277

#### Parameters

##### name

`string`

#### Returns

[`Schema`](Schema.md)\<`any`\>

***

### start()

> **start**(): `Promise`\<`void`\>

Defined in: node\_modules/@trust0/ridb-core/pkg/ridb\_core.d.ts:270

#### Returns

`Promise`\<`void`\>

#### Overrides

[`StorageInternal`](StorageInternal.md).[`start`](StorageInternal.md#start)

***

### write()

> **write**(`op`): `Promise`\<[`Doc`](../type-aliases/Doc.md)\<`Schemas`\[keyof `Schemas`\]\>\>

Defined in: node\_modules/@trust0/ridb-core/pkg/ridb\_core.d.ts:275

#### Parameters

##### op

[`Operation`](../type-aliases/Operation.md)\<`Schemas`\[keyof `Schemas`\]\>

#### Returns

`Promise`\<[`Doc`](../type-aliases/Doc.md)\<`Schemas`\[keyof `Schemas`\]\>\>

#### Overrides

[`StorageInternal`](StorageInternal.md).[`write`](StorageInternal.md#write)

***

### create()

> `static` **create**\<`SchemasCreate`\>(`dbName`, `schemas`, `options`?): `Promise`\<[`BaseStorage`](BaseStorage.md)\<`SchemasCreate`\>\>

Defined in: node\_modules/@trust0/ridb-core/pkg/ridb\_core.d.ts:252

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
