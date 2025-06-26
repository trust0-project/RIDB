[**@trust0/ridb-core**](../README.md)

***

[@trust0/ridb-core](../README.md) / Database

# Class: Database

Defined in: ridb\_core.js:938

Represents a database with collections of documents.

## Constructors

### Constructor

> **new Database**(): `Database`

#### Returns

`Database`

## Properties

### \_\_wbg\_ptr

> **\_\_wbg\_ptr**: `undefined` \| `number`

Defined in: ridb\_core.js:950

## Accessors

### collections

#### Get Signature

> **get** **collections**(): `object`

Defined in: ridb\_core.js:1001

Retrieves the collections in the database.

This function returns an `Object` containing the collections.

# Returns

* `Result<Object, JsValue>` - A result containing an `Object` with the collections or an error.

##### Returns

`object`

***

### started

#### Get Signature

> **get** **started**(): `boolean`

Defined in: ridb\_core.js:977

##### Returns

`boolean`

## Methods

### \_\_destroy\_into\_raw()

> **\_\_destroy\_into\_raw**(): `undefined` \| `number`

Defined in: ridb\_core.js:948

#### Returns

`undefined` \| `number`

***

### authenticate()

> **authenticate**(`password`): `Promise`\<`boolean`\>

Defined in: ridb\_core.js:985

#### Parameters

##### password

`string`

#### Returns

`Promise`\<`boolean`\>

***

### close()

> **close**(): `Promise`\<`any`\>

Defined in: ridb\_core.js:969

#### Returns

`Promise`\<`any`\>

***

### free()

> **free**(): `void`

Defined in: ridb\_core.js:955

#### Returns

`void`

***

### start()

> **start**(): `Promise`\<`any`\>

Defined in: ridb\_core.js:962

#### Returns

`Promise`\<`any`\>

***

### \_\_wrap()

> `static` **\_\_wrap**(`ptr`): `any`

Defined in: ridb\_core.js:940

#### Parameters

##### ptr

`any`

#### Returns

`any`

***

### create()

> `static` **create**(`db_name`, `schemas_js`, `migrations_js`, `plugins`, `module`, `password?`, `storage?`): `Promise`\<`Database`\>

Defined in: ridb\_core.js:1026

#### Parameters

##### db\_name

`string`

##### schemas\_js

`object`

##### migrations\_js

`object`

##### plugins

`any`[]

##### module

`any`

##### password?

`string`

##### storage?

`any`

#### Returns

`Promise`\<`Database`\>
