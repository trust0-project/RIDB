[**@trust0/ridb**](../README.md)

***

[@trust0/ridb](../README.md) / BaseStorage

# Class: BaseStorage\<Schemas\>

Defined in: ridb-core/pkg/ridb\_core.d.ts:97

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

Defined in: ridb-core/pkg/ridb\_core.d.ts:107

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

Defined in: ridb-core/pkg/ridb\_core.d.ts:115

***

### dbName

> `readonly` **dbName**: `string`

Defined in: ridb-core/pkg/ridb\_core.d.ts:112

***

### options

> `readonly` **options**: [`BaseStorageOptions`](../type-aliases/BaseStorageOptions.md)

Defined in: ridb-core/pkg/ridb\_core.d.ts:114

***

### schemas

> `readonly` **schemas**: `Record`\<keyof `Schemas`, [`Schema`](Schema.md)\<`Schemas`\[keyof `Schemas`\]\>\>

Defined in: ridb-core/pkg/ridb\_core.d.ts:113

## Methods

### addIndexSchemas()

> **addIndexSchemas**(): `null`

Defined in: ridb-core/pkg/ridb\_core.d.ts:125

#### Returns

`null`

***

### close()

> **close**(): `Promise`\<`void`\>

Defined in: ridb-core/pkg/ridb\_core.d.ts:117

#### Returns

`Promise`\<`void`\>

#### Overrides

[`StorageInternal`](StorageInternal.md).[`close`](StorageInternal.md#close)

***

### count()

> **count**(`colectionName`, `query`): `Promise`\<`number`\>

Defined in: ridb-core/pkg/ridb\_core.d.ts:118

#### Parameters

##### colectionName

keyof `Schemas`

##### query

[`QueryType`](../type-aliases/QueryType.md)\<`Schemas`\[keyof `Schemas`\]\>

#### Returns

`Promise`\<`number`\>

#### Overrides

[`StorageInternal`](StorageInternal.md).[`count`](StorageInternal.md#count)

***

### find()

> **find**(`collectionName`, `query`): `Promise`\<[`Doc`](../type-aliases/Doc.md)\<`Schemas`\[keyof `Schemas`\]\>[]\>

Defined in: ridb-core/pkg/ridb\_core.d.ts:120

#### Parameters

##### collectionName

keyof `Schemas`

##### query

[`QueryType`](../type-aliases/QueryType.md)\<`Schemas`\[keyof `Schemas`\]\>

#### Returns

`Promise`\<[`Doc`](../type-aliases/Doc.md)\<`Schemas`\[keyof `Schemas`\]\>[]\>

#### Overrides

[`StorageInternal`](StorageInternal.md).[`find`](StorageInternal.md#find)

***

### findDocumentById()

> **findDocumentById**(`collectionName`, `id`): `Promise`\<`undefined` \| `null` \| [`Doc`](../type-aliases/Doc.md)\<`Schemas`\[keyof `Schemas`\]\>\>

Defined in: ridb-core/pkg/ridb\_core.d.ts:119

#### Parameters

##### collectionName

keyof `Schemas`

##### id

`string`

#### Returns

`Promise`\<`undefined` \| `null` \| [`Doc`](../type-aliases/Doc.md)\<`Schemas`\[keyof `Schemas`\]\>\>

#### Overrides

[`StorageInternal`](StorageInternal.md).[`findDocumentById`](StorageInternal.md#finddocumentbyid)

***

### getOption()

> **getOption**(`name`): `undefined` \| `string` \| `number` \| `boolean`

Defined in: ridb-core/pkg/ridb\_core.d.ts:122

#### Parameters

##### name

`string`

#### Returns

`undefined` \| `string` \| `number` \| `boolean`

***

### getSchema()

> **getSchema**(`name`): [`Schema`](Schema.md)\<`any`\>

Defined in: ridb-core/pkg/ridb\_core.d.ts:123

#### Parameters

##### name

`string`

#### Returns

[`Schema`](Schema.md)\<`any`\>

***

### start()

> **start**(): `Promise`\<`void`\>

Defined in: ridb-core/pkg/ridb\_core.d.ts:116

#### Returns

`Promise`\<`void`\>

#### Overrides

[`StorageInternal`](StorageInternal.md).[`start`](StorageInternal.md#start)

***

### write()

> **write**(`op`): `Promise`\<[`Doc`](../type-aliases/Doc.md)\<`Schemas`\[keyof `Schemas`\]\>\>

Defined in: ridb-core/pkg/ridb\_core.d.ts:121

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

Defined in: ridb-core/pkg/ridb\_core.d.ts:98

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
