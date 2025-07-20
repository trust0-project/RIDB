[**Documentation**](../../../README.md)

***

[Documentation](../../../README.md) / [@trust0/ridb-core](../README.md) / IndexDB

# Class: IndexDB\<T\>

Defined in: ridb\_core.d.ts:145

Represents an IndexDB storage system extending the base storage functionality.

## Extends

- [`BaseStorage`](BaseStorage.md)\<`T`\>

## Type Parameters

### T

`T` *extends* [`SchemaTypeRecord`](../type-aliases/SchemaTypeRecord.md)

The schema type.

## Constructors

### Constructor

> **new IndexDB**\<`T`\>(`dbName`, `schemas`, `options?`): `IndexDB`\<`T`\>

Defined in: ridb\_core.d.ts:581

#### Parameters

##### dbName

`string`

##### schemas

`T`

##### options?

[`BaseStorageOptions`](../type-aliases/BaseStorageOptions.md)

#### Returns

`IndexDB`\<`T`\>

#### Inherited from

[`BaseStorage`](BaseStorage.md).[`constructor`](BaseStorage.md#constructor)

## Properties

### core

> `readonly` **core**: [`CoreStorage`](CoreStorage.md)

Defined in: ridb\_core.d.ts:589

#### Inherited from

[`BaseStorage`](BaseStorage.md).[`core`](BaseStorage.md#core)

***

### dbName

> `readonly` **dbName**: `string`

Defined in: ridb\_core.d.ts:586

#### Inherited from

[`BaseStorage`](BaseStorage.md).[`dbName`](BaseStorage.md#dbname)

***

### options

> `readonly` **options**: [`BaseStorageOptions`](../type-aliases/BaseStorageOptions.md)

Defined in: ridb\_core.d.ts:588

#### Inherited from

[`BaseStorage`](BaseStorage.md).[`options`](BaseStorage.md#options)

***

### schemas

> `readonly` **schemas**: `Record`\<keyof `Schemas`, [`Schema`](Schema.md)\<`Schemas`\[keyof `Schemas`\]\>\>

Defined in: ridb\_core.d.ts:587

#### Inherited from

[`BaseStorage`](BaseStorage.md).[`schemas`](BaseStorage.md#schemas-1)

## Methods

### addIndexSchemas()

> **addIndexSchemas**(): `null`

Defined in: ridb\_core.d.ts:599

#### Returns

`null`

#### Inherited from

[`BaseStorage`](BaseStorage.md).[`addIndexSchemas`](BaseStorage.md#addindexschemas)

***

### close()

> **close**(): `Promise`\<`void`\>

Defined in: ridb\_core.d.ts:591

#### Returns

`Promise`\<`void`\>

#### Inherited from

[`BaseStorage`](BaseStorage.md).[`close`](BaseStorage.md#close)

***

### count()

> **count**(`colectionName`, `query`, `options?`): `Promise`\<`number`\>

Defined in: ridb\_core.d.ts:592

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

> **find**(`collectionName`, `query`, `options?`): `Promise`\<[`Doc`](../type-aliases/Doc.md)\<`T`\[keyof `T`\]\>[]\>

Defined in: ridb\_core.d.ts:594

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

Defined in: ridb\_core.d.ts:593

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

Defined in: ridb\_core.d.ts:149

Frees the resources used by the in-memory storage.

#### Returns

`void`

***

### getOption()

> **getOption**(`name`): `undefined` \| `string` \| `number` \| `boolean`

Defined in: ridb\_core.d.ts:596

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

Defined in: ridb\_core.d.ts:597

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

Defined in: ridb\_core.d.ts:590

#### Returns

`Promise`\<`void`\>

#### Inherited from

[`BaseStorage`](BaseStorage.md).[`start`](BaseStorage.md#start)

***

### write()

> **write**(`op`): `Promise`\<[`Doc`](../type-aliases/Doc.md)\<`T`\[keyof `T`\]\>\>

Defined in: ridb\_core.d.ts:595

#### Parameters

##### op

[`Operation`](../type-aliases/Operation.md)\<`T`\[keyof `T`\]\>

#### Returns

`Promise`\<[`Doc`](../type-aliases/Doc.md)\<`T`\[keyof `T`\]\>\>

#### Inherited from

[`BaseStorage`](BaseStorage.md).[`write`](BaseStorage.md#write)

***

### create()

> `static` **create**\<`SchemasCreate`\>(`dbName`, `schemas`): `Promise`\<`IndexDB`\<`SchemasCreate`\>\>

Defined in: ridb\_core.d.ts:151

#### Type Parameters

##### SchemasCreate

`SchemasCreate` *extends* [`SchemaTypeRecord`](../type-aliases/SchemaTypeRecord.md)

#### Parameters

##### dbName

`string`

##### schemas

`SchemasCreate`

#### Returns

`Promise`\<`IndexDB`\<`SchemasCreate`\>\>

#### Overrides

[`BaseStorage`](BaseStorage.md).[`create`](BaseStorage.md#create)
