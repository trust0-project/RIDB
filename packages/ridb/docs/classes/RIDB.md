[**@trust0/ridb**](../README.md)

***

[@trust0/ridb](../README.md) / RIDB

# Class: RIDB\<T\>

Defined in: [ridb/src/index.ts:212](https://github.com/trust0-project/RIDB/blob/85db3a9d2dcaff16a7ea58fa276d6e3c4d9e6eea/packages/ridb/src/index.ts#L212)

## Type Parameters

• **T** *extends* [`SchemaTypeRecord`](../type-aliases/SchemaTypeRecord.md) = [`SchemaTypeRecord`](../type-aliases/SchemaTypeRecord.md)

## Constructors

### new RIDB()

> **new RIDB**\<`T`\>(`options`): [`RIDB`](RIDB.md)\<`T`\>

Defined in: [ridb/src/index.ts:250](https://github.com/trust0-project/RIDB/blob/85db3a9d2dcaff16a7ea58fa276d6e3c4d9e6eea/packages/ridb/src/index.ts#L250)

Creates an instance of RIDB.

#### Parameters

##### options

`DBOptions`\<`T`\>

#### Returns

[`RIDB`](RIDB.md)\<`T`\>

## Properties

### \_db

> `private` **\_db**: `undefined` \| [`Database`](Database.md)\<`T`\>

Defined in: [ridb/src/index.ts:213](https://github.com/trust0-project/RIDB/blob/85db3a9d2dcaff16a7ea58fa276d6e3c4d9e6eea/packages/ridb/src/index.ts#L213)

***

### \_sessionId

> `private` **\_sessionId**: `undefined` \| `string`

Defined in: [ridb/src/index.ts:215](https://github.com/trust0-project/RIDB/blob/85db3a9d2dcaff16a7ea58fa276d6e3c4d9e6eea/packages/ridb/src/index.ts#L215)

***

### \_worker

> `private` **\_worker**: `undefined` \| `SharedWorker`

Defined in: [ridb/src/index.ts:214](https://github.com/trust0-project/RIDB/blob/85db3a9d2dcaff16a7ea58fa276d6e3c4d9e6eea/packages/ridb/src/index.ts#L214)

***

### options

> `private` **options**: `DBOptions`\<`T`\>

Defined in: [ridb/src/index.ts:250](https://github.com/trust0-project/RIDB/blob/85db3a9d2dcaff16a7ea58fa276d6e3c4d9e6eea/packages/ridb/src/index.ts#L250)

***

### pendingRequests

> `private` **pendingRequests**: `PendingRequests`

Defined in: [ridb/src/index.ts:218](https://github.com/trust0-project/RIDB/blob/85db3a9d2dcaff16a7ea58fa276d6e3c4d9e6eea/packages/ridb/src/index.ts#L218)

***

### started

> **started**: `boolean` = `false`

Defined in: [ridb/src/index.ts:216](https://github.com/trust0-project/RIDB/blob/85db3a9d2dcaff16a7ea58fa276d6e3c4d9e6eea/packages/ridb/src/index.ts#L216)

## Accessors

### collections

#### Get Signature

> **get** **collections**(): \{ \[name in string \| number \| symbol\]: Collection\<Schema\<T\[name\]\>\> \}

Defined in: [ridb/src/index.ts:319](https://github.com/trust0-project/RIDB/blob/85db3a9d2dcaff16a7ea58fa276d6e3c4d9e6eea/packages/ridb/src/index.ts#L319)

##### Returns

\{ \[name in string \| number \| symbol\]: Collection\<Schema\<T\[name\]\>\> \}

***

### db

#### Get Signature

> **get** `private` **db**(): [`Database`](Database.md)\<`T`\>

Defined in: [ridb/src/index.ts:264](https://github.com/trust0-project/RIDB/blob/85db3a9d2dcaff16a7ea58fa276d6e3c4d9e6eea/packages/ridb/src/index.ts#L264)

Gets the database instance. Throws an error if the database has not been started.

##### Throws

Will throw an error if the database is not started.

##### Returns

[`Database`](Database.md)\<`T`\>

***

### dbCollections

#### Get Signature

> **get** `private` **dbCollections**(): \{ \[name in string \| number \| symbol\]: Collection\<Schema\<T\[name\]\>\> \}

Defined in: [ridb/src/index.ts:283](https://github.com/trust0-project/RIDB/blob/85db3a9d2dcaff16a7ea58fa276d6e3c4d9e6eea/packages/ridb/src/index.ts#L283)

Gets the collections from the database.

##### Returns

\{ \[name in string \| number \| symbol\]: Collection\<Schema\<T\[name\]\>\> \}

The collections object.

***

### dbName

#### Get Signature

> **get** `private` **dbName**(): `string`

Defined in: [ridb/src/index.ts:220](https://github.com/trust0-project/RIDB/blob/85db3a9d2dcaff16a7ea58fa276d6e3c4d9e6eea/packages/ridb/src/index.ts#L220)

##### Returns

`string`

***

### migrations

#### Get Signature

> **get** `private` **migrations**(): [`MigrationPathsForSchemas`](../type-aliases/MigrationPathsForSchemas.md)\<`T`\>

Defined in: [ridb/src/index.ts:228](https://github.com/trust0-project/RIDB/blob/85db3a9d2dcaff16a7ea58fa276d6e3c4d9e6eea/packages/ridb/src/index.ts#L228)

##### Returns

[`MigrationPathsForSchemas`](../type-aliases/MigrationPathsForSchemas.md)\<`T`\>

***

### plugins

#### Get Signature

> **get** `private` **plugins**(): *typeof* [`BasePlugin`](BasePlugin.md)[]

Defined in: [ridb/src/index.ts:232](https://github.com/trust0-project/RIDB/blob/85db3a9d2dcaff16a7ea58fa276d6e3c4d9e6eea/packages/ridb/src/index.ts#L232)

##### Returns

*typeof* [`BasePlugin`](BasePlugin.md)[]

***

### schemas

#### Get Signature

> **get** `private` **schemas**(): `T`

Defined in: [ridb/src/index.ts:224](https://github.com/trust0-project/RIDB/blob/85db3a9d2dcaff16a7ea58fa276d6e3c4d9e6eea/packages/ridb/src/index.ts#L224)

##### Returns

`T`

***

### useWorker

#### Get Signature

> **get** **useWorker**(): `boolean`

Defined in: [ridb/src/index.ts:236](https://github.com/trust0-project/RIDB/blob/85db3a9d2dcaff16a7ea58fa276d6e3c4d9e6eea/packages/ridb/src/index.ts#L236)

##### Returns

`boolean`

***

### worker

#### Get Signature

> **get** `private` **worker**(): `SharedWorker`

Defined in: [ridb/src/index.ts:271](https://github.com/trust0-project/RIDB/blob/85db3a9d2dcaff16a7ea58fa276d6e3c4d9e6eea/packages/ridb/src/index.ts#L271)

##### Returns

`SharedWorker`

***

### workerCollections

#### Get Signature

> **get** `private` **workerCollections**(): \{ \[name in string \| number \| symbol\]: Collection\<Schema\<T\[name\]\>\> \}

Defined in: [ridb/src/index.ts:287](https://github.com/trust0-project/RIDB/blob/85db3a9d2dcaff16a7ea58fa276d6e3c4d9e6eea/packages/ridb/src/index.ts#L287)

##### Returns

\{ \[name in string \| number \| symbol\]: Collection\<Schema\<T\[name\]\>\> \}

## Methods

### authenticate()

> **authenticate**(`password`): `Promise`\<`boolean`\>

Defined in: [ridb/src/index.ts:242](https://github.com/trust0-project/RIDB/blob/85db3a9d2dcaff16a7ea58fa276d6e3c4d9e6eea/packages/ridb/src/index.ts#L242)

#### Parameters

##### password

`string`

#### Returns

`Promise`\<`boolean`\>

***

### close()

> **close**(): `Promise`\<`void`\>

Defined in: [ridb/src/index.ts:407](https://github.com/trust0-project/RIDB/blob/85db3a9d2dcaff16a7ea58fa276d6e3c4d9e6eea/packages/ridb/src/index.ts#L407)

#### Returns

`Promise`\<`void`\>

***

### createDatabase()

> `private` **createDatabase**(`options`?): `Promise`\<[`Database`](Database.md)\<`T`\>\>

Defined in: [ridb/src/index.ts:348](https://github.com/trust0-project/RIDB/blob/85db3a9d2dcaff16a7ea58fa276d6e3c4d9e6eea/packages/ridb/src/index.ts#L348)

#### Parameters

##### options?

[`StartOptions`](../type-aliases/StartOptions.md)\<`T`\>

#### Returns

`Promise`\<[`Database`](Database.md)\<`T`\>\>

***

### createWorker()

> `private` **createWorker**(): `SharedWorker`

Defined in: [ridb/src/index.ts:323](https://github.com/trust0-project/RIDB/blob/85db3a9d2dcaff16a7ea58fa276d6e3c4d9e6eea/packages/ridb/src/index.ts#L323)

#### Returns

`SharedWorker`

***

### getStorageType()

> `private` **getStorageType**\<`T`\>(`storageType`): `Promise`\<*typeof* [`InMemory`](InMemory.md)\>

Defined in: [ridb/src/index.ts:252](https://github.com/trust0-project/RIDB/blob/85db3a9d2dcaff16a7ea58fa276d6e3c4d9e6eea/packages/ridb/src/index.ts#L252)

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

Defined in: [ridb/src/index.ts:330](https://github.com/trust0-project/RIDB/blob/85db3a9d2dcaff16a7ea58fa276d6e3c4d9e6eea/packages/ridb/src/index.ts#L330)

#### Parameters

##### event

`MessageEvent`

#### Returns

`Promise`\<`void`\>

***

### start()

> **start**(`options`?): `Promise`\<`void`\>

Defined in: [ridb/src/index.ts:380](https://github.com/trust0-project/RIDB/blob/85db3a9d2dcaff16a7ea58fa276d6e3c4d9e6eea/packages/ridb/src/index.ts#L380)

Starts the database.

#### Parameters

##### options?

[`StartOptions`](../type-aliases/StartOptions.md)\<`T`\>

#### Returns

`Promise`\<`void`\>

A promise that resolves to the database instance.
