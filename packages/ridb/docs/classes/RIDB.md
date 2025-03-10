[**@trust0/ridb**](../README.md)

***

[@trust0/ridb](../README.md) / RIDB

# Class: RIDB\<T\>

Defined in: [ridb/src/index.ts:203](https://github.com/trust0-project/RIDB/blob/c89348396189a4c9d82fb81a853b3b41c64e5199/packages/ridb/src/index.ts#L203)

## Type Parameters

• **T** *extends* [`SchemaTypeRecord`](../type-aliases/SchemaTypeRecord.md) = [`SchemaTypeRecord`](../type-aliases/SchemaTypeRecord.md)

## Constructors

### new RIDB()

> **new RIDB**\<`T`\>(`options`): [`RIDB`](RIDB.md)\<`T`\>

Defined in: [ridb/src/index.ts:241](https://github.com/trust0-project/RIDB/blob/c89348396189a4c9d82fb81a853b3b41c64e5199/packages/ridb/src/index.ts#L241)

Creates an instance of RIDB.

#### Parameters

##### options

`DBOptions`\<`T`\>

#### Returns

[`RIDB`](RIDB.md)\<`T`\>

## Properties

### \_db

> `private` **\_db**: `undefined` \| [`Database`](Database.md)\<`T`\>

Defined in: [ridb/src/index.ts:204](https://github.com/trust0-project/RIDB/blob/c89348396189a4c9d82fb81a853b3b41c64e5199/packages/ridb/src/index.ts#L204)

***

### \_sessionId

> `private` **\_sessionId**: `undefined` \| `string`

Defined in: [ridb/src/index.ts:206](https://github.com/trust0-project/RIDB/blob/c89348396189a4c9d82fb81a853b3b41c64e5199/packages/ridb/src/index.ts#L206)

***

### \_worker

> `private` **\_worker**: `undefined` \| `SharedWorker`

Defined in: [ridb/src/index.ts:205](https://github.com/trust0-project/RIDB/blob/c89348396189a4c9d82fb81a853b3b41c64e5199/packages/ridb/src/index.ts#L205)

***

### options

> `private` **options**: `DBOptions`\<`T`\>

Defined in: [ridb/src/index.ts:241](https://github.com/trust0-project/RIDB/blob/c89348396189a4c9d82fb81a853b3b41c64e5199/packages/ridb/src/index.ts#L241)

***

### pendingRequests

> `private` **pendingRequests**: `PendingRequests`

Defined in: [ridb/src/index.ts:209](https://github.com/trust0-project/RIDB/blob/c89348396189a4c9d82fb81a853b3b41c64e5199/packages/ridb/src/index.ts#L209)

***

### started

> **started**: `boolean` = `false`

Defined in: [ridb/src/index.ts:207](https://github.com/trust0-project/RIDB/blob/c89348396189a4c9d82fb81a853b3b41c64e5199/packages/ridb/src/index.ts#L207)

## Accessors

### collections

#### Get Signature

> **get** **collections**(): \{ \[name in string \| number \| symbol\]: Collection\<Schema\<T\[name\]\>\> \}

Defined in: [ridb/src/index.ts:310](https://github.com/trust0-project/RIDB/blob/c89348396189a4c9d82fb81a853b3b41c64e5199/packages/ridb/src/index.ts#L310)

##### Returns

\{ \[name in string \| number \| symbol\]: Collection\<Schema\<T\[name\]\>\> \}

***

### db

#### Get Signature

> **get** `private` **db**(): [`Database`](Database.md)\<`T`\>

Defined in: [ridb/src/index.ts:255](https://github.com/trust0-project/RIDB/blob/c89348396189a4c9d82fb81a853b3b41c64e5199/packages/ridb/src/index.ts#L255)

Gets the database instance. Throws an error if the database has not been started.

##### Throws

Will throw an error if the database is not started.

##### Returns

[`Database`](Database.md)\<`T`\>

***

### dbCollections

#### Get Signature

> **get** `private` **dbCollections**(): \{ \[name in string \| number \| symbol\]: Collection\<Schema\<T\[name\]\>\> \}

Defined in: [ridb/src/index.ts:274](https://github.com/trust0-project/RIDB/blob/c89348396189a4c9d82fb81a853b3b41c64e5199/packages/ridb/src/index.ts#L274)

Gets the collections from the database.

##### Returns

\{ \[name in string \| number \| symbol\]: Collection\<Schema\<T\[name\]\>\> \}

The collections object.

***

### dbName

#### Get Signature

> **get** `private` **dbName**(): `string`

Defined in: [ridb/src/index.ts:211](https://github.com/trust0-project/RIDB/blob/c89348396189a4c9d82fb81a853b3b41c64e5199/packages/ridb/src/index.ts#L211)

##### Returns

`string`

***

### migrations

#### Get Signature

> **get** `private` **migrations**(): [`MigrationPathsForSchemas`](../type-aliases/MigrationPathsForSchemas.md)\<`T`\>

Defined in: [ridb/src/index.ts:219](https://github.com/trust0-project/RIDB/blob/c89348396189a4c9d82fb81a853b3b41c64e5199/packages/ridb/src/index.ts#L219)

##### Returns

[`MigrationPathsForSchemas`](../type-aliases/MigrationPathsForSchemas.md)\<`T`\>

***

### plugins

#### Get Signature

> **get** `private` **plugins**(): *typeof* [`BasePlugin`](BasePlugin.md)[]

Defined in: [ridb/src/index.ts:223](https://github.com/trust0-project/RIDB/blob/c89348396189a4c9d82fb81a853b3b41c64e5199/packages/ridb/src/index.ts#L223)

##### Returns

*typeof* [`BasePlugin`](BasePlugin.md)[]

***

### schemas

#### Get Signature

> **get** `private` **schemas**(): `T`

Defined in: [ridb/src/index.ts:215](https://github.com/trust0-project/RIDB/blob/c89348396189a4c9d82fb81a853b3b41c64e5199/packages/ridb/src/index.ts#L215)

##### Returns

`T`

***

### useWorker

#### Get Signature

> **get** **useWorker**(): `boolean`

Defined in: [ridb/src/index.ts:227](https://github.com/trust0-project/RIDB/blob/c89348396189a4c9d82fb81a853b3b41c64e5199/packages/ridb/src/index.ts#L227)

##### Returns

`boolean`

***

### worker

#### Get Signature

> **get** `private` **worker**(): `SharedWorker`

Defined in: [ridb/src/index.ts:262](https://github.com/trust0-project/RIDB/blob/c89348396189a4c9d82fb81a853b3b41c64e5199/packages/ridb/src/index.ts#L262)

##### Returns

`SharedWorker`

***

### workerCollections

#### Get Signature

> **get** `private` **workerCollections**(): \{ \[name in string \| number \| symbol\]: Collection\<Schema\<T\[name\]\>\> \}

Defined in: [ridb/src/index.ts:278](https://github.com/trust0-project/RIDB/blob/c89348396189a4c9d82fb81a853b3b41c64e5199/packages/ridb/src/index.ts#L278)

##### Returns

\{ \[name in string \| number \| symbol\]: Collection\<Schema\<T\[name\]\>\> \}

## Methods

### authenticate()

> **authenticate**(`password`): `Promise`\<`boolean`\>

Defined in: [ridb/src/index.ts:233](https://github.com/trust0-project/RIDB/blob/c89348396189a4c9d82fb81a853b3b41c64e5199/packages/ridb/src/index.ts#L233)

#### Parameters

##### password

`string`

#### Returns

`Promise`\<`boolean`\>

***

### close()

> **close**(): `Promise`\<`void`\>

Defined in: [ridb/src/index.ts:398](https://github.com/trust0-project/RIDB/blob/c89348396189a4c9d82fb81a853b3b41c64e5199/packages/ridb/src/index.ts#L398)

#### Returns

`Promise`\<`void`\>

***

### createDatabase()

> `private` **createDatabase**(`options`?): `Promise`\<[`Database`](Database.md)\<`T`\>\>

Defined in: [ridb/src/index.ts:339](https://github.com/trust0-project/RIDB/blob/c89348396189a4c9d82fb81a853b3b41c64e5199/packages/ridb/src/index.ts#L339)

#### Parameters

##### options?

[`StartOptions`](../type-aliases/StartOptions.md)\<`T`\>

#### Returns

`Promise`\<[`Database`](Database.md)\<`T`\>\>

***

### createWorker()

> `private` **createWorker**(): `SharedWorker`

Defined in: [ridb/src/index.ts:314](https://github.com/trust0-project/RIDB/blob/c89348396189a4c9d82fb81a853b3b41c64e5199/packages/ridb/src/index.ts#L314)

#### Returns

`SharedWorker`

***

### getStorageType()

> `private` **getStorageType**\<`T`\>(`storageType`): `Promise`\<*typeof* [`InMemory`](InMemory.md)\>

Defined in: [ridb/src/index.ts:243](https://github.com/trust0-project/RIDB/blob/c89348396189a4c9d82fb81a853b3b41c64e5199/packages/ridb/src/index.ts#L243)

#### Type Parameters

• **T** *extends* [`StorageType`](../enumerations/StorageType.md)

#### Parameters

##### storageType

`T`

#### Returns

`Promise`\<*typeof* [`InMemory`](InMemory.md)\>

***

### handleWorkerMessage()

> `private` **handleWorkerMessage**(`event`): `Promise`\<`void`\>

Defined in: [ridb/src/index.ts:321](https://github.com/trust0-project/RIDB/blob/c89348396189a4c9d82fb81a853b3b41c64e5199/packages/ridb/src/index.ts#L321)

#### Parameters

##### event

`MessageEvent`

#### Returns

`Promise`\<`void`\>

***

### start()

> **start**(`options`?): `Promise`\<`void`\>

Defined in: [ridb/src/index.ts:371](https://github.com/trust0-project/RIDB/blob/c89348396189a4c9d82fb81a853b3b41c64e5199/packages/ridb/src/index.ts#L371)

Starts the database.

#### Parameters

##### options?

[`StartOptions`](../type-aliases/StartOptions.md)\<`T`\>

#### Returns

`Promise`\<`void`\>

A promise that resolves to the database instance.
