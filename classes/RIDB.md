[**@elribonazo/ridb**](../README.md) • **Docs**

***

[@elribonazo/ridb](../README.md) / RIDB

# Class: RIDB\<T\>

## Type Parameters

• **T** *extends* [`SchemaTypeRecord`](../namespaces/RIDBTypes/type-aliases/SchemaTypeRecord.md)

## Constructors

### new RIDB()

> **new RIDB**\<`T`\>(`schemas`): [`RIDB`](RIDB.md)\<`T`\>

#### Parameters

• **schemas**: `T`

#### Returns

[`RIDB`](RIDB.md)\<`T`\>

#### Defined in

[ts/src/index.ts:170](https://github.com/elribonazo/RIDB/blob/8e03a4ac74d394f9b7e128cbe2d73d450283b246/ts/src/index.ts#L170)

## Accessors

### collections

> `get` **collections**(): \{ \[name in string \| number \| symbol\]: Collection\<Schema\<T\[name\]\>\> \}

#### Returns

\{ \[name in string \| number \| symbol\]: Collection\<Schema\<T\[name\]\>\> \}

#### Defined in

[ts/src/index.ts:181](https://github.com/elribonazo/RIDB/blob/8e03a4ac74d394f9b7e128cbe2d73d450283b246/ts/src/index.ts#L181)

## Methods

### start()

> **start**(`storageType`?): `Promise`\<[`Database`](../namespaces/RIDBTypes/classes/Database.md)\<`T`\>\>

#### Parameters

• **storageType?**: *typeof* [`BaseStorage`](../namespaces/RIDBTypes/classes/BaseStorage.md)

#### Returns

`Promise`\<[`Database`](../namespaces/RIDBTypes/classes/Database.md)\<`T`\>\>

#### Defined in

[ts/src/index.ts:194](https://github.com/elribonazo/RIDB/blob/8e03a4ac74d394f9b7e128cbe2d73d450283b246/ts/src/index.ts#L194)
