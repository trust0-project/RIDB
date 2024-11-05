[**@trust0/ridb**](../../../README.md) • **Docs**

***

[@trust0/ridb](../../../README.md) / [RIDBTypes](../README.md) / InMemory

# Class: InMemory\<T\>

Represents an in-memory storage system extending the base storage functionality.

## Extends

- [`BaseStorage`](BaseStorage.md)\<`T`\>

## Type Parameters

• **T** *extends* [`SchemaType`](../type-aliases/SchemaType.md)

The schema type.

## Constructors

### new InMemory()

> **new InMemory**\<`T`\>(`name`, `schema_type`, `migration`): [`InMemory`](InMemory.md)\<`T`\>

Creates a new `BaseStorage` instance with the provided name and schema type.

#### Parameters

• **name**: `string`

The name of the storage.

• **schema\_type**: `T`

The schema type of the storage.

• **migration**: `MigrationPathsForSchema`\<`T`\>

#### Returns

[`InMemory`](InMemory.md)\<`T`\>

#### Inherited from

[`BaseStorage`](BaseStorage.md).[`constructor`](BaseStorage.md#constructors)

#### Defined in

pkg/ridb\_rust.d.ts:208

## Properties

### name

> `readonly` **name**: `string`

The name of the storage.

#### Inherited from

[`BaseStorage`](BaseStorage.md).[`name`](BaseStorage.md#name)

#### Defined in

pkg/ridb\_rust.d.ts:213

***

### schema

> `readonly` **schema**: [`Schema`](Schema.md)\<`T`\>

The schema associated with the storage.

#### Inherited from

[`BaseStorage`](BaseStorage.md).[`schema`](BaseStorage.md#schema)

#### Defined in

pkg/ridb\_rust.d.ts:218

## Methods

### close()

> **close**(): `Promise`\<`void`\>

Closes the storage.

#### Returns

`Promise`\<`void`\>

A promise that resolves when the storage is closed.

#### Inherited from

[`BaseStorage`](BaseStorage.md).[`close`](BaseStorage.md#close)

#### Defined in

pkg/ridb\_rust.d.ts:225

***

### count()

> **count**(`query`): `Promise`\<`number`\>

Counts the number of documents in the storage.

#### Parameters

• **query**: `QueryType`\<`T`\>

#### Returns

`Promise`\<`number`\>

A promise that resolves to the number of documents.

#### Inherited from

[`BaseStorage`](BaseStorage.md).[`count`](BaseStorage.md#count)

#### Defined in

pkg/ridb\_rust.d.ts:232

***

### find()

> **find**(`query`): `Promise`\<[`Doc`](../type-aliases/Doc.md)\<`T`\>[]\>

Queries the storage.

#### Parameters

• **query**: `QueryType`\<`T`\>

#### Returns

`Promise`\<[`Doc`](../type-aliases/Doc.md)\<`T`\>[]\>

A promise that resolves when the query is complete.

#### Inherited from

[`BaseStorage`](BaseStorage.md).[`find`](BaseStorage.md#find)

#### Defined in

pkg/ridb\_rust.d.ts:247

***

### findDocumentById()

> **findDocumentById**(`id`): `Promise`\<`null` \| [`Doc`](../type-aliases/Doc.md)\<`T`\>\>

Finds a document by its ID.

#### Parameters

• **id**: `string`

The ID of the document to find.

#### Returns

`Promise`\<`null` \| [`Doc`](../type-aliases/Doc.md)\<`T`\>\>

A promise that resolves to the found document or null.

#### Inherited from

[`BaseStorage`](BaseStorage.md).[`findDocumentById`](BaseStorage.md#finddocumentbyid)

#### Defined in

pkg/ridb\_rust.d.ts:240

***

### free()

> **free**(): `void`

Frees the resources used by the in-memory storage.

#### Returns

`void`

#### Overrides

[`BaseStorage`](BaseStorage.md).[`free`](BaseStorage.md#free)

#### Defined in

pkg/ridb\_rust.d.ts:96

***

### remove()

> **remove**(`id`): `Promise`\<`void`\>

Removes a document by its ID.

#### Parameters

• **id**: `string`

The ID of the document to remove.

#### Returns

`Promise`\<`void`\>

A promise that resolves when the document is removed.

#### Inherited from

[`BaseStorage`](BaseStorage.md).[`remove`](BaseStorage.md#remove)

#### Defined in

pkg/ridb\_rust.d.ts:255

***

### write()

> **write**(`op`): `Promise`\<[`Doc`](../type-aliases/Doc.md)\<`T`\>\>

Writes an operation to the storage.

#### Parameters

• **op**: [`Operation`](../type-aliases/Operation.md)\<`T`\>

The operation to write.

#### Returns

`Promise`\<[`Doc`](../type-aliases/Doc.md)\<`T`\>\>

A promise that resolves to the document written.

#### Inherited from

[`BaseStorage`](BaseStorage.md).[`write`](BaseStorage.md#write)

#### Defined in

pkg/ridb\_rust.d.ts:263
