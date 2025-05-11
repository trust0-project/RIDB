[**@trust0/ridb**](../README.md)

***

[@trust0/ridb](../README.md) / RIDB

# Class: RIDB\<T\>

Defined in: [index.ts:172](https://github.com/trust0-project/RIDB/blob/4815311545e43c9df945cbfb5e22c6947392dfb7/packages/ridb/src/index.ts#L172)

## Type Parameters

### T

`T` *extends* `SchemaTypeRecord` = `SchemaTypeRecord`

## Constructors

### Constructor

> **new RIDB**\<`T`\>(`options`): `RIDB`\<`T`\>

Defined in: [index.ts:210](https://github.com/trust0-project/RIDB/blob/4815311545e43c9df945cbfb5e22c6947392dfb7/packages/ridb/src/index.ts#L210)

Creates an instance of RIDB.

#### Parameters

##### options

[`DBOptions`](../type-aliases/DBOptions.md)\<`T`\>

#### Returns

`RIDB`\<`T`\>

## Properties

### \_db

> `private` **\_db**: `undefined` \| `Database`\<`T`\>

Defined in: [index.ts:173](https://github.com/trust0-project/RIDB/blob/4815311545e43c9df945cbfb5e22c6947392dfb7/packages/ridb/src/index.ts#L173)

***

### \_sessionId

> `private` **\_sessionId**: `undefined` \| `string`

Defined in: [index.ts:175](https://github.com/trust0-project/RIDB/blob/4815311545e43c9df945cbfb5e22c6947392dfb7/packages/ridb/src/index.ts#L175)

***

### \_worker

> `private` **\_worker**: `undefined` \| `SharedWorker`

Defined in: [index.ts:174](https://github.com/trust0-project/RIDB/blob/4815311545e43c9df945cbfb5e22c6947392dfb7/packages/ridb/src/index.ts#L174)

***

### options

> `private` **options**: [`DBOptions`](../type-aliases/DBOptions.md)\<`T`\>

Defined in: [index.ts:210](https://github.com/trust0-project/RIDB/blob/4815311545e43c9df945cbfb5e22c6947392dfb7/packages/ridb/src/index.ts#L210)

***

### pendingRequests

> `private` **pendingRequests**: [`PendingRequests`](../type-aliases/PendingRequests.md)

Defined in: [index.ts:178](https://github.com/trust0-project/RIDB/blob/4815311545e43c9df945cbfb5e22c6947392dfb7/packages/ridb/src/index.ts#L178)

***

### started

> **started**: `boolean` = `false`

Defined in: [index.ts:176](https://github.com/trust0-project/RIDB/blob/4815311545e43c9df945cbfb5e22c6947392dfb7/packages/ridb/src/index.ts#L176)

## Accessors

### collections

#### Get Signature

> **get** **collections**(): \{ \[name in string \| number \| symbol\]: Collection\<Schema\<T\[name\]\>\> \}

Defined in: [index.ts:279](https://github.com/trust0-project/RIDB/blob/4815311545e43c9df945cbfb5e22c6947392dfb7/packages/ridb/src/index.ts#L279)

##### Returns

\{ \[name in string \| number \| symbol\]: Collection\<Schema\<T\[name\]\>\> \}

***

### db

#### Get Signature

> **get** `private` **db**(): `Database`\<`T`\>

Defined in: [index.ts:224](https://github.com/trust0-project/RIDB/blob/4815311545e43c9df945cbfb5e22c6947392dfb7/packages/ridb/src/index.ts#L224)

Gets the database instance. Throws an error if the database has not been started.

##### Throws

Will throw an error if the database is not started.

##### Returns

`Database`\<`T`\>

***

### dbCollections

#### Get Signature

> **get** `private` **dbCollections**(): \{ \[name in string \| number \| symbol\]: Collection\<Schema\<T\[name\]\>\> \}

Defined in: [index.ts:243](https://github.com/trust0-project/RIDB/blob/4815311545e43c9df945cbfb5e22c6947392dfb7/packages/ridb/src/index.ts#L243)

Gets the collections from the database.

##### Returns

\{ \[name in string \| number \| symbol\]: Collection\<Schema\<T\[name\]\>\> \}

The collections object.

***

### dbName

#### Get Signature

> **get** `private` **dbName**(): `undefined` \| `string`

Defined in: [index.ts:180](https://github.com/trust0-project/RIDB/blob/4815311545e43c9df945cbfb5e22c6947392dfb7/packages/ridb/src/index.ts#L180)

##### Returns

`undefined` \| `string`

***

### migrations

#### Get Signature

> **get** `private` **migrations**(): `MigrationPathsForSchemas`\<`T`\>

Defined in: [index.ts:188](https://github.com/trust0-project/RIDB/blob/4815311545e43c9df945cbfb5e22c6947392dfb7/packages/ridb/src/index.ts#L188)

##### Returns

`MigrationPathsForSchemas`\<`T`\>

***

### plugins

#### Get Signature

> **get** `private` **plugins**(): *typeof* `BasePlugin`[]

Defined in: [index.ts:192](https://github.com/trust0-project/RIDB/blob/4815311545e43c9df945cbfb5e22c6947392dfb7/packages/ridb/src/index.ts#L192)

##### Returns

*typeof* `BasePlugin`[]

***

### schemas

#### Get Signature

> **get** `private` **schemas**(): `T`

Defined in: [index.ts:184](https://github.com/trust0-project/RIDB/blob/4815311545e43c9df945cbfb5e22c6947392dfb7/packages/ridb/src/index.ts#L184)

##### Returns

`T`

***

### useWorker

#### Get Signature

> **get** **useWorker**(): `boolean`

Defined in: [index.ts:196](https://github.com/trust0-project/RIDB/blob/4815311545e43c9df945cbfb5e22c6947392dfb7/packages/ridb/src/index.ts#L196)

##### Returns

`boolean`

***

### worker

#### Get Signature

> **get** `private` **worker**(): `SharedWorker`

Defined in: [index.ts:231](https://github.com/trust0-project/RIDB/blob/4815311545e43c9df945cbfb5e22c6947392dfb7/packages/ridb/src/index.ts#L231)

##### Returns

`SharedWorker`

***

### workerCollections

#### Get Signature

> **get** `private` **workerCollections**(): \{ \[name in string \| number \| symbol\]: Collection\<Schema\<T\[name\]\>\> \}

Defined in: [index.ts:247](https://github.com/trust0-project/RIDB/blob/4815311545e43c9df945cbfb5e22c6947392dfb7/packages/ridb/src/index.ts#L247)

##### Returns

\{ \[name in string \| number \| symbol\]: Collection\<Schema\<T\[name\]\>\> \}

## Methods

### authenticate()

> **authenticate**(`password`): `Promise`\<`boolean`\>

Defined in: [index.ts:202](https://github.com/trust0-project/RIDB/blob/4815311545e43c9df945cbfb5e22c6947392dfb7/packages/ridb/src/index.ts#L202)

#### Parameters

##### password

`string`

#### Returns

`Promise`\<`boolean`\>

***

### close()

> **close**(): `Promise`\<`void`\>

Defined in: [index.ts:376](https://github.com/trust0-project/RIDB/blob/4815311545e43c9df945cbfb5e22c6947392dfb7/packages/ridb/src/index.ts#L376)

#### Returns

`Promise`\<`void`\>

***

### createDatabase()

> `private` **createDatabase**(`options?`): `Promise`\<`Database`\<`T`\>\>

Defined in: [index.ts:313](https://github.com/trust0-project/RIDB/blob/4815311545e43c9df945cbfb5e22c6947392dfb7/packages/ridb/src/index.ts#L313)

#### Parameters

##### options?

[`StartOptions`](../type-aliases/StartOptions.md)\<`T`\>

#### Returns

`Promise`\<`Database`\<`T`\>\>

***

### createWorker()

> `private` **createWorker**(): `SharedWorker`

Defined in: [index.ts:283](https://github.com/trust0-project/RIDB/blob/4815311545e43c9df945cbfb5e22c6947392dfb7/packages/ridb/src/index.ts#L283)

#### Returns

`SharedWorker`

***

### getStorageType()

> `private` **getStorageType**\<`T`\>(`storageType`): `Promise`\<*typeof* `InMemory`\>

Defined in: [index.ts:212](https://github.com/trust0-project/RIDB/blob/4815311545e43c9df945cbfb5e22c6947392dfb7/packages/ridb/src/index.ts#L212)

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

Defined in: [index.ts:295](https://github.com/trust0-project/RIDB/blob/4815311545e43c9df945cbfb5e22c6947392dfb7/packages/ridb/src/index.ts#L295)

#### Parameters

##### event

`MessageEvent`

#### Returns

`Promise`\<`void`\>

***

### start()

> **start**(`options?`): `Promise`\<`void`\>

Defined in: [index.ts:348](https://github.com/trust0-project/RIDB/blob/4815311545e43c9df945cbfb5e22c6947392dfb7/packages/ridb/src/index.ts#L348)

Starts the database.

#### Parameters

##### options?

[`StartOptions`](../type-aliases/StartOptions.md)\<`T`\>

#### Returns

`Promise`\<`void`\>

A promise that resolves to the database instance.
