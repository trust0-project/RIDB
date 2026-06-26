[**Documentation**](../../../README.md)

***

[Documentation](../../../README.md) / [@trust0/ridb-core](../README.md) / BaseStorage

# Class: BaseStorage\<Schemas\>

Defined in: ridb\_core.d.ts:588

## Extends

- [`StorageInternal`](StorageInternal.md)\<`Schemas`\>

## Extended by

- [`InMemory`](InMemory.md)
- [`IndexDB`](IndexDB.md)

## Type Parameters

### Schemas

`Schemas` *extends* [`SchemaTypeRecord`](../type-aliases/SchemaTypeRecord.md)

## Constructors

### Constructor

> **new BaseStorage**\<`Schemas`\>(`dbName`, `schemas`, `options?`): `BaseStorage`\<`Schemas`\>

Defined in: ridb\_core.d.ts:598

#### Parameters

##### dbName

`string`

##### schemas

`Schemas`

##### options?

[`BaseStorageOptions`](../type-aliases/BaseStorageOptions.md)

#### Returns

`BaseStorage`\<`Schemas`\>

#### Overrides

[`StorageInternal`](StorageInternal.md).[`constructor`](StorageInternal.md#constructor)

## Properties

### core

> `readonly` **core**: [`CoreStorage`](CoreStorage.md)

Defined in: ridb\_core.d.ts:606

***

### dbName

> `readonly` **dbName**: `string`

Defined in: ridb\_core.d.ts:603

***

### options

> `readonly` **options**: [`BaseStorageOptions`](../type-aliases/BaseStorageOptions.md)

Defined in: ridb\_core.d.ts:605

***

### schemas

> `readonly` **schemas**: `Record`\<keyof `Schemas`, [`Schema`](Schema.md)\<`Schemas`\[keyof `Schemas`\]\>\>

Defined in: ridb\_core.d.ts:604

## Methods

### addIndexSchemas()

> **addIndexSchemas**(): `null`

Defined in: ridb\_core.d.ts:616

#### Returns

`null`

***

### close()

> **close**(): `Promise`\<`void`\>

Defined in: ridb\_core.d.ts:608

#### Returns

`Promise`\<`void`\>

#### Overrides

[`StorageInternal`](StorageInternal.md).[`close`](StorageInternal.md#close)

***

### count()

> **count**(`colectionName`, `query`, `options?`): `Promise`\<`number`\>

Defined in: ridb\_core.d.ts:609

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

> **find**(`collectionName`, `query`, `options?`): `Promise`\<[`Doc`](../type-aliases/Doc.md)\<`Schemas`\[keyof `Schemas`\]\>[]\>

Defined in: ridb\_core.d.ts:611

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

> **findDocumentById**(`collectionName`, `id`): `Promise`\<[`Doc`](../type-aliases/Doc.md)\<`Schemas`\[keyof `Schemas`\]\> \| `null`\>

Defined in: ridb\_core.d.ts:610

#### Parameters

##### collectionName

keyof `Schemas`

##### id

`string`

#### Returns

`Promise`\<[`Doc`](../type-aliases/Doc.md)\<`Schemas`\[keyof `Schemas`\]\> \| `null`\>

#### Overrides

[`StorageInternal`](StorageInternal.md).[`findDocumentById`](StorageInternal.md#finddocumentbyid)

***

### getOption()

> **getOption**(`name`): `string` \| `number` \| `boolean` \| `undefined`

Defined in: ridb\_core.d.ts:613

#### Parameters

##### name

`string`

#### Returns

`string` \| `number` \| `boolean` \| `undefined`

***

### getSchema()

> **getSchema**(`name`): [`Schema`](Schema.md)\<`any`\>

Defined in: ridb\_core.d.ts:614

#### Parameters

##### name

`string`

#### Returns

[`Schema`](Schema.md)\<`any`\>

***

### start()

> **start**(): `Promise`\<`void`\>

Defined in: ridb\_core.d.ts:607

#### Returns

`Promise`\<`void`\>

#### Overrides

[`StorageInternal`](StorageInternal.md).[`start`](StorageInternal.md#start)

***

### write()

> **write**(`op`): `Promise`\<[`Doc`](../type-aliases/Doc.md)\<`Schemas`\[keyof `Schemas`\]\>\>

Defined in: ridb\_core.d.ts:612

#### Parameters

##### op

[`Operation`](../type-aliases/Operation.md)\<`Schemas`\[keyof `Schemas`\]\>

#### Returns

`Promise`\<[`Doc`](../type-aliases/Doc.md)\<`Schemas`\[keyof `Schemas`\]\>\>

#### Overrides

[`StorageInternal`](StorageInternal.md).[`write`](StorageInternal.md#write)

***

### create()

> `static` **create**\<`SchemasCreate`\>(`dbName`, `schemas`, `options?`): `Promise`\<`BaseStorage`\<`SchemasCreate`\>\>

Defined in: ridb\_core.d.ts:589

#### Type Parameters

##### SchemasCreate

`SchemasCreate` *extends* [`SchemaTypeRecord`](../type-aliases/SchemaTypeRecord.md)

#### Parameters

##### dbName

`string`

##### schemas

`SchemasCreate`

##### options?

[`BaseStorageOptions`](../type-aliases/BaseStorageOptions.md)

#### Returns

`Promise`\<`BaseStorage`\<`SchemasCreate`\>\>
