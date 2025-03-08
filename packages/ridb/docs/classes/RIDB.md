[**@trust0/ridb**](../README.md)

***

[@trust0/ridb](../README.md) / RIDB

# Class: RIDB\<T\>

Defined in: [ridb/src/index.ts:199](https://github.com/trust0-project/RIDB/blob/23b6db69eaeecdb007c5527c1028a5ec7519b6e7/packages/ridb/src/index.ts#L199)

## Type Parameters

• **T** *extends* [`SchemaTypeRecord`](../type-aliases/SchemaTypeRecord.md) = [`SchemaTypeRecord`](../type-aliases/SchemaTypeRecord.md)

## Constructors

### new RIDB()

> **new RIDB**\<`T`\>(`options`): [`RIDB`](RIDB.md)\<`T`\>

Defined in: [ridb/src/index.ts:233](https://github.com/trust0-project/RIDB/blob/23b6db69eaeecdb007c5527c1028a5ec7519b6e7/packages/ridb/src/index.ts#L233)

Creates an instance of RIDB.

#### Parameters

##### options

`DBOptions`\<`T`\>

#### Returns

[`RIDB`](RIDB.md)\<`T`\>

## Properties

### \_db

> `private` **\_db**: `undefined` \| [`Database`](Database.md)\<`T`\>

Defined in: [ridb/src/index.ts:200](https://github.com/trust0-project/RIDB/blob/23b6db69eaeecdb007c5527c1028a5ec7519b6e7/packages/ridb/src/index.ts#L200)

***

### \_sessionId

> `private` **\_sessionId**: `undefined` \| `string`

Defined in: [ridb/src/index.ts:202](https://github.com/trust0-project/RIDB/blob/23b6db69eaeecdb007c5527c1028a5ec7519b6e7/packages/ridb/src/index.ts#L202)

***

### \_worker

> `private` **\_worker**: `undefined` \| `SharedWorker`

Defined in: [ridb/src/index.ts:201](https://github.com/trust0-project/RIDB/blob/23b6db69eaeecdb007c5527c1028a5ec7519b6e7/packages/ridb/src/index.ts#L201)

***

### options

> `private` **options**: `DBOptions`\<`T`\>

Defined in: [ridb/src/index.ts:233](https://github.com/trust0-project/RIDB/blob/23b6db69eaeecdb007c5527c1028a5ec7519b6e7/packages/ridb/src/index.ts#L233)

***

### pendingRequests

> `private` **pendingRequests**: `PendingRequests`

Defined in: [ridb/src/index.ts:205](https://github.com/trust0-project/RIDB/blob/23b6db69eaeecdb007c5527c1028a5ec7519b6e7/packages/ridb/src/index.ts#L205)

***

### started

> **started**: `boolean` = `false`

Defined in: [ridb/src/index.ts:203](https://github.com/trust0-project/RIDB/blob/23b6db69eaeecdb007c5527c1028a5ec7519b6e7/packages/ridb/src/index.ts#L203)

## Accessors

### collections

#### Get Signature

> **get** **collections**(): \{ \[name in string \| number \| symbol\]: Collection\<Schema\<T\[name\]\>\> \}

Defined in: [ridb/src/index.ts:301](https://github.com/trust0-project/RIDB/blob/23b6db69eaeecdb007c5527c1028a5ec7519b6e7/packages/ridb/src/index.ts#L301)

##### Returns

\{ \[name in string \| number \| symbol\]: Collection\<Schema\<T\[name\]\>\> \}

***

### db

#### Get Signature

> **get** `private` **db**(): [`Database`](Database.md)\<`T`\>

Defined in: [ridb/src/index.ts:246](https://github.com/trust0-project/RIDB/blob/23b6db69eaeecdb007c5527c1028a5ec7519b6e7/packages/ridb/src/index.ts#L246)

Gets the database instance. Throws an error if the database has not been started.

##### Throws

Will throw an error if the database is not started.

##### Returns

[`Database`](Database.md)\<`T`\>

***

### dbCollections

#### Get Signature

> **get** `private` **dbCollections**(): \{ \[name in string \| number \| symbol\]: Collection\<Schema\<T\[name\]\>\> \}

Defined in: [ridb/src/index.ts:265](https://github.com/trust0-project/RIDB/blob/23b6db69eaeecdb007c5527c1028a5ec7519b6e7/packages/ridb/src/index.ts#L265)

Gets the collections from the database.

##### Returns

\{ \[name in string \| number \| symbol\]: Collection\<Schema\<T\[name\]\>\> \}

The collections object.

***

### dbName

#### Get Signature

> **get** `private` **dbName**(): `string`

Defined in: [ridb/src/index.ts:207](https://github.com/trust0-project/RIDB/blob/23b6db69eaeecdb007c5527c1028a5ec7519b6e7/packages/ridb/src/index.ts#L207)

##### Returns

`string`

***

### migrations

#### Get Signature

> **get** `private` **migrations**(): [`MigrationPathsForSchemas`](../type-aliases/MigrationPathsForSchemas.md)\<`T`\>

Defined in: [ridb/src/index.ts:215](https://github.com/trust0-project/RIDB/blob/23b6db69eaeecdb007c5527c1028a5ec7519b6e7/packages/ridb/src/index.ts#L215)

##### Returns

[`MigrationPathsForSchemas`](../type-aliases/MigrationPathsForSchemas.md)\<`T`\>

***

### plugins

#### Get Signature

> **get** `private` **plugins**(): *typeof* [`BasePlugin`](BasePlugin.md)[]

Defined in: [ridb/src/index.ts:219](https://github.com/trust0-project/RIDB/blob/23b6db69eaeecdb007c5527c1028a5ec7519b6e7/packages/ridb/src/index.ts#L219)

##### Returns

*typeof* [`BasePlugin`](BasePlugin.md)[]

***

### schemas

#### Get Signature

> **get** `private` **schemas**(): `T`

Defined in: [ridb/src/index.ts:211](https://github.com/trust0-project/RIDB/blob/23b6db69eaeecdb007c5527c1028a5ec7519b6e7/packages/ridb/src/index.ts#L211)

##### Returns

`T`

***

### useWorker

#### Get Signature

> **get** **useWorker**(): `boolean`

Defined in: [ridb/src/index.ts:223](https://github.com/trust0-project/RIDB/blob/23b6db69eaeecdb007c5527c1028a5ec7519b6e7/packages/ridb/src/index.ts#L223)

##### Returns

`boolean`

***

### worker

#### Get Signature

> **get** `private` **worker**(): `SharedWorker`

Defined in: [ridb/src/index.ts:253](https://github.com/trust0-project/RIDB/blob/23b6db69eaeecdb007c5527c1028a5ec7519b6e7/packages/ridb/src/index.ts#L253)

##### Returns

`SharedWorker`

***

### workerCollections

#### Get Signature

> **get** `private` **workerCollections**(): \{ \[name in string \| number \| symbol\]: Collection\<Schema\<T\[name\]\>\> \}

Defined in: [ridb/src/index.ts:269](https://github.com/trust0-project/RIDB/blob/23b6db69eaeecdb007c5527c1028a5ec7519b6e7/packages/ridb/src/index.ts#L269)

##### Returns

\{ \[name in string \| number \| symbol\]: Collection\<Schema\<T\[name\]\>\> \}

## Methods

### close()

> **close**(): `Promise`\<`void`\>

Defined in: [ridb/src/index.ts:388](https://github.com/trust0-project/RIDB/blob/23b6db69eaeecdb007c5527c1028a5ec7519b6e7/packages/ridb/src/index.ts#L388)

#### Returns

`Promise`\<`void`\>

***

### createDatabase()

> `private` **createDatabase**(`options`?): `Promise`\<[`Database`](Database.md)\<`T`\>\>

Defined in: [ridb/src/index.ts:328](https://github.com/trust0-project/RIDB/blob/23b6db69eaeecdb007c5527c1028a5ec7519b6e7/packages/ridb/src/index.ts#L328)

#### Parameters

##### options?

[`StartOptions`](../type-aliases/StartOptions.md)\<`T`\>

#### Returns

`Promise`\<[`Database`](Database.md)\<`T`\>\>

***

### createWorker()

> `private` **createWorker**(): `Promise`\<`SharedWorker`\>

Defined in: [ridb/src/index.ts:305](https://github.com/trust0-project/RIDB/blob/23b6db69eaeecdb007c5527c1028a5ec7519b6e7/packages/ridb/src/index.ts#L305)

#### Returns

`Promise`\<`SharedWorker`\>

***

### getStorageType()

> `private` **getStorageType**\<`T`\>(`storageType`): *typeof* [`InMemory`](InMemory.md)

Defined in: [ridb/src/index.ts:235](https://github.com/trust0-project/RIDB/blob/23b6db69eaeecdb007c5527c1028a5ec7519b6e7/packages/ridb/src/index.ts#L235)

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

Defined in: [ridb/src/index.ts:313](https://github.com/trust0-project/RIDB/blob/23b6db69eaeecdb007c5527c1028a5ec7519b6e7/packages/ridb/src/index.ts#L313)

#### Parameters

##### event

`MessageEvent`

#### Returns

`Promise`\<`void`\>

***

### start()

> **start**(`options`?): `Promise`\<`void`\>

Defined in: [ridb/src/index.ts:361](https://github.com/trust0-project/RIDB/blob/23b6db69eaeecdb007c5527c1028a5ec7519b6e7/packages/ridb/src/index.ts#L361)

Starts the database.

#### Parameters

##### options?

[`StartOptions`](../type-aliases/StartOptions.md)\<`T`\>

#### Returns

`Promise`\<`void`\>

A promise that resolves to the database instance.
