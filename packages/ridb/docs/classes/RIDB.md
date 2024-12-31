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

[ridb/src/index.ts:179](https://github.com/elribonazo/RIDB/blob/c2facd49c4ac92f89675d530f538af834dbd30a5/packages/ridb/src/index.ts#L179)

## Properties

### \_db

> `private` **\_db**: `undefined` \| [`Database`](Database.md)\<`T`\>

#### Defined in

[ridb/src/index.ts:172](https://github.com/elribonazo/RIDB/blob/c2facd49c4ac92f89675d530f538af834dbd30a5/packages/ridb/src/index.ts#L172)

***

### dbName

> `private` **dbName**: `string`

#### Defined in

[ridb/src/index.ts:173](https://github.com/elribonazo/RIDB/blob/c2facd49c4ac92f89675d530f538af834dbd30a5/packages/ridb/src/index.ts#L173)

***

### migrations

> `private` **migrations**: [`MigrationPathsForSchemas`](../type-aliases/MigrationPathsForSchemas.md)\<`T`\>

#### Defined in

[ridb/src/index.ts:170](https://github.com/elribonazo/RIDB/blob/c2facd49c4ac92f89675d530f538af834dbd30a5/packages/ridb/src/index.ts#L170)

***

### plugins

> `private` **plugins**: *typeof* [`BasePlugin`](BasePlugin.md)[] = `[]`

#### Defined in

[ridb/src/index.ts:171](https://github.com/elribonazo/RIDB/blob/c2facd49c4ac92f89675d530f538af834dbd30a5/packages/ridb/src/index.ts#L171)

***

### schemas

> `private` **schemas**: `T`

#### Defined in

[ridb/src/index.ts:169](https://github.com/elribonazo/RIDB/blob/c2facd49c4ac92f89675d530f538af834dbd30a5/packages/ridb/src/index.ts#L169)

## Accessors

### collections

#### Get Signature

> **get** **collections**(): \{ \[name in string \| number \| symbol\]: Collection\<Schema\<T\[name\]\>\> \}

Gets the collections from the database.

##### Returns

\{ \[name in string \| number \| symbol\]: Collection\<Schema\<T\[name\]\>\> \}

The collections object.

#### Defined in

[ridb/src/index.ts:227](https://github.com/elribonazo/RIDB/blob/c2facd49c4ac92f89675d530f538af834dbd30a5/packages/ridb/src/index.ts#L227)

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

[ridb/src/index.ts:212](https://github.com/elribonazo/RIDB/blob/c2facd49c4ac92f89675d530f538af834dbd30a5/packages/ridb/src/index.ts#L212)

***

### started

#### Get Signature

> **get** **started**(): `boolean`

##### Returns

`boolean`

#### Defined in

[ridb/src/index.ts:219](https://github.com/elribonazo/RIDB/blob/c2facd49c4ac92f89675d530f538af834dbd30a5/packages/ridb/src/index.ts#L219)

## Methods

### close()

> **close**(): `Promise`\<`void`\>

#### Returns

`Promise`\<`void`\>

#### Defined in

[ridb/src/index.ts:287](https://github.com/elribonazo/RIDB/blob/c2facd49c4ac92f89675d530f538af834dbd30a5/packages/ridb/src/index.ts#L287)

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

[ridb/src/index.ts:200](https://github.com/elribonazo/RIDB/blob/c2facd49c4ac92f89675d530f538af834dbd30a5/packages/ridb/src/index.ts#L200)

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

[ridb/src/index.ts:250](https://github.com/elribonazo/RIDB/blob/c2facd49c4ac92f89675d530f538af834dbd30a5/packages/ridb/src/index.ts#L250)

***

### load()

> `private` `static` **load**(): `Promise`\<`__module`\>

Loads the RIDB Rust module.

#### Returns

`Promise`\<`__module`\>

A promise that resolves to the RIDB Rust module.

#### Defined in

[ridb/src/index.ts:236](https://github.com/elribonazo/RIDB/blob/c2facd49c4ac92f89675d530f538af834dbd30a5/packages/ridb/src/index.ts#L236)
