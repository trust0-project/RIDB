[**@trust0/ridb**](../README.md)

***

[@trust0/ridb](../README.md) / RIDB

# Class: RIDB\<T\>

Defined in: [ridb/src/index.ts:191](https://github.com/trust0-project/RIDB/blob/f5856b3eb59ea4bab3c7b8367b0d008033fed532/packages/ridb/src/index.ts#L191)

## Type Parameters

• **T** *extends* [`SchemaTypeRecord`](../type-aliases/SchemaTypeRecord.md) = [`SchemaTypeRecord`](../type-aliases/SchemaTypeRecord.md)

## Constructors

### new RIDB()

> **new RIDB**\<`T`\>(`options`): [`RIDB`](RIDB.md)\<`T`\>

Defined in: [ridb/src/index.ts:224](https://github.com/trust0-project/RIDB/blob/f5856b3eb59ea4bab3c7b8367b0d008033fed532/packages/ridb/src/index.ts#L224)

Creates an instance of RIDB.

#### Parameters

##### options

`DBOptions`\<`T`\>

#### Returns

[`RIDB`](RIDB.md)\<`T`\>

## Properties

### \_db

> `private` **\_db**: `undefined` \| [`Database`](Database.md)\<`T`\>

Defined in: [ridb/src/index.ts:192](https://github.com/trust0-project/RIDB/blob/f5856b3eb59ea4bab3c7b8367b0d008033fed532/packages/ridb/src/index.ts#L192)

***

### \_worker

> `private` **\_worker**: `undefined` \| `SharedWorker`

Defined in: [ridb/src/index.ts:193](https://github.com/trust0-project/RIDB/blob/f5856b3eb59ea4bab3c7b8367b0d008033fed532/packages/ridb/src/index.ts#L193)

***

### options

> `private` **options**: `DBOptions`\<`T`\>

Defined in: [ridb/src/index.ts:224](https://github.com/trust0-project/RIDB/blob/f5856b3eb59ea4bab3c7b8367b0d008033fed532/packages/ridb/src/index.ts#L224)

***

### pendingRequests

> `private` **pendingRequests**: `PendingRequests`

Defined in: [ridb/src/index.ts:196](https://github.com/trust0-project/RIDB/blob/f5856b3eb59ea4bab3c7b8367b0d008033fed532/packages/ridb/src/index.ts#L196)

***

### started

> **started**: `boolean` = `false`

Defined in: [ridb/src/index.ts:194](https://github.com/trust0-project/RIDB/blob/f5856b3eb59ea4bab3c7b8367b0d008033fed532/packages/ridb/src/index.ts#L194)

## Accessors

### collections

#### Get Signature

> **get** **collections**(): \{ \[name in string \| number \| symbol\]: Collection\<Schema\<T\[name\]\>\> \}

Defined in: [ridb/src/index.ts:293](https://github.com/trust0-project/RIDB/blob/f5856b3eb59ea4bab3c7b8367b0d008033fed532/packages/ridb/src/index.ts#L293)

##### Returns

\{ \[name in string \| number \| symbol\]: Collection\<Schema\<T\[name\]\>\> \}

***

### db

#### Get Signature

> **get** `private` **db**(): [`Database`](Database.md)\<`T`\>

Defined in: [ridb/src/index.ts:237](https://github.com/trust0-project/RIDB/blob/f5856b3eb59ea4bab3c7b8367b0d008033fed532/packages/ridb/src/index.ts#L237)

Gets the database instance. Throws an error if the database has not been started.

##### Throws

Will throw an error if the database is not started.

##### Returns

[`Database`](Database.md)\<`T`\>

***

### dbCollections

#### Get Signature

> **get** `private` **dbCollections**(): \{ \[name in string \| number \| symbol\]: Collection\<Schema\<T\[name\]\>\> \}

Defined in: [ridb/src/index.ts:256](https://github.com/trust0-project/RIDB/blob/f5856b3eb59ea4bab3c7b8367b0d008033fed532/packages/ridb/src/index.ts#L256)

Gets the collections from the database.

##### Returns

\{ \[name in string \| number \| symbol\]: Collection\<Schema\<T\[name\]\>\> \}

The collections object.

***

### dbName

#### Get Signature

> **get** `private` **dbName**(): `string`

Defined in: [ridb/src/index.ts:198](https://github.com/trust0-project/RIDB/blob/f5856b3eb59ea4bab3c7b8367b0d008033fed532/packages/ridb/src/index.ts#L198)

##### Returns

`string`

***

### migrations

#### Get Signature

> **get** `private` **migrations**(): [`MigrationPathsForSchemas`](../type-aliases/MigrationPathsForSchemas.md)\<`T`\>

Defined in: [ridb/src/index.ts:206](https://github.com/trust0-project/RIDB/blob/f5856b3eb59ea4bab3c7b8367b0d008033fed532/packages/ridb/src/index.ts#L206)

##### Returns

[`MigrationPathsForSchemas`](../type-aliases/MigrationPathsForSchemas.md)\<`T`\>

***

### plugins

#### Get Signature

> **get** `private` **plugins**(): *typeof* [`BasePlugin`](BasePlugin.md)[]

Defined in: [ridb/src/index.ts:210](https://github.com/trust0-project/RIDB/blob/f5856b3eb59ea4bab3c7b8367b0d008033fed532/packages/ridb/src/index.ts#L210)

##### Returns

*typeof* [`BasePlugin`](BasePlugin.md)[]

***

### schemas

#### Get Signature

> **get** `private` **schemas**(): `T`

Defined in: [ridb/src/index.ts:202](https://github.com/trust0-project/RIDB/blob/f5856b3eb59ea4bab3c7b8367b0d008033fed532/packages/ridb/src/index.ts#L202)

##### Returns

`T`

***

### useWorker

#### Get Signature

> **get** **useWorker**(): `boolean`

Defined in: [ridb/src/index.ts:214](https://github.com/trust0-project/RIDB/blob/f5856b3eb59ea4bab3c7b8367b0d008033fed532/packages/ridb/src/index.ts#L214)

##### Returns

`boolean`

***

### worker

#### Get Signature

> **get** `private` **worker**(): `SharedWorker`

Defined in: [ridb/src/index.ts:244](https://github.com/trust0-project/RIDB/blob/f5856b3eb59ea4bab3c7b8367b0d008033fed532/packages/ridb/src/index.ts#L244)

##### Returns

`SharedWorker`

***

### workerCollections

#### Get Signature

> **get** `private` **workerCollections**(): \{ \[name in string \| number \| symbol\]: Collection\<Schema\<T\[name\]\>\> \}

Defined in: [ridb/src/index.ts:260](https://github.com/trust0-project/RIDB/blob/f5856b3eb59ea4bab3c7b8367b0d008033fed532/packages/ridb/src/index.ts#L260)

##### Returns

\{ \[name in string \| number \| symbol\]: Collection\<Schema\<T\[name\]\>\> \}

## Methods

### close()

> **close**(): `Promise`\<`void`\>

Defined in: [ridb/src/index.ts:380](https://github.com/trust0-project/RIDB/blob/f5856b3eb59ea4bab3c7b8367b0d008033fed532/packages/ridb/src/index.ts#L380)

#### Returns

`Promise`\<`void`\>

***

### createDatabase()

> `private` **createDatabase**(`options`?): `Promise`\<[`Database`](Database.md)\<`T`\>\>

Defined in: [ridb/src/index.ts:320](https://github.com/trust0-project/RIDB/blob/f5856b3eb59ea4bab3c7b8367b0d008033fed532/packages/ridb/src/index.ts#L320)

#### Parameters

##### options?

[`StartOptions`](../type-aliases/StartOptions.md)\<`T`\>

#### Returns

`Promise`\<[`Database`](Database.md)\<`T`\>\>

***

### createWorker()

> `private` **createWorker**(): `Promise`\<`SharedWorker`\>

Defined in: [ridb/src/index.ts:297](https://github.com/trust0-project/RIDB/blob/f5856b3eb59ea4bab3c7b8367b0d008033fed532/packages/ridb/src/index.ts#L297)

#### Returns

`Promise`\<`SharedWorker`\>

***

### getStorageType()

> `private` **getStorageType**\<`T`\>(`storageType`): *typeof* [`InMemory`](InMemory.md)

Defined in: [ridb/src/index.ts:226](https://github.com/trust0-project/RIDB/blob/f5856b3eb59ea4bab3c7b8367b0d008033fed532/packages/ridb/src/index.ts#L226)

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

Defined in: [ridb/src/index.ts:305](https://github.com/trust0-project/RIDB/blob/f5856b3eb59ea4bab3c7b8367b0d008033fed532/packages/ridb/src/index.ts#L305)

#### Parameters

##### event

`MessageEvent`

#### Returns

`Promise`\<`void`\>

***

### start()

> **start**(`options`?): `Promise`\<`void`\>

Defined in: [ridb/src/index.ts:353](https://github.com/trust0-project/RIDB/blob/f5856b3eb59ea4bab3c7b8367b0d008033fed532/packages/ridb/src/index.ts#L353)

Starts the database.

#### Parameters

##### options?

[`StartOptions`](../type-aliases/StartOptions.md)\<`T`\>

#### Returns

`Promise`\<`void`\>

A promise that resolves to the database instance.
