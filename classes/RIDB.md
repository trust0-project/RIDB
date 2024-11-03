[**@elribonazo/ridb**](../README.md) • **Docs**

***

[@elribonazo/ridb](../README.md) / RIDB

# Class: RIDB\<T\>

Represents a RIDB (Rust IndexedDB) instance.

## Type Parameters

• **T** *extends* [`SchemaTypeRecord`](../namespaces/RIDBTypes/type-aliases/SchemaTypeRecord.md)

The type of the schema record.

## Constructors

### new RIDB()

> **new RIDB**\<`T`\>(`schemas`, `plugins`?): [`RIDB`](RIDB.md)\<`T`\>

Creates an instance of RIDB.

#### Parameters

• **schemas**: `T`

The schema definitions for the database.

• **plugins?**: *typeof* `BasePlugin`[] = `...`

The plugins to use.

#### Returns

[`RIDB`](RIDB.md)\<`T`\>

#### Defined in

[ts/src/index.ts:163](https://github.com/elribonazo/RIDB/blob/5c94388e8704c364967b97b7becd481bd32e5b69/ts/src/index.ts#L163)

## Accessors

### collections

#### Get Signature

> **get** **collections**(): \{ \[name in string \| number \| symbol\]: Collection\<Schema\<T\[name\]\>\> \}

Gets the collections from the database.

##### Returns

\{ \[name in string \| number \| symbol\]: Collection\<Schema\<T\[name\]\>\> \}

The collections object.

#### Defined in

[ts/src/index.ts:187](https://github.com/elribonazo/RIDB/blob/5c94388e8704c364967b97b7becd481bd32e5b69/ts/src/index.ts#L187)

## Methods

### start()

> **start**(`storageType`?): `Promise`\<[`Database`](../namespaces/RIDBTypes/classes/Database.md)\<`T`\>\>

Starts the database.

#### Parameters

• **storageType?**: *typeof* [`BaseStorage`](../namespaces/RIDBTypes/classes/BaseStorage.md)

The storage type to use.

#### Returns

`Promise`\<[`Database`](../namespaces/RIDBTypes/classes/Database.md)\<`T`\>\>

A promise that resolves to the database instance.

#### Defined in

[ts/src/index.ts:228](https://github.com/elribonazo/RIDB/blob/5c94388e8704c364967b97b7becd481bd32e5b69/ts/src/index.ts#L228)
