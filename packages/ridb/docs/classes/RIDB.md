[**@trust0/ridb**](../README.md)

***

[@trust0/ridb](../README.md) / RIDB

# Class: RIDB\<T\>

Defined in: [index.ts:167](https://github.com/trust0-project/RIDB/blob/40b5c2c88b47dd5db201bd993b2e70350246bff3/packages/ridb/src/index.ts#L167)

## Type Parameters

### T

`T` *extends* `SchemaTypeRecord` = `SchemaTypeRecord`

## Constructors

### Constructor

> **new RIDB**\<`T`\>(`options`): `RIDB`\<`T`\>

Defined in: [index.ts:205](https://github.com/trust0-project/RIDB/blob/40b5c2c88b47dd5db201bd993b2e70350246bff3/packages/ridb/src/index.ts#L205)

Creates an instance of RIDB.

#### Parameters

##### options

[`DBOptions`](../type-aliases/DBOptions.md)\<`T`\>

#### Returns

`RIDB`\<`T`\>

## Properties

### \_db

> `private` **\_db**: `undefined` \| `Database`\<`T`\>

Defined in: [index.ts:168](https://github.com/trust0-project/RIDB/blob/40b5c2c88b47dd5db201bd993b2e70350246bff3/packages/ridb/src/index.ts#L168)

***

### \_sessionId

> `private` **\_sessionId**: `undefined` \| `string`

Defined in: [index.ts:170](https://github.com/trust0-project/RIDB/blob/40b5c2c88b47dd5db201bd993b2e70350246bff3/packages/ridb/src/index.ts#L170)

***

### \_worker

> `private` **\_worker**: `undefined` \| `SharedWorker`

Defined in: [index.ts:169](https://github.com/trust0-project/RIDB/blob/40b5c2c88b47dd5db201bd993b2e70350246bff3/packages/ridb/src/index.ts#L169)

***

### options

> `private` **options**: [`DBOptions`](../type-aliases/DBOptions.md)\<`T`\>

Defined in: [index.ts:205](https://github.com/trust0-project/RIDB/blob/40b5c2c88b47dd5db201bd993b2e70350246bff3/packages/ridb/src/index.ts#L205)

***

### pendingRequests

> `private` **pendingRequests**: [`PendingRequests`](../type-aliases/PendingRequests.md)

Defined in: [index.ts:173](https://github.com/trust0-project/RIDB/blob/40b5c2c88b47dd5db201bd993b2e70350246bff3/packages/ridb/src/index.ts#L173)

***

### started

> **started**: `boolean` = `false`

Defined in: [index.ts:171](https://github.com/trust0-project/RIDB/blob/40b5c2c88b47dd5db201bd993b2e70350246bff3/packages/ridb/src/index.ts#L171)

## Accessors

### collections

#### Get Signature

> **get** **collections**(): \{ \[name in string \| number \| symbol\]: Collection\<Schema\<T\[name\]\>\> \}

Defined in: [index.ts:274](https://github.com/trust0-project/RIDB/blob/40b5c2c88b47dd5db201bd993b2e70350246bff3/packages/ridb/src/index.ts#L274)

##### Returns

\{ \[name in string \| number \| symbol\]: Collection\<Schema\<T\[name\]\>\> \}

***

### db

#### Get Signature

> **get** `private` **db**(): `Database`\<`T`\>

Defined in: [index.ts:219](https://github.com/trust0-project/RIDB/blob/40b5c2c88b47dd5db201bd993b2e70350246bff3/packages/ridb/src/index.ts#L219)

Gets the database instance. Throws an error if the database has not been started.

##### Throws

Will throw an error if the database is not started.

##### Returns

`Database`\<`T`\>

***

### dbCollections

#### Get Signature

> **get** `private` **dbCollections**(): \{ \[name in string \| number \| symbol\]: Collection\<Schema\<T\[name\]\>\> \}

Defined in: [index.ts:238](https://github.com/trust0-project/RIDB/blob/40b5c2c88b47dd5db201bd993b2e70350246bff3/packages/ridb/src/index.ts#L238)

Gets the collections from the database.

##### Returns

\{ \[name in string \| number \| symbol\]: Collection\<Schema\<T\[name\]\>\> \}

The collections object.

***

### dbName

#### Get Signature

> **get** `private` **dbName**(): `string`

Defined in: [index.ts:175](https://github.com/trust0-project/RIDB/blob/40b5c2c88b47dd5db201bd993b2e70350246bff3/packages/ridb/src/index.ts#L175)

##### Returns

`string`

***

### migrations

#### Get Signature

> **get** `private` **migrations**(): `MigrationPathsForSchemas`\<`T`\>

Defined in: [index.ts:183](https://github.com/trust0-project/RIDB/blob/40b5c2c88b47dd5db201bd993b2e70350246bff3/packages/ridb/src/index.ts#L183)

##### Returns

`MigrationPathsForSchemas`\<`T`\>

***

### plugins

#### Get Signature

> **get** `private` **plugins**(): *typeof* `BasePlugin`[]

Defined in: [index.ts:187](https://github.com/trust0-project/RIDB/blob/40b5c2c88b47dd5db201bd993b2e70350246bff3/packages/ridb/src/index.ts#L187)

##### Returns

*typeof* `BasePlugin`[]

***

### schemas

#### Get Signature

> **get** `private` **schemas**(): `T`

Defined in: [index.ts:179](https://github.com/trust0-project/RIDB/blob/40b5c2c88b47dd5db201bd993b2e70350246bff3/packages/ridb/src/index.ts#L179)

##### Returns

`T`

***

### useWorker

#### Get Signature

> **get** **useWorker**(): `boolean`

Defined in: [index.ts:191](https://github.com/trust0-project/RIDB/blob/40b5c2c88b47dd5db201bd993b2e70350246bff3/packages/ridb/src/index.ts#L191)

##### Returns

`boolean`

***

### worker

#### Get Signature

> **get** `private` **worker**(): `SharedWorker`

Defined in: [index.ts:226](https://github.com/trust0-project/RIDB/blob/40b5c2c88b47dd5db201bd993b2e70350246bff3/packages/ridb/src/index.ts#L226)

##### Returns

`SharedWorker`

***

### workerCollections

#### Get Signature

> **get** `private` **workerCollections**(): \{ \[name in string \| number \| symbol\]: Collection\<Schema\<T\[name\]\>\> \}

Defined in: [index.ts:242](https://github.com/trust0-project/RIDB/blob/40b5c2c88b47dd5db201bd993b2e70350246bff3/packages/ridb/src/index.ts#L242)

##### Returns

\{ \[name in string \| number \| symbol\]: Collection\<Schema\<T\[name\]\>\> \}

## Methods

### authenticate()

> **authenticate**(`password`): `Promise`\<`boolean`\>

Defined in: [index.ts:197](https://github.com/trust0-project/RIDB/blob/40b5c2c88b47dd5db201bd993b2e70350246bff3/packages/ridb/src/index.ts#L197)

#### Parameters

##### password

`string`

#### Returns

`Promise`\<`boolean`\>

***

### close()

> **close**(): `Promise`\<`void`\>

Defined in: [index.ts:368](https://github.com/trust0-project/RIDB/blob/40b5c2c88b47dd5db201bd993b2e70350246bff3/packages/ridb/src/index.ts#L368)

#### Returns

`Promise`\<`void`\>

***

### createDatabase()

> `private` **createDatabase**(`options?`): `Promise`\<`Database`\<`T`\>\>

Defined in: [index.ts:308](https://github.com/trust0-project/RIDB/blob/40b5c2c88b47dd5db201bd993b2e70350246bff3/packages/ridb/src/index.ts#L308)

#### Parameters

##### options?

[`StartOptions`](../type-aliases/StartOptions.md)\<`T`\>

#### Returns

`Promise`\<`Database`\<`T`\>\>

***

### createWorker()

> `private` **createWorker**(): `SharedWorker`

Defined in: [index.ts:278](https://github.com/trust0-project/RIDB/blob/40b5c2c88b47dd5db201bd993b2e70350246bff3/packages/ridb/src/index.ts#L278)

#### Returns

`SharedWorker`

***

### getStorageType()

> `private` **getStorageType**\<`T`\>(`storageType`): `Promise`\<*typeof* `InMemory`\>

Defined in: [index.ts:207](https://github.com/trust0-project/RIDB/blob/40b5c2c88b47dd5db201bd993b2e70350246bff3/packages/ridb/src/index.ts#L207)

#### Type Parameters

##### T

`T` *extends* [`StorageType`](../enumerations/StorageType.md)

#### Parameters

##### storageType

`T`

#### Returns

`Promise`\<*typeof* `InMemory`\>

***

### handleWorkerMessage()

> `private` **handleWorkerMessage**(`event`): `Promise`\<`void`\>

Defined in: [index.ts:290](https://github.com/trust0-project/RIDB/blob/40b5c2c88b47dd5db201bd993b2e70350246bff3/packages/ridb/src/index.ts#L290)

#### Parameters

##### event

`MessageEvent`

#### Returns

`Promise`\<`void`\>

***

### start()

> **start**(`options?`): `Promise`\<`void`\>

Defined in: [index.ts:341](https://github.com/trust0-project/RIDB/blob/40b5c2c88b47dd5db201bd993b2e70350246bff3/packages/ridb/src/index.ts#L341)

Starts the database.

#### Parameters

##### options?

[`StartOptions`](../type-aliases/StartOptions.md)\<`T`\>

#### Returns

`Promise`\<`void`\>

A promise that resolves to the database instance.
