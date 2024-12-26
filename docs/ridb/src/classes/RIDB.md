[**trust0**](../../../README.md)

***

[trust0](../../../README.md) / [ridb/src](../README.md) / RIDB

# Class: RIDB\<T\>

## Type Parameters

• **T** *extends* `RIDBTypes.SchemaTypeRecord` = `RIDBTypes.SchemaTypeRecord`

## Constructors

### new RIDB()

> **new RIDB**\<`T`\>(`options`): [`RIDB`](RIDB.md)\<`T`\>

Creates an instance of RIDB.

#### Parameters

##### options

`object` & `MigrationsParameter`\<`T`\>

#### Returns

[`RIDB`](RIDB.md)\<`T`\>

#### Defined in

[packages/ridb/src/index.ts:144](https://github.com/elribonazo/RIDB/blob/974d49d3847a2b9cdd8e4a97073f7824e8a3a1db/packages/ridb/src/index.ts#L144)

## Properties

### \_db

> `private` **\_db**: `Database`\<`T`\>

#### Defined in

[packages/ridb/src/index.ts:137](https://github.com/elribonazo/RIDB/blob/974d49d3847a2b9cdd8e4a97073f7824e8a3a1db/packages/ridb/src/index.ts#L137)

***

### dbName

> `private` **dbName**: `string`

#### Defined in

[packages/ridb/src/index.ts:138](https://github.com/elribonazo/RIDB/blob/974d49d3847a2b9cdd8e4a97073f7824e8a3a1db/packages/ridb/src/index.ts#L138)

***

### migrations

> `private` **migrations**: `MigrationPathsForSchemas`\<`T`\>

#### Defined in

[packages/ridb/src/index.ts:135](https://github.com/elribonazo/RIDB/blob/974d49d3847a2b9cdd8e4a97073f7824e8a3a1db/packages/ridb/src/index.ts#L135)

***

### plugins

> `private` **plugins**: *typeof* `BasePlugin`[] = `[]`

#### Defined in

[packages/ridb/src/index.ts:136](https://github.com/elribonazo/RIDB/blob/974d49d3847a2b9cdd8e4a97073f7824e8a3a1db/packages/ridb/src/index.ts#L136)

***

### schemas

> `private` **schemas**: `T`

#### Defined in

[packages/ridb/src/index.ts:134](https://github.com/elribonazo/RIDB/blob/974d49d3847a2b9cdd8e4a97073f7824e8a3a1db/packages/ridb/src/index.ts#L134)

## Accessors

### collections

#### Get Signature

> **get** **collections**(): \{ \[name in string \| number \| symbol\]: Collection\<Schema\<T\[name\]\>\> \}

Gets the collections from the database.

##### Returns

\{ \[name in string \| number \| symbol\]: Collection\<Schema\<T\[name\]\>\> \}

The collections object.

#### Defined in

[packages/ridb/src/index.ts:192](https://github.com/elribonazo/RIDB/blob/974d49d3847a2b9cdd8e4a97073f7824e8a3a1db/packages/ridb/src/index.ts#L192)

***

### db

#### Get Signature

> **get** `private` **db**(): `Database`\<`T`\>

Gets the database instance. Throws an error if the database has not been started.

##### Throws

Will throw an error if the database is not started.

##### Returns

`Database`\<`T`\>

#### Defined in

[packages/ridb/src/index.ts:177](https://github.com/elribonazo/RIDB/blob/974d49d3847a2b9cdd8e4a97073f7824e8a3a1db/packages/ridb/src/index.ts#L177)

***

### started

#### Get Signature

> **get** **started**(): `boolean`

##### Returns

`boolean`

#### Defined in

[packages/ridb/src/index.ts:184](https://github.com/elribonazo/RIDB/blob/974d49d3847a2b9cdd8e4a97073f7824e8a3a1db/packages/ridb/src/index.ts#L184)

## Methods

### close()

> **close**(): `Promise`\<`void`\>

#### Returns

`Promise`\<`void`\>

#### Defined in

[packages/ridb/src/index.ts:253](https://github.com/elribonazo/RIDB/blob/974d49d3847a2b9cdd8e4a97073f7824e8a3a1db/packages/ridb/src/index.ts#L253)

***

### getStorageType()

> `private` **getStorageType**\<`T`\>(`storageType`): *typeof* `IndexDB`

#### Type Parameters

• **T** *extends* [`StorageType`](../enumerations/StorageType.md)

#### Parameters

##### storageType

`T`

#### Returns

*typeof* `IndexDB`

#### Defined in

[packages/ridb/src/index.ts:165](https://github.com/elribonazo/RIDB/blob/974d49d3847a2b9cdd8e4a97073f7824e8a3a1db/packages/ridb/src/index.ts#L165)

***

### start()

> **start**(`options`?): `Promise`\<`Database`\<`T`\>\>

Starts the database.

#### Parameters

##### options?

[`StartOptions`](../type-aliases/StartOptions.md)\<`T`\>

#### Returns

`Promise`\<`Database`\<`T`\>\>

A promise that resolves to the database instance.

#### Defined in

[packages/ridb/src/index.ts:215](https://github.com/elribonazo/RIDB/blob/974d49d3847a2b9cdd8e4a97073f7824e8a3a1db/packages/ridb/src/index.ts#L215)

***

### load()

> `private` `static` **load**(): `Promise`\<`__module`\>

Loads the RIDB Rust module.

#### Returns

`Promise`\<`__module`\>

A promise that resolves to the RIDB Rust module.

#### Defined in

[packages/ridb/src/index.ts:201](https://github.com/elribonazo/RIDB/blob/974d49d3847a2b9cdd8e4a97073f7824e8a3a1db/packages/ridb/src/index.ts#L201)
