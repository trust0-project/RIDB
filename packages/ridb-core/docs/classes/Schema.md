[**@trust0/ridb-core**](../README.md)

***

[@trust0/ridb-core](../README.md) / Schema

# Class: Schema

Defined in: ridb\_core.js:1994

Represents the schema of a collection, including version, primary key, type, required fields, properties, and indexes.

## Constructors

### Constructor

> **new Schema**(): `Schema`

#### Returns

`Schema`

## Properties

### \_\_wbg\_ptr

> **\_\_wbg\_ptr**: `undefined` \| `number`

Defined in: ridb\_core.js:2006

## Accessors

### encrypted

#### Get Signature

> **get** **encrypted**(): `undefined` \| `string`[]

Defined in: ridb\_core.js:2164

##### Returns

`undefined` \| `string`[]

***

### indexes

#### Get Signature

> **get** **indexes**(): `undefined` \| `string`[]

Defined in: ridb\_core.js:2145

Retrieves the indexes of the schema, if any.

# Returns

* `Option<Vec<String>>` - The indexes of the schema, if any.

##### Returns

`undefined` \| `string`[]

***

### primaryKey

#### Get Signature

> **get** **primaryKey**(): `string`

Defined in: ridb\_core.js:2097

Retrieves the primary key of the schema.

# Returns

* `String` - The primary key of the schema.

##### Returns

`string`

***

### properties

#### Get Signature

> **get** **properties**(): `any`

Defined in: ridb\_core.js:2188

Retrieves the properties of the schema.

# Returns

* `Result<JsValue, JsValue>` - A result containing the properties as a `JsValue` or an error.

##### Returns

`any`

***

### type

#### Get Signature

> **get** **type**(): `string`

Defined in: ridb\_core.js:2121

Retrieves the type of the schema.

# Returns

* `String` - The type of the schema.

##### Returns

`string`

***

### version

#### Get Signature

> **get** **version**(): `number`

Defined in: ridb\_core.js:2085

Retrieves the version of the schema.

# Returns

* `i32` - The version of the schema.

##### Returns

`number`

## Methods

### \_\_destroy\_into\_raw()

> **\_\_destroy\_into\_raw**(): `undefined` \| `number`

Defined in: ridb\_core.js:2004

#### Returns

`undefined` \| `number`

***

### free()

> **free**(): `void`

Defined in: ridb\_core.js:2011

#### Returns

`void`

***

### is\_valid()

> **is\_valid**(): `boolean`

Defined in: ridb\_core.js:2034

#### Returns

`boolean`

***

### validate()

> **validate**(`document`): `void`

Defined in: ridb\_core.js:2018

#### Parameters

##### document

`any`

#### Returns

`void`

***

### \_\_wrap()

> `static` **\_\_wrap**(`ptr`): `any`

Defined in: ridb\_core.js:1996

#### Parameters

##### ptr

`any`

#### Returns

`any`

***

### create()

> `static` **create**(`schema`): `Schema`

Defined in: ridb\_core.js:2062

Creates a new `Schema` instance from a given `JsValue`.

# Arguments

* `schema` - A `JsValue` representing the schema.

# Returns

* `Result<Schema, JsValue>` - A result containing the new `Schema` instance or an error.

#### Parameters

##### schema

`any`

#### Returns

`Schema`
