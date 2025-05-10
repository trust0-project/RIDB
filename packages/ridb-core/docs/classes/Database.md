[**@trust0/ridb-core**](../README.md)

***

[@trust0/ridb-core](../README.md) / Database

# Class: Database

Defined in: ridb\_core.js:937

Represents a database with collections of documents.

## Constructors

### Constructor

> **new Database**(): `Database`

#### Returns

`Database`

## Properties

### \_\_wbg\_ptr

> **\_\_wbg\_ptr**: `undefined` \| `number`

Defined in: ridb\_core.js:949

## Accessors

### collections

#### Get Signature

> **get** **collections**(): `object`

Defined in: ridb\_core.js:1000

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

Defined in: ridb\_core.js:976

##### Returns

`boolean`

## Methods

### \_\_destroy\_into\_raw()

> **\_\_destroy\_into\_raw**(): `undefined` \| `number`

Defined in: ridb\_core.js:947

#### Returns

`undefined` \| `number`

***

### authenticate()

> **authenticate**(`password`): `Promise`\<`boolean`\>

Defined in: ridb\_core.js:984

#### Parameters

##### password

`string`

#### Returns

`Promise`\<`boolean`\>

***

### close()

> **close**(): `Promise`\<`any`\>

Defined in: ridb\_core.js:968

#### Returns

`Promise`\<`any`\>

***

### free()

> **free**(): `void`

Defined in: ridb\_core.js:954

#### Returns

`void`

***

### start()

> **start**(): `Promise`\<`any`\>

Defined in: ridb\_core.js:961

#### Returns

`Promise`\<`any`\>

***

### \_\_wrap()

> `static` **\_\_wrap**(`ptr`): `any`

Defined in: ridb\_core.js:939

#### Parameters

##### ptr

`any`

#### Returns

`any`

***

### create()

> `static` **create**(`db_name`, `schemas_js`, `migrations_js`, `plugins`, `module`, `password?`, `storage?`): `Promise`\<`Database`\>

Defined in: ridb\_core.js:1025

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
