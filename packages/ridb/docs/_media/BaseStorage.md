[**@trust0/ridb-core**](../README.md)

***

[@trust0/ridb-core](../README.md) / BaseStorage

# Class: BaseStorage

Defined in: ridb\_core.js:532

Represents the base storage with a name and schema.

## Constructors

### Constructor

> **new BaseStorage**(`name`, `schemas_js`, `options?`): `BaseStorage`

Defined in: ridb\_core.js:560

Creates a new `BaseStorage` instance with the provided name and schema type.

# Arguments

* `name` - The name of the storage.
* `schema_type` - The schema type in `JsValue` format.

# Returns

* `Result<BaseStorage, JsValue>` - A result containing the new `BaseStorage` instance or an error.

#### Parameters

##### name

`string`

##### schemas\_js

`object`

##### options?

`object`

#### Returns

`BaseStorage`

## Properties

### \_\_wbg\_ptr

> **\_\_wbg\_ptr**: `number`

Defined in: ridb\_core.js:536

## Accessors

### core

#### Get Signature

> **get** **core**(): [`CoreStorage`](CoreStorage.md)

Defined in: ridb\_core.js:641

##### Returns

[`CoreStorage`](CoreStorage.md)

## Methods

### \_\_destroy\_into\_raw()

> **\_\_destroy\_into\_raw**(): `number`

Defined in: ridb\_core.js:534

#### Returns

`number`

***

### addIndexSchemas()

> **addIndexSchemas**(): `any`

Defined in: ridb\_core.js:581

#### Returns

`any`

***

### free()

> **free**(): `void`

Defined in: ridb\_core.js:541

#### Returns

`void`

***

### getOption()

> **getOption**(`name`): `any`

Defined in: ridb\_core.js:600

#### Parameters

##### name

`string`

#### Returns

`any`

***

### getSchema()

> **getSchema**(`name`): [`Schema`](Schema.md)

Defined in: ridb\_core.js:621

#### Parameters

##### name

`string`

#### Returns

[`Schema`](Schema.md)
