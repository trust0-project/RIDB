[**@trust0/ridb**](../README.md)

***

[@trust0/ridb](../README.md) / StorageInternal

# Class: `abstract` StorageInternal\<Schemas\>

Defined in: node\_modules/@trust0/ridb-core/pkg/ridb\_core.d.ts:362

## Extended by

- [`BaseStorage`](BaseStorage.md)

## Type Parameters

• **Schemas** *extends* [`SchemaTypeRecord`](../type-aliases/SchemaTypeRecord.md)

## Constructors

### new StorageInternal()

> **new StorageInternal**\<`Schemas`\>(`name`, `schemas`): [`StorageInternal`](StorageInternal.md)\<`Schemas`\>

Defined in: node\_modules/@trust0/ridb-core/pkg/ridb\_core.d.ts:363

#### Parameters

##### name

`string`

##### schemas

`Schemas`

#### Returns

[`StorageInternal`](StorageInternal.md)\<`Schemas`\>

## Methods

### close()

> `abstract` **close**(): `Promise`\<`void`\>

Defined in: node\_modules/@trust0/ridb-core/pkg/ridb\_core.d.ts:368

#### Returns

`Promise`\<`void`\>

***

### count()

> `abstract` **count**(`colectionName`, `query`, `options`?): `Promise`\<`number`\>

Defined in: node\_modules/@trust0/ridb-core/pkg/ridb\_core.d.ts:369

#### Parameters

##### colectionName

keyof `Schemas`

##### query

[`QueryType`](../type-aliases/QueryType.md)\<`Schemas`\[keyof `Schemas`\]\>

##### options?

[`QueryOptions`](../type-aliases/QueryOptions.md)

#### Returns

`Promise`\<`number`\>

***

### find()

> `abstract` **find**(`collectionName`, `query`, `options`?): `Promise`\<[`Doc`](../type-aliases/Doc.md)\<`Schemas`\[keyof `Schemas`\]\>[]\>

Defined in: node\_modules/@trust0/ridb-core/pkg/ridb\_core.d.ts:378

#### Parameters

##### collectionName

keyof `Schemas`

##### query

[`QueryType`](../type-aliases/QueryType.md)\<`Schemas`\[keyof `Schemas`\]\>

##### options?

[`QueryOptions`](../type-aliases/QueryOptions.md)

#### Returns

`Promise`\<[`Doc`](../type-aliases/Doc.md)\<`Schemas`\[keyof `Schemas`\]\>[]\>

***

### findDocumentById()

> `abstract` **findDocumentById**(`collectionName`, `id`): `Promise`\<`null` \| [`Doc`](../type-aliases/Doc.md)\<`Schemas`\[keyof `Schemas`\]\>\>

Defined in: node\_modules/@trust0/ridb-core/pkg/ridb\_core.d.ts:374

#### Parameters

##### collectionName

keyof `Schemas`

##### id

`string`

#### Returns

`Promise`\<`null` \| [`Doc`](../type-aliases/Doc.md)\<`Schemas`\[keyof `Schemas`\]\>\>

***

### start()

> `abstract` **start**(): `Promise`\<`void`\>

Defined in: node\_modules/@trust0/ridb-core/pkg/ridb\_core.d.ts:367

#### Returns

`Promise`\<`void`\>

***

### write()

> `abstract` **write**(`op`): `Promise`\<[`Doc`](../type-aliases/Doc.md)\<`Schemas`\[keyof `Schemas`\]\>\>

Defined in: node\_modules/@trust0/ridb-core/pkg/ridb\_core.d.ts:383

#### Parameters

##### op

[`Operation`](../type-aliases/Operation.md)\<`Schemas`\[keyof `Schemas`\]\>

#### Returns

`Promise`\<[`Doc`](../type-aliases/Doc.md)\<`Schemas`\[keyof `Schemas`\]\>\>
