[**@elribonazo/ridb**](../../../README.md) • **Docs**

***

[@elribonazo/ridb](../../../README.md) / [RIDBTypes](../README.md) / Database

# Class: Database\<T\>

Represents a database containing collections of documents.

## Type Parameters

• **T** *extends* [`SchemaTypeRecord`](../type-aliases/SchemaTypeRecord.md)

A record of schema types.

## Constructors

### new Database()

> **new Database**\<`T`\>(): [`Database`](Database.md)\<`T`\>

#### Returns

[`Database`](Database.md)\<`T`\>

## Properties

### collections

> `readonly` **collections**: \{ \[name in string \| number \| symbol\]: Collection\<Schema\<T\[name\]\>\> \}

The collections in the database.

This is a read-only property where the key is the name of the collection and the value is a `Collection` instance.

#### Defined in

pkg/ridb\_rust.d.ts:484

## Methods

### create()

> `static` **create**\<`TS`\>(`schemas`, `storage`): `Promise`\<[`Database`](Database.md)\<`TS`\>\>

Creates a new `Database` instance with the provided schemas and storage module.

#### Type Parameters

• **TS** *extends* [`SchemaTypeRecord`](../type-aliases/SchemaTypeRecord.md)

A record of schema types.

#### Parameters

• **schemas**: `TS`

The schemas to use for the collections.

• **storage**: [`StorageModule`](../type-aliases/StorageModule.md)

The storage module to use.

#### Returns

`Promise`\<[`Database`](Database.md)\<`TS`\>\>

A promise that resolves to the created `Database` instance.

#### Defined in

pkg/ridb\_rust.d.ts:474
