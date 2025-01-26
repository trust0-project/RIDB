[**@trust0/ridb**](../README.md)

***

[@trust0/ridb](../README.md) / RIDB

# Class: RIDB\<T\>

Defined in: [ridb/src/index.ts:158](https://github.com/trust0-project/RIDB/blob/2c09198f6158019cec7175daeb2fb6bf65fb560a/packages/ridb/src/index.ts#L158)

## Type Parameters

• **T** *extends* [`SchemaTypeRecord`](../type-aliases/SchemaTypeRecord.md) = [`SchemaTypeRecord`](../type-aliases/SchemaTypeRecord.md)

## Constructors

### new RIDB()

> **new RIDB**\<`T`\>(`options`): [`RIDB`](RIDB.md)\<`T`\>

Defined in: [ridb/src/index.ts:170](https://github.com/trust0-project/RIDB/blob/2c09198f6158019cec7175daeb2fb6bf65fb560a/packages/ridb/src/index.ts#L170)

Creates an instance of RIDB.

#### Parameters

##### options

`object` & [`MigrationsParameter`](../type-aliases/MigrationsParameter.md)\<`T`\>

#### Returns

[`RIDB`](RIDB.md)\<`T`\>

## Properties

### \_db

> `private` **\_db**: `undefined` \| [`Database`](Database.md)\<`T`\>

Defined in: [ridb/src/index.ts:163](https://github.com/trust0-project/RIDB/blob/2c09198f6158019cec7175daeb2fb6bf65fb560a/packages/ridb/src/index.ts#L163)

***

### dbName

> `private` **dbName**: `string`

Defined in: [ridb/src/index.ts:164](https://github.com/trust0-project/RIDB/blob/2c09198f6158019cec7175daeb2fb6bf65fb560a/packages/ridb/src/index.ts#L164)

***

### migrations

> `private` **migrations**: [`MigrationPathsForSchemas`](../type-aliases/MigrationPathsForSchemas.md)\<`T`\>

Defined in: [ridb/src/index.ts:161](https://github.com/trust0-project/RIDB/blob/2c09198f6158019cec7175daeb2fb6bf65fb560a/packages/ridb/src/index.ts#L161)

***

### plugins

> `private` **plugins**: *typeof* [`BasePlugin`](BasePlugin.md)[] = `[]`

Defined in: [ridb/src/index.ts:162](https://github.com/trust0-project/RIDB/blob/2c09198f6158019cec7175daeb2fb6bf65fb560a/packages/ridb/src/index.ts#L162)

***

### schemas

> `private` **schemas**: `T`

Defined in: [ridb/src/index.ts:160](https://github.com/trust0-project/RIDB/blob/2c09198f6158019cec7175daeb2fb6bf65fb560a/packages/ridb/src/index.ts#L160)

## Accessors

### collections

#### Get Signature

> **get** **collections**(): \{ \[name in string \| number \| symbol\]: Collection\<Schema\<T\[name\]\>\> \}

Defined in: [ridb/src/index.ts:218](https://github.com/trust0-project/RIDB/blob/2c09198f6158019cec7175daeb2fb6bf65fb560a/packages/ridb/src/index.ts#L218)

Gets the collections from the database.

##### Returns

\{ \[name in string \| number \| symbol\]: Collection\<Schema\<T\[name\]\>\> \}

The collections object.

***

### db

#### Get Signature

> **get** `private` **db**(): [`Database`](Database.md)\<`T`\>

Defined in: [ridb/src/index.ts:203](https://github.com/trust0-project/RIDB/blob/2c09198f6158019cec7175daeb2fb6bf65fb560a/packages/ridb/src/index.ts#L203)

Gets the database instance. Throws an error if the database has not been started.

##### Throws

Will throw an error if the database is not started.

##### Returns

[`Database`](Database.md)\<`T`\>

***

### started

#### Get Signature

> **get** **started**(): `boolean`

Defined in: [ridb/src/index.ts:210](https://github.com/trust0-project/RIDB/blob/2c09198f6158019cec7175daeb2fb6bf65fb560a/packages/ridb/src/index.ts#L210)

##### Returns

`boolean`

## Methods

### close()

> **close**(): `Promise`\<`void`\>

Defined in: [ridb/src/index.ts:279](https://github.com/trust0-project/RIDB/blob/2c09198f6158019cec7175daeb2fb6bf65fb560a/packages/ridb/src/index.ts#L279)

#### Returns

`Promise`\<`void`\>

***

### getStorageType()

> `private` **getStorageType**\<`T`\>(`storageType`): *typeof* [`InMemory`](InMemory.md)

Defined in: [ridb/src/index.ts:191](https://github.com/trust0-project/RIDB/blob/2c09198f6158019cec7175daeb2fb6bf65fb560a/packages/ridb/src/index.ts#L191)

#### Type Parameters

• **T** *extends* [`StorageType`](../enumerations/StorageType.md)

#### Parameters

##### storageType

`T`

#### Returns

*typeof* [`InMemory`](InMemory.md)

***

### start()

> **start**(`options`?): `Promise`\<[`Database`](Database.md)\<`T`\>\>

Defined in: [ridb/src/index.ts:241](https://github.com/trust0-project/RIDB/blob/2c09198f6158019cec7175daeb2fb6bf65fb560a/packages/ridb/src/index.ts#L241)

Starts the database.

#### Parameters

##### options?

[`StartOptions`](../type-aliases/StartOptions.md)\<`T`\>

#### Returns

`Promise`\<[`Database`](Database.md)\<`T`\>\>

A promise that resolves to the database instance.

***

### load()

> `private` `static` **load**(): `Promise`\<`__module`\>

Defined in: [ridb/src/index.ts:227](https://github.com/trust0-project/RIDB/blob/2c09198f6158019cec7175daeb2fb6bf65fb560a/packages/ridb/src/index.ts#L227)

Loads the RIDB Rust module.

#### Returns

`Promise`\<`__module`\>

A promise that resolves to the RIDB Rust module.
