[**@trust0/ridb**](../README.md)

***

[@trust0/ridb](../README.md) / RIDB

# Class: RIDB\<T\>

Defined in: [index.ts:129](https://github.com/trust0-project/RIDB/blob/7fefdb506c021235200393ecaaa3bb6ff87b0ea9/packages/ridb/src/index.ts#L129)

## Extends

- `RIDBCore`\<`T`\>

## Type Parameters

### T

`T` *extends* `SchemaTypeRecord` = `SchemaTypeRecord`

## Constructors

### Constructor

> **new RIDB**\<`T`\>(`options`): `RIDB`\<`T`\>

Defined in: [core.ts:30](https://github.com/trust0-project/RIDB/blob/7fefdb506c021235200393ecaaa3bb6ff87b0ea9/packages/ridb/src/core.ts#L30)

Creates an instance of RIDBImplementation.

#### Parameters

##### options

[`DBOptions`](../type-aliases/DBOptions.md)\<`T`\>

#### Returns

`RIDB`\<`T`\>

#### Inherited from

`RIDBCore<T>.constructor`

## Properties

### \_db

> `protected` **\_db**: `undefined` \| `Database`\<`T`\>

Defined in: [core.ts:22](https://github.com/trust0-project/RIDB/blob/7fefdb506c021235200393ecaaa3bb6ff87b0ea9/packages/ridb/src/core.ts#L22)

#### Inherited from

`RIDBCore._db`

***

### \_sessionId

> `private` **\_sessionId**: `undefined` \| `string`

Defined in: [index.ts:132](https://github.com/trust0-project/RIDB/blob/7fefdb506c021235200393ecaaa3bb6ff87b0ea9/packages/ridb/src/index.ts#L132)

***

### \_worker

> `private` **\_worker**: `undefined` \| `SharedWorker`

Defined in: [index.ts:131](https://github.com/trust0-project/RIDB/blob/7fefdb506c021235200393ecaaa3bb6ff87b0ea9/packages/ridb/src/index.ts#L131)

***

### options

> `protected` **options**: [`DBOptions`](../type-aliases/DBOptions.md)\<`T`\>

Defined in: [core.ts:30](https://github.com/trust0-project/RIDB/blob/7fefdb506c021235200393ecaaa3bb6ff87b0ea9/packages/ridb/src/core.ts#L30)

#### Inherited from

`RIDBCore.options`

***

### pendingRequests

> `protected` **pendingRequests**: [`PendingRequests`](../type-aliases/PendingRequests.md)

Defined in: [core.ts:24](https://github.com/trust0-project/RIDB/blob/7fefdb506c021235200393ecaaa3bb6ff87b0ea9/packages/ridb/src/core.ts#L24)

#### Inherited from

`RIDBCore.pendingRequests`

***

### started

> **started**: `boolean` = `false`

Defined in: [core.ts:23](https://github.com/trust0-project/RIDB/blob/7fefdb506c021235200393ecaaa3bb6ff87b0ea9/packages/ridb/src/core.ts#L23)

#### Inherited from

`RIDBCore.started`

## Accessors

### collections

#### Get Signature

> **get** **collections**(): \{ \[name in string \| number \| symbol\]: Collection\<Schema\<T\[name\]\>\> \}

Defined in: [index.ts:191](https://github.com/trust0-project/RIDB/blob/7fefdb506c021235200393ecaaa3bb6ff87b0ea9/packages/ridb/src/index.ts#L191)

Gets the collections from the database.

##### Returns

\{ \[name in string \| number \| symbol\]: Collection\<Schema\<T\[name\]\>\> \}

The collections object.

#### Overrides

`RIDBCore.collections`

***

### db

#### Get Signature

> **get** **db**(): `Database`\<`T`\>

Defined in: [core.ts:59](https://github.com/trust0-project/RIDB/blob/7fefdb506c021235200393ecaaa3bb6ff87b0ea9/packages/ridb/src/core.ts#L59)

Gets the database instance. Throws an error if the database has not been started.

##### Throws

Will throw an error if the database is not started.

##### Returns

`Database`\<`T`\>

#### Inherited from

`RIDBCore.db`

***

### dbCollections

#### Get Signature

> **get** `private` **dbCollections**(): \{ \[name in string \| number \| symbol\]: Collection\<Schema\<T\[name\]\>\> \}

Defined in: [index.ts:155](https://github.com/trust0-project/RIDB/blob/7fefdb506c021235200393ecaaa3bb6ff87b0ea9/packages/ridb/src/index.ts#L155)

Gets the collections from the database.

##### Returns

\{ \[name in string \| number \| symbol\]: Collection\<Schema\<T\[name\]\>\> \}

The collections object.

***

### dbName

#### Get Signature

> **get** `protected` **dbName**(): `undefined` \| `string`

Defined in: [core.ts:32](https://github.com/trust0-project/RIDB/blob/7fefdb506c021235200393ecaaa3bb6ff87b0ea9/packages/ridb/src/core.ts#L32)

##### Returns

`undefined` \| `string`

#### Inherited from

`RIDBCore.dbName`

***

### migrations

#### Get Signature

> **get** `protected` **migrations**(): `MigrationPathsForSchemas`\<`T`\>

Defined in: [core.ts:40](https://github.com/trust0-project/RIDB/blob/7fefdb506c021235200393ecaaa3bb6ff87b0ea9/packages/ridb/src/core.ts#L40)

##### Returns

`MigrationPathsForSchemas`\<`T`\>

#### Inherited from

`RIDBCore.migrations`

***

### plugins

#### Get Signature

> **get** `protected` **plugins**(): *typeof* `BasePlugin`[]

Defined in: [core.ts:44](https://github.com/trust0-project/RIDB/blob/7fefdb506c021235200393ecaaa3bb6ff87b0ea9/packages/ridb/src/core.ts#L44)

##### Returns

*typeof* `BasePlugin`[]

#### Inherited from

`RIDBCore.plugins`

***

### schemas

#### Get Signature

> **get** `protected` **schemas**(): `T`

Defined in: [core.ts:36](https://github.com/trust0-project/RIDB/blob/7fefdb506c021235200393ecaaa3bb6ff87b0ea9/packages/ridb/src/core.ts#L36)

##### Returns

`T`

#### Inherited from

`RIDBCore.schemas`

***

### useWorker

#### Get Signature

> **get** **useWorker**(): `boolean`

Defined in: [index.ts:134](https://github.com/trust0-project/RIDB/blob/7fefdb506c021235200393ecaaa3bb6ff87b0ea9/packages/ridb/src/index.ts#L134)

##### Returns

`boolean`

***

### worker

#### Get Signature

> **get** `private` **worker**(): `SharedWorker`

Defined in: [index.ts:144](https://github.com/trust0-project/RIDB/blob/7fefdb506c021235200393ecaaa3bb6ff87b0ea9/packages/ridb/src/index.ts#L144)

##### Returns

`SharedWorker`

***

### workerCollections

#### Get Signature

> **get** `private` **workerCollections**(): \{ \[name in string \| number \| symbol\]: Collection\<Schema\<T\[name\]\>\> \}

Defined in: [index.ts:159](https://github.com/trust0-project/RIDB/blob/7fefdb506c021235200393ecaaa3bb6ff87b0ea9/packages/ridb/src/index.ts#L159)

##### Returns

\{ \[name in string \| number \| symbol\]: Collection\<Schema\<T\[name\]\>\> \}

## Methods

### authenticate()

> **authenticate**(`password`): `Promise`\<`boolean`\>

Defined in: [index.ts:140](https://github.com/trust0-project/RIDB/blob/7fefdb506c021235200393ecaaa3bb6ff87b0ea9/packages/ridb/src/index.ts#L140)

#### Parameters

##### password

`string`

#### Returns

`Promise`\<`boolean`\>

#### Overrides

`RIDBCore.authenticate`

***

### close()

> **close**(): `Promise`\<`void`\>

Defined in: [index.ts:247](https://github.com/trust0-project/RIDB/blob/7fefdb506c021235200393ecaaa3bb6ff87b0ea9/packages/ridb/src/index.ts#L247)

#### Returns

`Promise`\<`void`\>

#### Overrides

`RIDBCore.close`

***

### createDatabase()

> `protected` **createDatabase**(`options?`): `Promise`\<`Database`\<`T`\>\>

Defined in: [core.ts:78](https://github.com/trust0-project/RIDB/blob/7fefdb506c021235200393ecaaa3bb6ff87b0ea9/packages/ridb/src/core.ts#L78)

#### Parameters

##### options?

[`StartOptions`](../type-aliases/StartOptions.md)\<`T`\>

#### Returns

`Promise`\<`Database`\<`T`\>\>

#### Inherited from

`RIDBCore.createDatabase`

***

### getStorageType()

> `protected` **getStorageType**\<`T`\>(`storageType`): `Promise`\<*typeof* `InMemory`\>

Defined in: [core.ts:48](https://github.com/trust0-project/RIDB/blob/7fefdb506c021235200393ecaaa3bb6ff87b0ea9/packages/ridb/src/core.ts#L48)

#### Type Parameters

##### T

`T` *extends* [`StorageType`](../enumerations/StorageType.md)

#### Parameters

##### storageType

`T`

#### Returns

`Promise`\<*typeof* `InMemory`\>

#### Inherited from

`RIDBCore.getStorageType`

***

### initializeWorker()

> `private` **initializeWorker**(): `Promise`\<`void`\>

Defined in: [index.ts:195](https://github.com/trust0-project/RIDB/blob/7fefdb506c021235200393ecaaa3bb6ff87b0ea9/packages/ridb/src/index.ts#L195)

#### Returns

`Promise`\<`void`\>

***

### start()

> **start**(`options?`): `Promise`\<`void`\>

Defined in: [index.ts:217](https://github.com/trust0-project/RIDB/blob/7fefdb506c021235200393ecaaa3bb6ff87b0ea9/packages/ridb/src/index.ts#L217)

Starts the database.

#### Parameters

##### options?

[`StartOptions`](../type-aliases/StartOptions.md)\<`T`\>

#### Returns

`Promise`\<`void`\>

A promise that resolves when the database is started.

#### Overrides

`RIDBCore.start`
