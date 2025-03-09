[**@trust0/ridb**](../README.md)

***

[@trust0/ridb](../README.md) / RIDB

# Class: RIDB\<T\>

Defined in: [src/index.ts:206](https://github.com/trust0-project/RIDB/blob/a6b0121185877080bad7ab1361054cc4574a1bd2/packages/ridb/src/index.ts#L206)

## Type Parameters

• **T** *extends* [`SchemaTypeRecord`](../type-aliases/SchemaTypeRecord.md) = [`SchemaTypeRecord`](../type-aliases/SchemaTypeRecord.md)

## Constructors

### new RIDB()

> **new RIDB**\<`T`\>(`options`): [`RIDB`](RIDB.md)\<`T`\>

Defined in: [src/index.ts:240](https://github.com/trust0-project/RIDB/blob/a6b0121185877080bad7ab1361054cc4574a1bd2/packages/ridb/src/index.ts#L240)

Creates an instance of RIDB.

#### Parameters

##### options

`DBOptions`\<`T`\>

#### Returns

[`RIDB`](RIDB.md)\<`T`\>

## Properties

### \_db

> `private` **\_db**: `undefined` \| [`Database`](Database.md)\<`T`\>

Defined in: [src/index.ts:207](https://github.com/trust0-project/RIDB/blob/a6b0121185877080bad7ab1361054cc4574a1bd2/packages/ridb/src/index.ts#L207)

***

### \_sessionId

> `private` **\_sessionId**: `undefined` \| `string`

Defined in: [src/index.ts:209](https://github.com/trust0-project/RIDB/blob/a6b0121185877080bad7ab1361054cc4574a1bd2/packages/ridb/src/index.ts#L209)

***

### \_worker

> `private` **\_worker**: `undefined` \| `SharedWorker`

Defined in: [src/index.ts:208](https://github.com/trust0-project/RIDB/blob/a6b0121185877080bad7ab1361054cc4574a1bd2/packages/ridb/src/index.ts#L208)

***

### options

> `private` **options**: `DBOptions`\<`T`\>

Defined in: [src/index.ts:240](https://github.com/trust0-project/RIDB/blob/a6b0121185877080bad7ab1361054cc4574a1bd2/packages/ridb/src/index.ts#L240)

***

### pendingRequests

> `private` **pendingRequests**: `PendingRequests`

Defined in: [src/index.ts:212](https://github.com/trust0-project/RIDB/blob/a6b0121185877080bad7ab1361054cc4574a1bd2/packages/ridb/src/index.ts#L212)

***

### started

> **started**: `boolean` = `false`

Defined in: [src/index.ts:210](https://github.com/trust0-project/RIDB/blob/a6b0121185877080bad7ab1361054cc4574a1bd2/packages/ridb/src/index.ts#L210)

## Accessors

### collections

#### Get Signature

> **get** **collections**(): \{ \[name in string \| number \| symbol\]: Collection\<Schema\<T\[name\]\>\> \}

Defined in: [src/index.ts:308](https://github.com/trust0-project/RIDB/blob/a6b0121185877080bad7ab1361054cc4574a1bd2/packages/ridb/src/index.ts#L308)

##### Returns

\{ \[name in string \| number \| symbol\]: Collection\<Schema\<T\[name\]\>\> \}

***

### db

#### Get Signature

> **get** `private` **db**(): [`Database`](Database.md)\<`T`\>

Defined in: [src/index.ts:253](https://github.com/trust0-project/RIDB/blob/a6b0121185877080bad7ab1361054cc4574a1bd2/packages/ridb/src/index.ts#L253)

Gets the database instance. Throws an error if the database has not been started.

##### Throws

Will throw an error if the database is not started.

##### Returns

[`Database`](Database.md)\<`T`\>

***

### dbCollections

#### Get Signature

> **get** `private` **dbCollections**(): \{ \[name in string \| number \| symbol\]: Collection\<Schema\<T\[name\]\>\> \}

Defined in: [src/index.ts:272](https://github.com/trust0-project/RIDB/blob/a6b0121185877080bad7ab1361054cc4574a1bd2/packages/ridb/src/index.ts#L272)

Gets the collections from the database.

##### Returns

\{ \[name in string \| number \| symbol\]: Collection\<Schema\<T\[name\]\>\> \}

The collections object.

***

### dbName

#### Get Signature

> **get** `private` **dbName**(): `string`

Defined in: [src/index.ts:214](https://github.com/trust0-project/RIDB/blob/a6b0121185877080bad7ab1361054cc4574a1bd2/packages/ridb/src/index.ts#L214)

##### Returns

`string`

***

### migrations

#### Get Signature

> **get** `private` **migrations**(): [`MigrationPathsForSchemas`](../type-aliases/MigrationPathsForSchemas.md)\<`T`\>

Defined in: [src/index.ts:222](https://github.com/trust0-project/RIDB/blob/a6b0121185877080bad7ab1361054cc4574a1bd2/packages/ridb/src/index.ts#L222)

##### Returns

[`MigrationPathsForSchemas`](../type-aliases/MigrationPathsForSchemas.md)\<`T`\>

***

### plugins

#### Get Signature

> **get** `private` **plugins**(): *typeof* [`BasePlugin`](BasePlugin.md)[]

Defined in: [src/index.ts:226](https://github.com/trust0-project/RIDB/blob/a6b0121185877080bad7ab1361054cc4574a1bd2/packages/ridb/src/index.ts#L226)

##### Returns

*typeof* [`BasePlugin`](BasePlugin.md)[]

***

### schemas

#### Get Signature

> **get** `private` **schemas**(): `T`

Defined in: [src/index.ts:218](https://github.com/trust0-project/RIDB/blob/a6b0121185877080bad7ab1361054cc4574a1bd2/packages/ridb/src/index.ts#L218)

##### Returns

`T`

***

### useWorker

#### Get Signature

> **get** **useWorker**(): `boolean`

Defined in: [src/index.ts:230](https://github.com/trust0-project/RIDB/blob/a6b0121185877080bad7ab1361054cc4574a1bd2/packages/ridb/src/index.ts#L230)

##### Returns

`boolean`

***

### worker

#### Get Signature

> **get** `private` **worker**(): `SharedWorker`

Defined in: [src/index.ts:260](https://github.com/trust0-project/RIDB/blob/a6b0121185877080bad7ab1361054cc4574a1bd2/packages/ridb/src/index.ts#L260)

##### Returns

`SharedWorker`

***

### workerCollections

#### Get Signature

> **get** `private` **workerCollections**(): \{ \[name in string \| number \| symbol\]: Collection\<Schema\<T\[name\]\>\> \}

Defined in: [src/index.ts:276](https://github.com/trust0-project/RIDB/blob/a6b0121185877080bad7ab1361054cc4574a1bd2/packages/ridb/src/index.ts#L276)

##### Returns

\{ \[name in string \| number \| symbol\]: Collection\<Schema\<T\[name\]\>\> \}

## Methods

### close()

> **close**(): `Promise`\<`void`\>

Defined in: [src/index.ts:395](https://github.com/trust0-project/RIDB/blob/a6b0121185877080bad7ab1361054cc4574a1bd2/packages/ridb/src/index.ts#L395)

#### Returns

`Promise`\<`void`\>

***

### createDatabase()

> `private` **createDatabase**(`options`?): `Promise`\<[`Database`](Database.md)\<`T`\>\>

Defined in: [src/index.ts:336](https://github.com/trust0-project/RIDB/blob/a6b0121185877080bad7ab1361054cc4574a1bd2/packages/ridb/src/index.ts#L336)

#### Parameters

##### options?

[`StartOptions`](../type-aliases/StartOptions.md)\<`T`\>

#### Returns

`Promise`\<[`Database`](Database.md)\<`T`\>\>

***

### createWorker()

> `private` **createWorker**(): `Promise`\<`SharedWorker`\>

Defined in: [src/index.ts:312](https://github.com/trust0-project/RIDB/blob/a6b0121185877080bad7ab1361054cc4574a1bd2/packages/ridb/src/index.ts#L312)

#### Returns

`Promise`\<`SharedWorker`\>

***

### getStorageType()

> `private` **getStorageType**\<`T`\>(`storageType`): *typeof* [`InMemory`](InMemory.md)

Defined in: [src/index.ts:242](https://github.com/trust0-project/RIDB/blob/a6b0121185877080bad7ab1361054cc4574a1bd2/packages/ridb/src/index.ts#L242)

#### Type Parameters

• **T** *extends* [`StorageType`](../enumerations/StorageType.md)

#### Parameters

##### storageType

`T`

#### Returns

*typeof* [`InMemory`](InMemory.md)

***

### handleWorkerMessage()

> `private` **handleWorkerMessage**(`event`): `Promise`\<`void`\>

Defined in: [src/index.ts:320](https://github.com/trust0-project/RIDB/blob/a6b0121185877080bad7ab1361054cc4574a1bd2/packages/ridb/src/index.ts#L320)

#### Parameters

##### event

`MessageEvent`

#### Returns

`Promise`\<`void`\>

***

### start()

> **start**(`options`?): `Promise`\<`void`\>

Defined in: [src/index.ts:368](https://github.com/trust0-project/RIDB/blob/a6b0121185877080bad7ab1361054cc4574a1bd2/packages/ridb/src/index.ts#L368)

Starts the database.

#### Parameters

##### options?

[`StartOptions`](../type-aliases/StartOptions.md)\<`T`\>

#### Returns

`Promise`\<`void`\>

A promise that resolves to the database instance.
