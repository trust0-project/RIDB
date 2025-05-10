[**@trust0/ridb-core**](../README.md)

***

[@trust0/ridb-core](../README.md) / Query

# Class: Query

Defined in: ridb\_core.js:1605

## Constructors

### Constructor

> **new Query**(`query`, `schema`): `Query`

Defined in: ridb\_core.js:1622

#### Parameters

##### query

`any`

##### schema

[`Schema`](Schema.md)

#### Returns

`Query`

## Properties

### \_\_wbg\_ptr

> **\_\_wbg\_ptr**: `number`

Defined in: ridb\_core.js:1609

## Accessors

### query

#### Get Signature

> **get** **query**(): `any`

Defined in: ridb\_core.js:1643

##### Returns

`any`

## Methods

### \_\_destroy\_into\_raw()

> **\_\_destroy\_into\_raw**(): `number`

Defined in: ridb\_core.js:1607

#### Returns

`number`

***

### free()

> **free**(): `void`

Defined in: ridb\_core.js:1614

#### Returns

`void`

***

### get()

> **get**(`property_name`): `any`

Defined in: ridb\_core.js:1732

Returns the value of a property from the (normalized) query by its name.
This will scan the normalized query structure (including arrays, $and/$or blocks, etc.)
to find the first occurrence of the given property name and return its corresponding value.

If not found, an error is returned.

Example:
  let val = query.get("age")?;
  // val is a JsValue that might be a number, string, boolean, array, or object (e.g., { "$gt": 30 })

#### Parameters

##### property\_name

`string`

#### Returns

`any`

***

### get\_properties()

> **get\_properties**(): `string`[]

Defined in: ridb\_core.js:1663

Returns the schema properties (fields) that are used in the query.
The query may contain operators like $and, $or, $gt, $lt, etc.

#### Returns

`string`[]

***

### parse()

> **parse**(): `any`

Defined in: ridb\_core.js:1684

#### Returns

`any`

***

### process\_query()

> **process\_query**(`query`): `any`

Defined in: ridb\_core.js:1703

#### Parameters

##### query

`any`

#### Returns

`any`
