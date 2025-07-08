[**Documentation**](../../../README.md)

***

[Documentation](../../../README.md) / [@trust0/ridb-core](../README.md) / Collection

# Class: Collection\<T\>

Defined in: ridb\_core.d.ts:342

Collection is a class that represents a collection of documents in a database.

## Type Parameters

### T

`T` *extends* [`SchemaType`](../type-aliases/SchemaType.md)

A schema type defining the structure of the documents in the collection.

## Constructors

### Constructor

> **new Collection**\<`T`\>(): `Collection`\<`T`\>

#### Returns

`Collection`\<`T`\>

## Methods

### count()

> **count**(`query`, `options?`): `Promise`\<`number`\>

Defined in: ridb\_core.d.ts:354

count all documents in the collection.

#### Parameters

##### query

[`QueryType`](../type-aliases/QueryType.md)\<`T`\>

##### options?

[`QueryOptions`](../type-aliases/QueryOptions.md)

#### Returns

`Promise`\<`number`\>

A promise that resolves to an array of documents.

***

### create()

> **create**(`document`): `Promise`\<[`Doc`](../type-aliases/Doc.md)\<`T`\>\>

Defined in: ridb\_core.d.ts:375

Creates a new document in the collection.

#### Parameters

##### document

[`CreateDoc`](../type-aliases/CreateDoc.md)\<`T`\>

The document to create.

#### Returns

`Promise`\<[`Doc`](../type-aliases/Doc.md)\<`T`\>\>

A promise that resolves to the created document.

***

### delete()

> **delete**(`id`): `Promise`\<`void`\>

Defined in: ridb\_core.d.ts:382

Deletes a document in the collection by its ID.

#### Parameters

##### id

`string`

The ID of the document to delete.

#### Returns

`Promise`\<`void`\>

A promise that resolves when the deletion is complete.

***

### find()

> **find**(`query`, `options?`): `Promise`\<[`Doc`](../type-aliases/Doc.md)\<`T`\>[]\>

Defined in: ridb\_core.d.ts:348

Finds all documents in the collection.

#### Parameters

##### query

[`QueryType`](../type-aliases/QueryType.md)\<`T`\>

##### options?

[`QueryOptions`](../type-aliases/QueryOptions.md)

#### Returns

`Promise`\<[`Doc`](../type-aliases/Doc.md)\<`T`\>[]\>

A promise that resolves to an array of documents.

***

### findById()

> **findById**(`id`): `Promise`\<[`Doc`](../type-aliases/Doc.md)\<`T`\>\>

Defined in: ridb\_core.d.ts:361

Finds a single document in the collection by its ID.

#### Parameters

##### id

`string`

The ID of the document to find.

#### Returns

`Promise`\<[`Doc`](../type-aliases/Doc.md)\<`T`\>\>

A promise that resolves to the found document.

***

### update()

> **update**(`document`): `Promise`\<`void`\>

Defined in: ridb\_core.d.ts:368

Updates a document in the collection by its ID.

#### Parameters

##### document

`Partial`\<[`Doc`](../type-aliases/Doc.md)\<`T`\>\>

A partial document containing the fields to update.

#### Returns

`Promise`\<`void`\>

A promise that resolves when the update is complete.
