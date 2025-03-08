[**@trust0/ridb**](../README.md)

***

[@trust0/ridb](../README.md) / RIDB

# Class: RIDB\<T\>

Defined in: [ridb/src/index.ts:199](https://github.com/trust0-project/RIDB/blob/8f8fe2edeed75fca8df293b533a5cdcbd0518592/packages/ridb/src/index.ts#L199)

## Type Parameters

• **T** *extends* [`SchemaTypeRecord`](../type-aliases/SchemaTypeRecord.md) = [`SchemaTypeRecord`](../type-aliases/SchemaTypeRecord.md)

## Constructors

### new RIDB()

> **new RIDB**\<`T`\>(`options`): [`RIDB`](RIDB.md)\<`T`\>

Defined in: [ridb/src/index.ts:233](https://github.com/trust0-project/RIDB/blob/8f8fe2edeed75fca8df293b533a5cdcbd0518592/packages/ridb/src/index.ts#L233)

Creates an instance of RIDB.

#### Parameters

##### options

`DBOptions`\<`T`\>

#### Returns

[`RIDB`](RIDB.md)\<`T`\>

## Properties

### \_db

> `private` **\_db**: `undefined` \| [`Database`](Database.md)\<`T`\>

<<<<<<< HEAD
Defined in: [ridb/src/index.ts:192](https://github.com/trust0-project/RIDB/blob/95cbc5b53ed3fac8905847d17f3f95ff7c36dbcb/packages/ridb/src/index.ts#L192)
=======
Defined in: [ridb/src/index.ts:200](https://github.com/trust0-project/RIDB/blob/8f8fe2edeed75fca8df293b533a5cdcbd0518592/packages/ridb/src/index.ts#L200)

***

### \_sessionId

> `private` **\_sessionId**: `undefined` \| `string`

Defined in: [ridb/src/index.ts:202](https://github.com/trust0-project/RIDB/blob/8f8fe2edeed75fca8df293b533a5cdcbd0518592/packages/ridb/src/index.ts#L202)
>>>>>>> f496532 (chore(release): 1.3.2-rc.3 [skip ci])

***

### \_worker

> `private` **\_worker**: `undefined` \| `SharedWorker`

<<<<<<< HEAD
Defined in: [ridb/src/index.ts:193](https://github.com/trust0-project/RIDB/blob/95cbc5b53ed3fac8905847d17f3f95ff7c36dbcb/packages/ridb/src/index.ts#L193)
=======
Defined in: [ridb/src/index.ts:201](https://github.com/trust0-project/RIDB/blob/8f8fe2edeed75fca8df293b533a5cdcbd0518592/packages/ridb/src/index.ts#L201)
>>>>>>> f496532 (chore(release): 1.3.2-rc.3 [skip ci])

***

### options

> `private` **options**: `DBOptions`\<`T`\>

<<<<<<< HEAD
Defined in: [ridb/src/index.ts:224](https://github.com/trust0-project/RIDB/blob/95cbc5b53ed3fac8905847d17f3f95ff7c36dbcb/packages/ridb/src/index.ts#L224)
=======
Defined in: [ridb/src/index.ts:233](https://github.com/trust0-project/RIDB/blob/8f8fe2edeed75fca8df293b533a5cdcbd0518592/packages/ridb/src/index.ts#L233)
>>>>>>> f496532 (chore(release): 1.3.2-rc.3 [skip ci])

***

### pendingRequests

> `private` **pendingRequests**: `PendingRequests`

<<<<<<< HEAD
Defined in: [ridb/src/index.ts:196](https://github.com/trust0-project/RIDB/blob/95cbc5b53ed3fac8905847d17f3f95ff7c36dbcb/packages/ridb/src/index.ts#L196)
=======
Defined in: [ridb/src/index.ts:205](https://github.com/trust0-project/RIDB/blob/8f8fe2edeed75fca8df293b533a5cdcbd0518592/packages/ridb/src/index.ts#L205)
>>>>>>> f496532 (chore(release): 1.3.2-rc.3 [skip ci])

***

### started

> **started**: `boolean` = `false`

<<<<<<< HEAD
Defined in: [ridb/src/index.ts:194](https://github.com/trust0-project/RIDB/blob/95cbc5b53ed3fac8905847d17f3f95ff7c36dbcb/packages/ridb/src/index.ts#L194)
=======
Defined in: [ridb/src/index.ts:203](https://github.com/trust0-project/RIDB/blob/8f8fe2edeed75fca8df293b533a5cdcbd0518592/packages/ridb/src/index.ts#L203)
>>>>>>> f496532 (chore(release): 1.3.2-rc.3 [skip ci])

## Accessors

### collections

#### Get Signature

> **get** **collections**(): \{ \[name in string \| number \| symbol\]: Collection\<Schema\<T\[name\]\>\> \}

<<<<<<< HEAD
Defined in: [ridb/src/index.ts:293](https://github.com/trust0-project/RIDB/blob/95cbc5b53ed3fac8905847d17f3f95ff7c36dbcb/packages/ridb/src/index.ts#L293)
=======
Defined in: [ridb/src/index.ts:301](https://github.com/trust0-project/RIDB/blob/8f8fe2edeed75fca8df293b533a5cdcbd0518592/packages/ridb/src/index.ts#L301)
>>>>>>> f496532 (chore(release): 1.3.2-rc.3 [skip ci])

##### Returns

\{ \[name in string \| number \| symbol\]: Collection\<Schema\<T\[name\]\>\> \}

***

### db

#### Get Signature

> **get** `private` **db**(): [`Database`](Database.md)\<`T`\>

<<<<<<< HEAD
Defined in: [ridb/src/index.ts:237](https://github.com/trust0-project/RIDB/blob/95cbc5b53ed3fac8905847d17f3f95ff7c36dbcb/packages/ridb/src/index.ts#L237)
=======
Defined in: [ridb/src/index.ts:246](https://github.com/trust0-project/RIDB/blob/8f8fe2edeed75fca8df293b533a5cdcbd0518592/packages/ridb/src/index.ts#L246)
>>>>>>> f496532 (chore(release): 1.3.2-rc.3 [skip ci])

Gets the database instance. Throws an error if the database has not been started.

##### Throws

Will throw an error if the database is not started.

##### Returns

[`Database`](Database.md)\<`T`\>

***

### dbCollections

#### Get Signature

> **get** `private` **dbCollections**(): \{ \[name in string \| number \| symbol\]: Collection\<Schema\<T\[name\]\>\> \}

<<<<<<< HEAD
Defined in: [ridb/src/index.ts:256](https://github.com/trust0-project/RIDB/blob/95cbc5b53ed3fac8905847d17f3f95ff7c36dbcb/packages/ridb/src/index.ts#L256)
=======
Defined in: [ridb/src/index.ts:265](https://github.com/trust0-project/RIDB/blob/8f8fe2edeed75fca8df293b533a5cdcbd0518592/packages/ridb/src/index.ts#L265)
>>>>>>> f496532 (chore(release): 1.3.2-rc.3 [skip ci])

Gets the collections from the database.

##### Returns

\{ \[name in string \| number \| symbol\]: Collection\<Schema\<T\[name\]\>\> \}

The collections object.

***

### dbName

#### Get Signature

> **get** `private` **dbName**(): `string`

<<<<<<< HEAD
Defined in: [ridb/src/index.ts:198](https://github.com/trust0-project/RIDB/blob/95cbc5b53ed3fac8905847d17f3f95ff7c36dbcb/packages/ridb/src/index.ts#L198)
=======
Defined in: [ridb/src/index.ts:207](https://github.com/trust0-project/RIDB/blob/8f8fe2edeed75fca8df293b533a5cdcbd0518592/packages/ridb/src/index.ts#L207)
>>>>>>> f496532 (chore(release): 1.3.2-rc.3 [skip ci])

##### Returns

`string`

***

### migrations

#### Get Signature

> **get** `private` **migrations**(): [`MigrationPathsForSchemas`](../type-aliases/MigrationPathsForSchemas.md)\<`T`\>

<<<<<<< HEAD
Defined in: [ridb/src/index.ts:206](https://github.com/trust0-project/RIDB/blob/95cbc5b53ed3fac8905847d17f3f95ff7c36dbcb/packages/ridb/src/index.ts#L206)
=======
Defined in: [ridb/src/index.ts:215](https://github.com/trust0-project/RIDB/blob/8f8fe2edeed75fca8df293b533a5cdcbd0518592/packages/ridb/src/index.ts#L215)
>>>>>>> f496532 (chore(release): 1.3.2-rc.3 [skip ci])

##### Returns

[`MigrationPathsForSchemas`](../type-aliases/MigrationPathsForSchemas.md)\<`T`\>

***

### plugins

#### Get Signature

> **get** `private` **plugins**(): *typeof* [`BasePlugin`](BasePlugin.md)[]

<<<<<<< HEAD
Defined in: [ridb/src/index.ts:210](https://github.com/trust0-project/RIDB/blob/95cbc5b53ed3fac8905847d17f3f95ff7c36dbcb/packages/ridb/src/index.ts#L210)
=======
Defined in: [ridb/src/index.ts:219](https://github.com/trust0-project/RIDB/blob/8f8fe2edeed75fca8df293b533a5cdcbd0518592/packages/ridb/src/index.ts#L219)
>>>>>>> f496532 (chore(release): 1.3.2-rc.3 [skip ci])

##### Returns

*typeof* [`BasePlugin`](BasePlugin.md)[]

***

### schemas

#### Get Signature

> **get** `private` **schemas**(): `T`

<<<<<<< HEAD
Defined in: [ridb/src/index.ts:202](https://github.com/trust0-project/RIDB/blob/95cbc5b53ed3fac8905847d17f3f95ff7c36dbcb/packages/ridb/src/index.ts#L202)
=======
Defined in: [ridb/src/index.ts:211](https://github.com/trust0-project/RIDB/blob/8f8fe2edeed75fca8df293b533a5cdcbd0518592/packages/ridb/src/index.ts#L211)
>>>>>>> f496532 (chore(release): 1.3.2-rc.3 [skip ci])

##### Returns

`T`

***

### useWorker

#### Get Signature

> **get** **useWorker**(): `boolean`

<<<<<<< HEAD
Defined in: [ridb/src/index.ts:214](https://github.com/trust0-project/RIDB/blob/95cbc5b53ed3fac8905847d17f3f95ff7c36dbcb/packages/ridb/src/index.ts#L214)
=======
Defined in: [ridb/src/index.ts:223](https://github.com/trust0-project/RIDB/blob/8f8fe2edeed75fca8df293b533a5cdcbd0518592/packages/ridb/src/index.ts#L223)
>>>>>>> f496532 (chore(release): 1.3.2-rc.3 [skip ci])

##### Returns

`boolean`

***

### worker

#### Get Signature

> **get** `private` **worker**(): `SharedWorker`

<<<<<<< HEAD
Defined in: [ridb/src/index.ts:244](https://github.com/trust0-project/RIDB/blob/95cbc5b53ed3fac8905847d17f3f95ff7c36dbcb/packages/ridb/src/index.ts#L244)
=======
Defined in: [ridb/src/index.ts:253](https://github.com/trust0-project/RIDB/blob/8f8fe2edeed75fca8df293b533a5cdcbd0518592/packages/ridb/src/index.ts#L253)
>>>>>>> f496532 (chore(release): 1.3.2-rc.3 [skip ci])

##### Returns

`SharedWorker`

***

### workerCollections

#### Get Signature

> **get** `private` **workerCollections**(): \{ \[name in string \| number \| symbol\]: Collection\<Schema\<T\[name\]\>\> \}

<<<<<<< HEAD
Defined in: [ridb/src/index.ts:260](https://github.com/trust0-project/RIDB/blob/95cbc5b53ed3fac8905847d17f3f95ff7c36dbcb/packages/ridb/src/index.ts#L260)
=======
Defined in: [ridb/src/index.ts:269](https://github.com/trust0-project/RIDB/blob/8f8fe2edeed75fca8df293b533a5cdcbd0518592/packages/ridb/src/index.ts#L269)
>>>>>>> f496532 (chore(release): 1.3.2-rc.3 [skip ci])

##### Returns

\{ \[name in string \| number \| symbol\]: Collection\<Schema\<T\[name\]\>\> \}

## Methods

### close()

> **close**(): `Promise`\<`void`\>

<<<<<<< HEAD
Defined in: [ridb/src/index.ts:380](https://github.com/trust0-project/RIDB/blob/95cbc5b53ed3fac8905847d17f3f95ff7c36dbcb/packages/ridb/src/index.ts#L380)
=======
Defined in: [ridb/src/index.ts:388](https://github.com/trust0-project/RIDB/blob/8f8fe2edeed75fca8df293b533a5cdcbd0518592/packages/ridb/src/index.ts#L388)
>>>>>>> f496532 (chore(release): 1.3.2-rc.3 [skip ci])

#### Returns

`Promise`\<`void`\>

***

### createDatabase()

> `private` **createDatabase**(`options`?): `Promise`\<[`Database`](Database.md)\<`T`\>\>

<<<<<<< HEAD
Defined in: [ridb/src/index.ts:320](https://github.com/trust0-project/RIDB/blob/95cbc5b53ed3fac8905847d17f3f95ff7c36dbcb/packages/ridb/src/index.ts#L320)
=======
Defined in: [ridb/src/index.ts:328](https://github.com/trust0-project/RIDB/blob/8f8fe2edeed75fca8df293b533a5cdcbd0518592/packages/ridb/src/index.ts#L328)
>>>>>>> f496532 (chore(release): 1.3.2-rc.3 [skip ci])

#### Parameters

##### options?

[`StartOptions`](../type-aliases/StartOptions.md)\<`T`\>

#### Returns

`Promise`\<[`Database`](Database.md)\<`T`\>\>

***

### createWorker()

> `private` **createWorker**(): `Promise`\<`SharedWorker`\>

<<<<<<< HEAD
Defined in: [ridb/src/index.ts:297](https://github.com/trust0-project/RIDB/blob/95cbc5b53ed3fac8905847d17f3f95ff7c36dbcb/packages/ridb/src/index.ts#L297)
=======
Defined in: [ridb/src/index.ts:305](https://github.com/trust0-project/RIDB/blob/8f8fe2edeed75fca8df293b533a5cdcbd0518592/packages/ridb/src/index.ts#L305)
>>>>>>> f496532 (chore(release): 1.3.2-rc.3 [skip ci])

#### Returns

`Promise`\<`SharedWorker`\>

***

### getStorageType()

> `private` **getStorageType**\<`T`\>(`storageType`): *typeof* [`InMemory`](InMemory.md)

<<<<<<< HEAD
Defined in: [ridb/src/index.ts:226](https://github.com/trust0-project/RIDB/blob/95cbc5b53ed3fac8905847d17f3f95ff7c36dbcb/packages/ridb/src/index.ts#L226)
=======
Defined in: [ridb/src/index.ts:235](https://github.com/trust0-project/RIDB/blob/8f8fe2edeed75fca8df293b533a5cdcbd0518592/packages/ridb/src/index.ts#L235)
>>>>>>> f496532 (chore(release): 1.3.2-rc.3 [skip ci])

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

<<<<<<< HEAD
Defined in: [ridb/src/index.ts:305](https://github.com/trust0-project/RIDB/blob/95cbc5b53ed3fac8905847d17f3f95ff7c36dbcb/packages/ridb/src/index.ts#L305)
=======
Defined in: [ridb/src/index.ts:313](https://github.com/trust0-project/RIDB/blob/8f8fe2edeed75fca8df293b533a5cdcbd0518592/packages/ridb/src/index.ts#L313)
>>>>>>> f496532 (chore(release): 1.3.2-rc.3 [skip ci])

#### Parameters

##### event

`MessageEvent`

#### Returns

`Promise`\<`void`\>

***

### start()

> **start**(`options`?): `Promise`\<`void`\>

<<<<<<< HEAD
Defined in: [ridb/src/index.ts:353](https://github.com/trust0-project/RIDB/blob/95cbc5b53ed3fac8905847d17f3f95ff7c36dbcb/packages/ridb/src/index.ts#L353)
=======
Defined in: [ridb/src/index.ts:361](https://github.com/trust0-project/RIDB/blob/8f8fe2edeed75fca8df293b533a5cdcbd0518592/packages/ridb/src/index.ts#L361)
>>>>>>> f496532 (chore(release): 1.3.2-rc.3 [skip ci])

Starts the database.

#### Parameters

##### options?

[`StartOptions`](../type-aliases/StartOptions.md)\<`T`\>

#### Returns

`Promise`\<`void`\>

A promise that resolves to the database instance.
