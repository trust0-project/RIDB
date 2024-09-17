[**@elribonazo/ridb**](../../../README.md) • **Docs**

***

[@elribonazo/ridb](../../../README.md) / [RIDBTypes](../README.md) / StorageInternal

# Class: `abstract` StorageInternal\<T\>

Represents the internal storage interface with abstract methods for various storage operations.

## Extended by

- [`BaseStorage`](BaseStorage.md)

## Type Parameters

• **T** *extends* [`SchemaType`](../type-aliases/SchemaType.md)

The schema type.

## Constructors

### new StorageInternal()

> **new StorageInternal**\<`T`\>(): [`StorageInternal`](StorageInternal.md)\<`T`\>

#### Returns

[`StorageInternal`](StorageInternal.md)\<`T`\>

## Methods

### close()

> `abstract` **close**(): `Promise`\<`void`\>

Closes the storage.

#### Returns

`Promise`\<`void`\>

A promise that resolves when the storage is closed.

#### Defined in

pkg/ridb\_rust.d.ts:314

***

### count()

> `abstract` **count**(`query`): `Promise`\<`number`\>

Counts the number of documents in the storage.

#### Parameters

• **query**: [`QueryType`](../type-aliases/QueryType.md)\<`T`\>

#### Returns

`Promise`\<`number`\>

A promise that resolves to the number of documents.

#### Defined in

pkg/ridb\_rust.d.ts:299

***

### find()

> `abstract` **find**(`query`): `Promise`\<[`Doc`](../type-aliases/Doc.md)\<`T`\>[]\>

Queries the storage.

#### Parameters

• **query**: [`QueryType`](../type-aliases/QueryType.md)\<`T`\>

#### Returns

`Promise`\<[`Doc`](../type-aliases/Doc.md)\<`T`\>[]\>

A promise that resolves when the query is complete.

#### Defined in

pkg/ridb\_rust.d.ts:284

***

### findDocumentById()

> `abstract` **findDocumentById**(`id`): `Promise`\<`null` \| [`Doc`](../type-aliases/Doc.md)\<`T`\>\>

Finds a document by its ID.

#### Parameters

• **id**: `string`

The ID of the document to find.

#### Returns

`Promise`\<`null` \| [`Doc`](../type-aliases/Doc.md)\<`T`\>\>

A promise that resolves to the found document or null.

#### Defined in

pkg/ridb\_rust.d.ts:292

***

### remove()

> `abstract` **remove**(`id`): `Promise`\<`void`\>

Removes a document by its ID.

#### Parameters

• **id**: `string`

The ID of the document to remove.

#### Returns

`Promise`\<`void`\>

A promise that resolves when the document is removed.

#### Defined in

pkg/ridb\_rust.d.ts:307

***

### write()

> `abstract` **write**(`op`): `Promise`\<[`Doc`](../type-aliases/Doc.md)\<`T`\>\>

Writes an operation to the storage.

#### Parameters

• **op**: [`Operation`](../type-aliases/Operation.md)\<`T`\>

The operation to write.

#### Returns

`Promise`\<[`Doc`](../type-aliases/Doc.md)\<`T`\>\>

A promise that resolves to the document written.

#### Defined in

pkg/ridb\_rust.d.ts:277
