[**@trust0/ridb**](../README.md)

***

[@trust0/ridb](../README.md) / Collection

# Class: Collection\<T\>

Defined in: node\_modules/@trust0/ridb-core/pkg/ridb\_core.d.ts:200

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

> **count**(`query`, `options`?): `Promise`\<`number`\>

Defined in: node\_modules/@trust0/ridb-core/pkg/ridb\_core.d.ts:212

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

Defined in: node\_modules/@trust0/ridb-core/pkg/ridb\_core.d.ts:234

Creates a new document in the collection.

#### Parameters

##### document

[`Doc`](../type-aliases/Doc.md)\<`T`\>

The document to create.

#### Returns

`Promise`\<[`Doc`](../type-aliases/Doc.md)\<`T`\>\>

A promise that resolves to the created document.

***

### delete()

> **delete**(`id`): `Promise`\<`void`\>

Defined in: node\_modules/@trust0/ridb-core/pkg/ridb\_core.d.ts:241

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

> **find**(`query`, `options`?): `Promise`\<[`Doc`](../type-aliases/Doc.md)\<`T`\>[]\>

Defined in: node\_modules/@trust0/ridb-core/pkg/ridb\_core.d.ts:206

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

Defined in: node\_modules/@trust0/ridb-core/pkg/ridb\_core.d.ts:219

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

Defined in: node\_modules/@trust0/ridb-core/pkg/ridb\_core.d.ts:227

Updates a document in the collection by its ID.

#### Parameters

##### document

`Partial`\<[`Doc`](../type-aliases/Doc.md)\<`T`\>\>

A partial document containing the fields to update.

#### Returns

`Promise`\<`void`\>

A promise that resolves when the update is complete.
