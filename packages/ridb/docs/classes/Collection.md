[**@trust0/ridb**](../README.md)

***

[@trust0/ridb](../README.md) / Collection

# Class: Collection\<T\>

Collection is a class that represents a collection of documents in a database.

## Type Parameters

• **T** *extends* [`SchemaType`](../type-aliases/SchemaType.md)

A schema type defining the structure of the documents in the collection.

## Constructors

### new Collection()

> **new Collection**\<`T`\>(): [`Collection`](Collection.md)\<`T`\>

#### Returns

[`Collection`](Collection.md)\<`T`\>

## Methods

### count()

> **count**(`query`): `Promise`\<`number`\>

count all documents in the collection.

#### Parameters

##### query

[`QueryType`](../type-aliases/QueryType.md)\<`T`\>

#### Returns

`Promise`\<`number`\>

A promise that resolves to an array of documents.

#### Defined in

ridb-wasm/pkg/ridb\_wasm.d.ts:305

***

### create()

> **create**(`document`): `Promise`\<[`Doc`](../type-aliases/Doc.md)\<`T`\>\>

Creates a new document in the collection.

#### Parameters

##### document

[`Doc`](../type-aliases/Doc.md)\<`T`\>

The document to create.

#### Returns

`Promise`\<[`Doc`](../type-aliases/Doc.md)\<`T`\>\>

A promise that resolves to the created document.

#### Defined in

ridb-wasm/pkg/ridb\_wasm.d.ts:327

***

### delete()

> **delete**(`id`): `Promise`\<`void`\>

Deletes a document in the collection by its ID.

#### Parameters

##### id

`string`

The ID of the document to delete.

#### Returns

`Promise`\<`void`\>

A promise that resolves when the deletion is complete.

#### Defined in

ridb-wasm/pkg/ridb\_wasm.d.ts:334

***

### find()

> **find**(`query`): `Promise`\<[`Doc`](../type-aliases/Doc.md)\<`T`\>[]\>

Finds all documents in the collection.

#### Parameters

##### query

[`QueryType`](../type-aliases/QueryType.md)\<`T`\>

#### Returns

`Promise`\<[`Doc`](../type-aliases/Doc.md)\<`T`\>[]\>

A promise that resolves to an array of documents.

#### Defined in

ridb-wasm/pkg/ridb\_wasm.d.ts:299

***

### findById()

> **findById**(`id`): `Promise`\<[`Doc`](../type-aliases/Doc.md)\<`T`\>\>

Finds a single document in the collection by its ID.

#### Parameters

##### id

`string`

The ID of the document to find.

#### Returns

`Promise`\<[`Doc`](../type-aliases/Doc.md)\<`T`\>\>

A promise that resolves to the found document.

#### Defined in

ridb-wasm/pkg/ridb\_wasm.d.ts:312

***

### update()

> **update**(`document`): `Promise`\<`void`\>

Updates a document in the collection by its ID.

#### Parameters

##### document

`Partial`\<[`Doc`](../type-aliases/Doc.md)\<`T`\>\>

A partial document containing the fields to update.

#### Returns

`Promise`\<`void`\>

A promise that resolves when the update is complete.

#### Defined in

ridb-wasm/pkg/ridb\_wasm.d.ts:320
