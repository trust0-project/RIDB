[**@trust0/ridb**](../README.md)

***

[@trust0/ridb](../README.md) / RIDB

# Class: RIDB\<T\>

Defined in: [index.ts:168](https://github.com/trust0-project/RIDB/blob/4f509ace700a81f7b619907a7f52dd8788b6e257/packages/ridb/src/index.ts#L168)

## Type Parameters

### T

`T` *extends* `SchemaTypeRecord` = `SchemaTypeRecord`

## Constructors

### Constructor

> **new RIDB**\<`T`\>(`options`): `RIDB`\<`T`\>

Defined in: [index.ts:206](https://github.com/trust0-project/RIDB/blob/4f509ace700a81f7b619907a7f52dd8788b6e257/packages/ridb/src/index.ts#L206)

Creates an instance of RIDB.

#### Parameters

##### options

[`DBOptions`](../type-aliases/DBOptions.md)\<`T`\>

#### Returns

`RIDB`\<`T`\>

## Properties

### \_db

> `private` **\_db**: `undefined` \| `Database`\<`T`\>

Defined in: [index.ts:169](https://github.com/trust0-project/RIDB/blob/4f509ace700a81f7b619907a7f52dd8788b6e257/packages/ridb/src/index.ts#L169)

***

### \_sessionId

> `private` **\_sessionId**: `undefined` \| `string`

Defined in: [index.ts:171](https://github.com/trust0-project/RIDB/blob/4f509ace700a81f7b619907a7f52dd8788b6e257/packages/ridb/src/index.ts#L171)

***

### \_worker

> `private` **\_worker**: `undefined` \| `SharedWorker`

Defined in: [index.ts:170](https://github.com/trust0-project/RIDB/blob/4f509ace700a81f7b619907a7f52dd8788b6e257/packages/ridb/src/index.ts#L170)

***

### options

> `private` **options**: [`DBOptions`](../type-aliases/DBOptions.md)\<`T`\>

Defined in: [index.ts:206](https://github.com/trust0-project/RIDB/blob/4f509ace700a81f7b619907a7f52dd8788b6e257/packages/ridb/src/index.ts#L206)

***

### pendingRequests

> `private` **pendingRequests**: [`PendingRequests`](../type-aliases/PendingRequests.md)

Defined in: [index.ts:174](https://github.com/trust0-project/RIDB/blob/4f509ace700a81f7b619907a7f52dd8788b6e257/packages/ridb/src/index.ts#L174)

***

### started

> **started**: `boolean` = `false`

Defined in: [index.ts:172](https://github.com/trust0-project/RIDB/blob/4f509ace700a81f7b619907a7f52dd8788b6e257/packages/ridb/src/index.ts#L172)

## Accessors

### collections

#### Get Signature

> **get** **collections**(): \{ \[name in string \| number \| symbol\]: Collection\<Schema\<T\[name\]\>\> \}

Defined in: [index.ts:275](https://github.com/trust0-project/RIDB/blob/4f509ace700a81f7b619907a7f52dd8788b6e257/packages/ridb/src/index.ts#L275)

##### Returns

\{ \[name in string \| number \| symbol\]: Collection\<Schema\<T\[name\]\>\> \}

***

### db

#### Get Signature

> **get** `private` **db**(): `Database`\<`T`\>

Defined in: [index.ts:220](https://github.com/trust0-project/RIDB/blob/4f509ace700a81f7b619907a7f52dd8788b6e257/packages/ridb/src/index.ts#L220)

Gets the database instance. Throws an error if the database has not been started.

##### Throws

Will throw an error if the database is not started.

##### Returns

`Database`\<`T`\>

***

### dbCollections

#### Get Signature

> **get** `private` **dbCollections**(): \{ \[name in string \| number \| symbol\]: Collection\<Schema\<T\[name\]\>\> \}

Defined in: [index.ts:239](https://github.com/trust0-project/RIDB/blob/4f509ace700a81f7b619907a7f52dd8788b6e257/packages/ridb/src/index.ts#L239)

Gets the collections from the database.

##### Returns

\{ \[name in string \| number \| symbol\]: Collection\<Schema\<T\[name\]\>\> \}

The collections object.

***

### dbName

#### Get Signature

> **get** `private` **dbName**(): `string`

Defined in: [index.ts:176](https://github.com/trust0-project/RIDB/blob/4f509ace700a81f7b619907a7f52dd8788b6e257/packages/ridb/src/index.ts#L176)

##### Returns

`string`

***

### migrations

#### Get Signature

> **get** `private` **migrations**(): `MigrationPathsForSchemas`\<`T`\>

Defined in: [index.ts:184](https://github.com/trust0-project/RIDB/blob/4f509ace700a81f7b619907a7f52dd8788b6e257/packages/ridb/src/index.ts#L184)

##### Returns

`MigrationPathsForSchemas`\<`T`\>

***

### plugins

#### Get Signature

> **get** `private` **plugins**(): *typeof* `BasePlugin`[]

Defined in: [index.ts:188](https://github.com/trust0-project/RIDB/blob/4f509ace700a81f7b619907a7f52dd8788b6e257/packages/ridb/src/index.ts#L188)

##### Returns

*typeof* `BasePlugin`[]

***

### schemas

#### Get Signature

> **get** `private` **schemas**(): `T`

Defined in: [index.ts:180](https://github.com/trust0-project/RIDB/blob/4f509ace700a81f7b619907a7f52dd8788b6e257/packages/ridb/src/index.ts#L180)

##### Returns

`T`

***

### useWorker

#### Get Signature

> **get** **useWorker**(): `boolean`

Defined in: [index.ts:192](https://github.com/trust0-project/RIDB/blob/4f509ace700a81f7b619907a7f52dd8788b6e257/packages/ridb/src/index.ts#L192)

##### Returns

`boolean`

***

### worker

#### Get Signature

> **get** `private` **worker**(): `SharedWorker`

Defined in: [index.ts:227](https://github.com/trust0-project/RIDB/blob/4f509ace700a81f7b619907a7f52dd8788b6e257/packages/ridb/src/index.ts#L227)

##### Returns

`SharedWorker`

***

### workerCollections

#### Get Signature

> **get** `private` **workerCollections**(): \{ \[name in string \| number \| symbol\]: Collection\<Schema\<T\[name\]\>\> \}

Defined in: [index.ts:243](https://github.com/trust0-project/RIDB/blob/4f509ace700a81f7b619907a7f52dd8788b6e257/packages/ridb/src/index.ts#L243)

##### Returns

\{ \[name in string \| number \| symbol\]: Collection\<Schema\<T\[name\]\>\> \}

## Methods

### authenticate()

> **authenticate**(`password`): `Promise`\<`boolean`\>

Defined in: [index.ts:198](https://github.com/trust0-project/RIDB/blob/4f509ace700a81f7b619907a7f52dd8788b6e257/packages/ridb/src/index.ts#L198)

#### Parameters

##### password

`string`

#### Returns

`Promise`\<`boolean`\>

***

### close()

> **close**(): `Promise`\<`void`\>

Defined in: [index.ts:369](https://github.com/trust0-project/RIDB/blob/4f509ace700a81f7b619907a7f52dd8788b6e257/packages/ridb/src/index.ts#L369)

#### Returns

`Promise`\<`void`\>

***

### createDatabase()

> `private` **createDatabase**(`options?`): `Promise`\<`Database`\<`T`\>\>

Defined in: [index.ts:309](https://github.com/trust0-project/RIDB/blob/4f509ace700a81f7b619907a7f52dd8788b6e257/packages/ridb/src/index.ts#L309)

#### Parameters

##### options?

[`StartOptions`](../type-aliases/StartOptions.md)\<`T`\>

#### Returns

`Promise`\<`Database`\<`T`\>\>

***

### createWorker()

> `private` **createWorker**(): `SharedWorker`

Defined in: [index.ts:279](https://github.com/trust0-project/RIDB/blob/4f509ace700a81f7b619907a7f52dd8788b6e257/packages/ridb/src/index.ts#L279)

#### Returns

`SharedWorker`

***

### getStorageType()

> `private` **getStorageType**\<`T`\>(`storageType`): `Promise`\<*typeof* `InMemory`\>

Defined in: [index.ts:208](https://github.com/trust0-project/RIDB/blob/4f509ace700a81f7b619907a7f52dd8788b6e257/packages/ridb/src/index.ts#L208)

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

Defined in: [index.ts:291](https://github.com/trust0-project/RIDB/blob/4f509ace700a81f7b619907a7f52dd8788b6e257/packages/ridb/src/index.ts#L291)

#### Parameters

##### event

`MessageEvent`

#### Returns

`Promise`\<`void`\>

***

### start()

> **start**(`options?`): `Promise`\<`void`\>

Defined in: [index.ts:342](https://github.com/trust0-project/RIDB/blob/4f509ace700a81f7b619907a7f52dd8788b6e257/packages/ridb/src/index.ts#L342)

Starts the database.

#### Parameters

##### options?

[`StartOptions`](../type-aliases/StartOptions.md)\<`T`\>

#### Returns

`Promise`\<`void`\>

A promise that resolves to the database instance.
