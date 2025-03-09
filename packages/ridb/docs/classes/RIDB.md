[**@trust0/ridb**](../README.md)

***

[@trust0/ridb](../README.md) / RIDB

# Class: RIDB\<T\>

Defined in: [ridb/src/index.ts:208](https://github.com/trust0-project/RIDB/blob/934a985e11b05b646d6653d7f479a4358cad8c68/packages/ridb/src/index.ts#L208)

## Type Parameters

• **T** *extends* [`SchemaTypeRecord`](../type-aliases/SchemaTypeRecord.md) = [`SchemaTypeRecord`](../type-aliases/SchemaTypeRecord.md)

## Constructors

### new RIDB()

> **new RIDB**\<`T`\>(`options`): [`RIDB`](RIDB.md)\<`T`\>

Defined in: [ridb/src/index.ts:246](https://github.com/trust0-project/RIDB/blob/934a985e11b05b646d6653d7f479a4358cad8c68/packages/ridb/src/index.ts#L246)

Creates an instance of RIDB.

#### Parameters

##### options

`DBOptions`\<`T`\>

#### Returns

[`RIDB`](RIDB.md)\<`T`\>

## Properties

### \_db

> `private` **\_db**: `undefined` \| [`Database`](Database.md)\<`T`\>

Defined in: [ridb/src/index.ts:209](https://github.com/trust0-project/RIDB/blob/934a985e11b05b646d6653d7f479a4358cad8c68/packages/ridb/src/index.ts#L209)

***

### \_sessionId

> `private` **\_sessionId**: `undefined` \| `string`

Defined in: [ridb/src/index.ts:211](https://github.com/trust0-project/RIDB/blob/934a985e11b05b646d6653d7f479a4358cad8c68/packages/ridb/src/index.ts#L211)

***

### \_worker

> `private` **\_worker**: `undefined` \| `SharedWorker`

Defined in: [ridb/src/index.ts:210](https://github.com/trust0-project/RIDB/blob/934a985e11b05b646d6653d7f479a4358cad8c68/packages/ridb/src/index.ts#L210)

***

### options

> `private` **options**: `DBOptions`\<`T`\>

Defined in: [ridb/src/index.ts:246](https://github.com/trust0-project/RIDB/blob/934a985e11b05b646d6653d7f479a4358cad8c68/packages/ridb/src/index.ts#L246)

***

### pendingRequests

> `private` **pendingRequests**: `PendingRequests`

Defined in: [ridb/src/index.ts:214](https://github.com/trust0-project/RIDB/blob/934a985e11b05b646d6653d7f479a4358cad8c68/packages/ridb/src/index.ts#L214)

***

### started

> **started**: `boolean` = `false`

Defined in: [ridb/src/index.ts:212](https://github.com/trust0-project/RIDB/blob/934a985e11b05b646d6653d7f479a4358cad8c68/packages/ridb/src/index.ts#L212)

## Accessors

### collections

#### Get Signature

> **get** **collections**(): \{ \[name in string \| number \| symbol\]: Collection\<Schema\<T\[name\]\>\> \}

Defined in: [ridb/src/index.ts:314](https://github.com/trust0-project/RIDB/blob/934a985e11b05b646d6653d7f479a4358cad8c68/packages/ridb/src/index.ts#L314)

##### Returns

\{ \[name in string \| number \| symbol\]: Collection\<Schema\<T\[name\]\>\> \}

***

### db

#### Get Signature

> **get** `private` **db**(): [`Database`](Database.md)\<`T`\>

Defined in: [ridb/src/index.ts:259](https://github.com/trust0-project/RIDB/blob/934a985e11b05b646d6653d7f479a4358cad8c68/packages/ridb/src/index.ts#L259)

Gets the database instance. Throws an error if the database has not been started.

##### Throws

Will throw an error if the database is not started.

##### Returns

[`Database`](Database.md)\<`T`\>

***

### dbCollections

#### Get Signature

> **get** `private` **dbCollections**(): \{ \[name in string \| number \| symbol\]: Collection\<Schema\<T\[name\]\>\> \}

Defined in: [ridb/src/index.ts:278](https://github.com/trust0-project/RIDB/blob/934a985e11b05b646d6653d7f479a4358cad8c68/packages/ridb/src/index.ts#L278)

Gets the collections from the database.

##### Returns

\{ \[name in string \| number \| symbol\]: Collection\<Schema\<T\[name\]\>\> \}

The collections object.

***

### dbName

#### Get Signature

> **get** `private` **dbName**(): `string`

Defined in: [ridb/src/index.ts:216](https://github.com/trust0-project/RIDB/blob/934a985e11b05b646d6653d7f479a4358cad8c68/packages/ridb/src/index.ts#L216)

##### Returns

`string`

***

### migrations

#### Get Signature

> **get** `private` **migrations**(): [`MigrationPathsForSchemas`](../type-aliases/MigrationPathsForSchemas.md)\<`T`\>

Defined in: [ridb/src/index.ts:224](https://github.com/trust0-project/RIDB/blob/934a985e11b05b646d6653d7f479a4358cad8c68/packages/ridb/src/index.ts#L224)

##### Returns

[`MigrationPathsForSchemas`](../type-aliases/MigrationPathsForSchemas.md)\<`T`\>

***

### plugins

#### Get Signature

> **get** `private` **plugins**(): *typeof* [`BasePlugin`](BasePlugin.md)[]

Defined in: [ridb/src/index.ts:228](https://github.com/trust0-project/RIDB/blob/934a985e11b05b646d6653d7f479a4358cad8c68/packages/ridb/src/index.ts#L228)

##### Returns

*typeof* [`BasePlugin`](BasePlugin.md)[]

***

### schemas

#### Get Signature

> **get** `private` **schemas**(): `T`

Defined in: [ridb/src/index.ts:220](https://github.com/trust0-project/RIDB/blob/934a985e11b05b646d6653d7f479a4358cad8c68/packages/ridb/src/index.ts#L220)

##### Returns

`T`

***

### useWorker

#### Get Signature

> **get** **useWorker**(): `boolean`

Defined in: [ridb/src/index.ts:232](https://github.com/trust0-project/RIDB/blob/934a985e11b05b646d6653d7f479a4358cad8c68/packages/ridb/src/index.ts#L232)

##### Returns

`boolean`

***

### worker

#### Get Signature

> **get** `private` **worker**(): `SharedWorker`

Defined in: [ridb/src/index.ts:266](https://github.com/trust0-project/RIDB/blob/934a985e11b05b646d6653d7f479a4358cad8c68/packages/ridb/src/index.ts#L266)

##### Returns

`SharedWorker`

***

### workerCollections

#### Get Signature

> **get** `private` **workerCollections**(): \{ \[name in string \| number \| symbol\]: Collection\<Schema\<T\[name\]\>\> \}

Defined in: [ridb/src/index.ts:282](https://github.com/trust0-project/RIDB/blob/934a985e11b05b646d6653d7f479a4358cad8c68/packages/ridb/src/index.ts#L282)

##### Returns

\{ \[name in string \| number \| symbol\]: Collection\<Schema\<T\[name\]\>\> \}

## Methods

### authenticate()

> **authenticate**(`password`): `Promise`\<`boolean`\>

Defined in: [ridb/src/index.ts:238](https://github.com/trust0-project/RIDB/blob/934a985e11b05b646d6653d7f479a4358cad8c68/packages/ridb/src/index.ts#L238)

#### Parameters

##### password

`string`

#### Returns

`Promise`\<`boolean`\>

***

### close()

> **close**(): `Promise`\<`void`\>

Defined in: [ridb/src/index.ts:401](https://github.com/trust0-project/RIDB/blob/934a985e11b05b646d6653d7f479a4358cad8c68/packages/ridb/src/index.ts#L401)

#### Returns

`Promise`\<`void`\>

***

### createDatabase()

> `private` **createDatabase**(`options`?): `Promise`\<[`Database`](Database.md)\<`T`\>\>

Defined in: [ridb/src/index.ts:342](https://github.com/trust0-project/RIDB/blob/934a985e11b05b646d6653d7f479a4358cad8c68/packages/ridb/src/index.ts#L342)

#### Parameters

##### options?

[`StartOptions`](../type-aliases/StartOptions.md)\<`T`\>

#### Returns

`Promise`\<[`Database`](Database.md)\<`T`\>\>

***

### createWorker()

> `private` **createWorker**(): `Promise`\<`SharedWorker`\>

Defined in: [ridb/src/index.ts:318](https://github.com/trust0-project/RIDB/blob/934a985e11b05b646d6653d7f479a4358cad8c68/packages/ridb/src/index.ts#L318)

#### Returns

`Promise`\<`SharedWorker`\>

***

### getStorageType()

> `private` **getStorageType**\<`T`\>(`storageType`): *typeof* [`InMemory`](InMemory.md)

Defined in: [ridb/src/index.ts:248](https://github.com/trust0-project/RIDB/blob/934a985e11b05b646d6653d7f479a4358cad8c68/packages/ridb/src/index.ts#L248)

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

Defined in: [ridb/src/index.ts:326](https://github.com/trust0-project/RIDB/blob/934a985e11b05b646d6653d7f479a4358cad8c68/packages/ridb/src/index.ts#L326)

#### Parameters

##### event

`MessageEvent`

#### Returns

`Promise`\<`void`\>

***

### start()

> **start**(`options`?): `Promise`\<`void`\>

Defined in: [ridb/src/index.ts:374](https://github.com/trust0-project/RIDB/blob/934a985e11b05b646d6653d7f479a4358cad8c68/packages/ridb/src/index.ts#L374)

Starts the database.

#### Parameters

##### options?

[`StartOptions`](../type-aliases/StartOptions.md)\<`T`\>

#### Returns

`Promise`\<`void`\>

A promise that resolves to the database instance.
