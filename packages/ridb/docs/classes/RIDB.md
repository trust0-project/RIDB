[**@trust0/ridb**](../README.md)

***

[@trust0/ridb](../README.md) / RIDB

# Class: RIDB\<T\>

## Type Parameters

• **T** *extends* [`SchemaTypeRecord`](../type-aliases/SchemaTypeRecord.md) = [`SchemaTypeRecord`](../type-aliases/SchemaTypeRecord.md)

## Constructors

### new RIDB()

> **new RIDB**\<`T`\>(`options`): [`RIDB`](RIDB.md)\<`T`\>

Creates an instance of RIDB.

#### Parameters

##### options

`object` & [`MigrationsParameter`](../type-aliases/MigrationsParameter.md)\<`T`\>

#### Returns

[`RIDB`](RIDB.md)\<`T`\>

#### Defined in

[packages/ridb/src/index.ts:184](https://github.com/elribonazo/RIDB/blob/4b743397ab8270ad6b4bb904610668f22eb08c58/packages/ridb/src/index.ts#L184)

## Properties

### \_db

> `private` **\_db**: `undefined` \| [`Database`](Database.md)\<`T`\>

#### Defined in

[packages/ridb/src/index.ts:177](https://github.com/elribonazo/RIDB/blob/4b743397ab8270ad6b4bb904610668f22eb08c58/packages/ridb/src/index.ts#L177)

***

### dbName

> `private` **dbName**: `string`

#### Defined in

[packages/ridb/src/index.ts:178](https://github.com/elribonazo/RIDB/blob/4b743397ab8270ad6b4bb904610668f22eb08c58/packages/ridb/src/index.ts#L178)

***

### migrations

> `private` **migrations**: [`MigrationPathsForSchemas`](../type-aliases/MigrationPathsForSchemas.md)\<`T`\>

#### Defined in

[packages/ridb/src/index.ts:175](https://github.com/elribonazo/RIDB/blob/4b743397ab8270ad6b4bb904610668f22eb08c58/packages/ridb/src/index.ts#L175)

***

### plugins

> `private` **plugins**: *typeof* [`BasePlugin`](BasePlugin.md)[] = `[]`

#### Defined in

[packages/ridb/src/index.ts:176](https://github.com/elribonazo/RIDB/blob/4b743397ab8270ad6b4bb904610668f22eb08c58/packages/ridb/src/index.ts#L176)

***

### schemas

> `private` **schemas**: `T`

#### Defined in

[packages/ridb/src/index.ts:174](https://github.com/elribonazo/RIDB/blob/4b743397ab8270ad6b4bb904610668f22eb08c58/packages/ridb/src/index.ts#L174)

## Accessors

### collections

#### Get Signature

> **get** **collections**(): \{ \[name in string \| number \| symbol\]: Collection\<Schema\<T\[name\]\>\> \}

Gets the collections from the database.

##### Returns

\{ \[name in string \| number \| symbol\]: Collection\<Schema\<T\[name\]\>\> \}

The collections object.

#### Defined in

[packages/ridb/src/index.ts:232](https://github.com/elribonazo/RIDB/blob/4b743397ab8270ad6b4bb904610668f22eb08c58/packages/ridb/src/index.ts#L232)

***

### db

#### Get Signature

> **get** `private` **db**(): [`Database`](Database.md)\<`T`\>

Gets the database instance. Throws an error if the database has not been started.

##### Throws

Will throw an error if the database is not started.

##### Returns

[`Database`](Database.md)\<`T`\>

#### Defined in

[packages/ridb/src/index.ts:217](https://github.com/elribonazo/RIDB/blob/4b743397ab8270ad6b4bb904610668f22eb08c58/packages/ridb/src/index.ts#L217)

***

### started

#### Get Signature

> **get** **started**(): `boolean`

##### Returns

`boolean`

#### Defined in

[packages/ridb/src/index.ts:224](https://github.com/elribonazo/RIDB/blob/4b743397ab8270ad6b4bb904610668f22eb08c58/packages/ridb/src/index.ts#L224)

## Methods

### close()

> **close**(): `Promise`\<`void`\>

#### Returns

`Promise`\<`void`\>

#### Defined in

[packages/ridb/src/index.ts:293](https://github.com/elribonazo/RIDB/blob/4b743397ab8270ad6b4bb904610668f22eb08c58/packages/ridb/src/index.ts#L293)

***

### getStorageType()

> `private` **getStorageType**\<`T`\>(`storageType`): *typeof* [`IndexDB`](IndexDB.md)

#### Type Parameters

• **T** *extends* [`StorageType`](../enumerations/StorageType.md)

#### Parameters

##### storageType

`T`

#### Returns

*typeof* [`IndexDB`](IndexDB.md)

#### Defined in

[packages/ridb/src/index.ts:205](https://github.com/elribonazo/RIDB/blob/4b743397ab8270ad6b4bb904610668f22eb08c58/packages/ridb/src/index.ts#L205)

***

### start()

> **start**(`options`?): `Promise`\<[`Database`](Database.md)\<`T`\>\>

Starts the database.

#### Parameters

##### options?

[`StartOptions`](../type-aliases/StartOptions.md)\<`T`\>

#### Returns

`Promise`\<[`Database`](Database.md)\<`T`\>\>

A promise that resolves to the database instance.

#### Defined in

[packages/ridb/src/index.ts:255](https://github.com/elribonazo/RIDB/blob/4b743397ab8270ad6b4bb904610668f22eb08c58/packages/ridb/src/index.ts#L255)

***

### load()

> `private` `static` **load**(): `Promise`\<`__module`\>

Loads the RIDB Rust module.

#### Returns

`Promise`\<`__module`\>

A promise that resolves to the RIDB Rust module.

#### Defined in

[packages/ridb/src/index.ts:241](https://github.com/elribonazo/RIDB/blob/4b743397ab8270ad6b4bb904610668f22eb08c58/packages/ridb/src/index.ts#L241)
