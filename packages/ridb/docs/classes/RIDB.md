[**@trust0/ridb**](../README.md)

***

[@trust0/ridb](../README.md) / RIDB

# Class: RIDB\<T\>

Defined in: [index.ts:169](https://github.com/trust0-project/RIDB/blob/7186e3c53446e30ba6bd6372c643e48c6837c634/packages/ridb/src/index.ts#L169)

## Type Parameters

### T

`T` *extends* `SchemaTypeRecord` = `SchemaTypeRecord`

## Constructors

### Constructor

> **new RIDB**\<`T`\>(`options`): `RIDB`\<`T`\>

Defined in: [index.ts:207](https://github.com/trust0-project/RIDB/blob/7186e3c53446e30ba6bd6372c643e48c6837c634/packages/ridb/src/index.ts#L207)

Creates an instance of RIDB.

#### Parameters

##### options

[`DBOptions`](../type-aliases/DBOptions.md)\<`T`\>

#### Returns

`RIDB`\<`T`\>

## Properties

### \_db

> `private` **\_db**: `undefined` \| `Database`\<`T`\>

Defined in: [index.ts:170](https://github.com/trust0-project/RIDB/blob/7186e3c53446e30ba6bd6372c643e48c6837c634/packages/ridb/src/index.ts#L170)

***

### \_sessionId

> `private` **\_sessionId**: `undefined` \| `string`

Defined in: [index.ts:172](https://github.com/trust0-project/RIDB/blob/7186e3c53446e30ba6bd6372c643e48c6837c634/packages/ridb/src/index.ts#L172)

***

### \_worker

> `private` **\_worker**: `undefined` \| `SharedWorker`

Defined in: [index.ts:171](https://github.com/trust0-project/RIDB/blob/7186e3c53446e30ba6bd6372c643e48c6837c634/packages/ridb/src/index.ts#L171)

***

### options

> `private` **options**: [`DBOptions`](../type-aliases/DBOptions.md)\<`T`\>

Defined in: [index.ts:207](https://github.com/trust0-project/RIDB/blob/7186e3c53446e30ba6bd6372c643e48c6837c634/packages/ridb/src/index.ts#L207)

***

### pendingRequests

> `private` **pendingRequests**: [`PendingRequests`](../type-aliases/PendingRequests.md)

Defined in: [index.ts:175](https://github.com/trust0-project/RIDB/blob/7186e3c53446e30ba6bd6372c643e48c6837c634/packages/ridb/src/index.ts#L175)

***

### started

> **started**: `boolean` = `false`

Defined in: [index.ts:173](https://github.com/trust0-project/RIDB/blob/7186e3c53446e30ba6bd6372c643e48c6837c634/packages/ridb/src/index.ts#L173)

## Accessors

### collections

#### Get Signature

> **get** **collections**(): \{ \[name in string \| number \| symbol\]: Collection\<Schema\<T\[name\]\>\> \}

Defined in: [index.ts:276](https://github.com/trust0-project/RIDB/blob/7186e3c53446e30ba6bd6372c643e48c6837c634/packages/ridb/src/index.ts#L276)

##### Returns

\{ \[name in string \| number \| symbol\]: Collection\<Schema\<T\[name\]\>\> \}

***

### db

#### Get Signature

> **get** `private` **db**(): `Database`\<`T`\>

Defined in: [index.ts:221](https://github.com/trust0-project/RIDB/blob/7186e3c53446e30ba6bd6372c643e48c6837c634/packages/ridb/src/index.ts#L221)

Gets the database instance. Throws an error if the database has not been started.

##### Throws

Will throw an error if the database is not started.

##### Returns

`Database`\<`T`\>

***

### dbCollections

#### Get Signature

> **get** `private` **dbCollections**(): \{ \[name in string \| number \| symbol\]: Collection\<Schema\<T\[name\]\>\> \}

Defined in: [index.ts:240](https://github.com/trust0-project/RIDB/blob/7186e3c53446e30ba6bd6372c643e48c6837c634/packages/ridb/src/index.ts#L240)

Gets the collections from the database.

##### Returns

\{ \[name in string \| number \| symbol\]: Collection\<Schema\<T\[name\]\>\> \}

The collections object.

***

### dbName

#### Get Signature

> **get** `private` **dbName**(): `string`

Defined in: [index.ts:177](https://github.com/trust0-project/RIDB/blob/7186e3c53446e30ba6bd6372c643e48c6837c634/packages/ridb/src/index.ts#L177)

##### Returns

`string`

***

### migrations

#### Get Signature

> **get** `private` **migrations**(): `MigrationPathsForSchemas`\<`T`\>

Defined in: [index.ts:185](https://github.com/trust0-project/RIDB/blob/7186e3c53446e30ba6bd6372c643e48c6837c634/packages/ridb/src/index.ts#L185)

##### Returns

`MigrationPathsForSchemas`\<`T`\>

***

### plugins

#### Get Signature

> **get** `private` **plugins**(): *typeof* `BasePlugin`[]

Defined in: [index.ts:189](https://github.com/trust0-project/RIDB/blob/7186e3c53446e30ba6bd6372c643e48c6837c634/packages/ridb/src/index.ts#L189)

##### Returns

*typeof* `BasePlugin`[]

***

### schemas

#### Get Signature

> **get** `private` **schemas**(): `T`

Defined in: [index.ts:181](https://github.com/trust0-project/RIDB/blob/7186e3c53446e30ba6bd6372c643e48c6837c634/packages/ridb/src/index.ts#L181)

##### Returns

`T`

***

### useWorker

#### Get Signature

> **get** **useWorker**(): `boolean`

Defined in: [index.ts:193](https://github.com/trust0-project/RIDB/blob/7186e3c53446e30ba6bd6372c643e48c6837c634/packages/ridb/src/index.ts#L193)

##### Returns

`boolean`

***

### worker

#### Get Signature

> **get** `private` **worker**(): `SharedWorker`

Defined in: [index.ts:228](https://github.com/trust0-project/RIDB/blob/7186e3c53446e30ba6bd6372c643e48c6837c634/packages/ridb/src/index.ts#L228)

##### Returns

`SharedWorker`

***

### workerCollections

#### Get Signature

> **get** `private` **workerCollections**(): \{ \[name in string \| number \| symbol\]: Collection\<Schema\<T\[name\]\>\> \}

Defined in: [index.ts:244](https://github.com/trust0-project/RIDB/blob/7186e3c53446e30ba6bd6372c643e48c6837c634/packages/ridb/src/index.ts#L244)

##### Returns

\{ \[name in string \| number \| symbol\]: Collection\<Schema\<T\[name\]\>\> \}

## Methods

### authenticate()

> **authenticate**(`password`): `Promise`\<`boolean`\>

Defined in: [index.ts:199](https://github.com/trust0-project/RIDB/blob/7186e3c53446e30ba6bd6372c643e48c6837c634/packages/ridb/src/index.ts#L199)

#### Parameters

##### password

`string`

#### Returns

`Promise`\<`boolean`\>

***

### close()

> **close**(): `Promise`\<`void`\>

Defined in: [index.ts:371](https://github.com/trust0-project/RIDB/blob/7186e3c53446e30ba6bd6372c643e48c6837c634/packages/ridb/src/index.ts#L371)

#### Returns

`Promise`\<`void`\>

***

### createDatabase()

> `private` **createDatabase**(`options?`): `Promise`\<`Database`\<`T`\>\>

Defined in: [index.ts:310](https://github.com/trust0-project/RIDB/blob/7186e3c53446e30ba6bd6372c643e48c6837c634/packages/ridb/src/index.ts#L310)

#### Parameters

##### options?

[`StartOptions`](../type-aliases/StartOptions.md)\<`T`\>

#### Returns

`Promise`\<`Database`\<`T`\>\>

***

### createWorker()

> `private` **createWorker**(): `SharedWorker`

Defined in: [index.ts:280](https://github.com/trust0-project/RIDB/blob/7186e3c53446e30ba6bd6372c643e48c6837c634/packages/ridb/src/index.ts#L280)

#### Returns

`SharedWorker`

***

### getStorageType()

> `private` **getStorageType**\<`T`\>(`storageType`): `Promise`\<*typeof* `InMemory`\>

Defined in: [index.ts:209](https://github.com/trust0-project/RIDB/blob/7186e3c53446e30ba6bd6372c643e48c6837c634/packages/ridb/src/index.ts#L209)

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

Defined in: [index.ts:292](https://github.com/trust0-project/RIDB/blob/7186e3c53446e30ba6bd6372c643e48c6837c634/packages/ridb/src/index.ts#L292)

#### Parameters

##### event

`MessageEvent`

#### Returns

`Promise`\<`void`\>

***

### start()

> **start**(`options?`): `Promise`\<`void`\>

Defined in: [index.ts:343](https://github.com/trust0-project/RIDB/blob/7186e3c53446e30ba6bd6372c643e48c6837c634/packages/ridb/src/index.ts#L343)

Starts the database.

#### Parameters

##### options?

[`StartOptions`](../type-aliases/StartOptions.md)\<`T`\>

#### Returns

`Promise`\<`void`\>

A promise that resolves to the database instance.
