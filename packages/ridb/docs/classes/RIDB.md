[**@trust0/ridb**](../README.md)

***

[@trust0/ridb](../README.md) / RIDB

# Class: RIDB\<T\>

Defined in: [index.ts:125](https://github.com/trust0-project/RIDB/blob/96bdd9e989f3b9d3bb9f1e9e2333148433a17232/packages/ridb/src/index.ts#L125)

Main RIDB class that provides database functionality with optional worker support

## Type Parameters

### T

`T` *extends* `SchemaTypeRecord` = `SchemaTypeRecord`

## Constructors

### Constructor

> **new RIDB**\<`T`\>(`options`): `RIDB`\<`T`\>

Defined in: [index.ts:131](https://github.com/trust0-project/RIDB/blob/96bdd9e989f3b9d3bb9f1e9e2333148433a17232/packages/ridb/src/index.ts#L131)

Creates a new RIDB instance

#### Parameters

##### options

[`DBOptions`](../type-aliases/DBOptions.md)\<`T`\>

#### Returns

`RIDB`\<`T`\>

## Properties

### adapter

> `private` **adapter**: [`RIDBAbstract`](../interfaces/RIDBAbstract.md)\<`T`\>

Defined in: [index.ts:126](https://github.com/trust0-project/RIDB/blob/96bdd9e989f3b9d3bb9f1e9e2333148433a17232/packages/ridb/src/index.ts#L126)

***

### options

> `private` **options**: [`DBOptions`](../type-aliases/DBOptions.md)\<`T`\>

Defined in: [index.ts:131](https://github.com/trust0-project/RIDB/blob/96bdd9e989f3b9d3bb9f1e9e2333148433a17232/packages/ridb/src/index.ts#L131)

## Accessors

### collections

#### Get Signature

> **get** **collections**(): \{ \[name in string \| number \| symbol\]: Collection\<Schema\<T\[name\]\>\> \}

Defined in: [index.ts:138](https://github.com/trust0-project/RIDB/blob/96bdd9e989f3b9d3bb9f1e9e2333148433a17232/packages/ridb/src/index.ts#L138)

Get the collections from the database

##### Returns

\{ \[name in string \| number \| symbol\]: Collection\<Schema\<T\[name\]\>\> \}

***

### started

#### Get Signature

> **get** **started**(): `boolean`

Defined in: [index.ts:159](https://github.com/trust0-project/RIDB/blob/96bdd9e989f3b9d3bb9f1e9e2333148433a17232/packages/ridb/src/index.ts#L159)

Whether the database has been started

##### Returns

`boolean`

## Methods

### close()

> **close**(): `Promise`\<`void`\>

Defined in: [index.ts:152](https://github.com/trust0-project/RIDB/blob/96bdd9e989f3b9d3bb9f1e9e2333148433a17232/packages/ridb/src/index.ts#L152)

Closes the database

#### Returns

`Promise`\<`void`\>

***

### start()

> **start**(`options?`): `Promise`\<`void`\>

Defined in: [index.ts:145](https://github.com/trust0-project/RIDB/blob/96bdd9e989f3b9d3bb9f1e9e2333148433a17232/packages/ridb/src/index.ts#L145)

Starts the database

#### Parameters

##### options?

[`StartOptions`](../type-aliases/StartOptions.md)\<`T`\>

#### Returns

`Promise`\<`void`\>
