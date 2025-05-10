[**@trust0/ridb-core**](../README.md)

***

[@trust0/ridb-core](../README.md) / Operation

# Class: Operation

Defined in: ridb\_core.js:1284

Represents an operation to be performed on a collection.

## Constructors

### Constructor

> **new Operation**(): `Operation`

#### Returns

`Operation`

## Properties

### \_\_wbg\_ptr

> **\_\_wbg\_ptr**: `undefined` \| `number`

Defined in: ridb\_core.js:1296

## Accessors

### collection

#### Get Signature

> **get** **collection**(): `string`

Defined in: ridb\_core.js:1313

Retrieves the name of the collection.

# Returns

* `String` - The name of the collection.

##### Returns

`string`

***

### data

#### Get Signature

> **get** **data**(): `any`

Defined in: ridb\_core.js:1349

Retrieves the data involved in the operation.

# Returns

* `JsValue` - The data involved in the operation.

##### Returns

`any`

***

### opType

#### Get Signature

> **get** **opType**(): `Readonly`\<\{ `0`: `"CREATE"`; `1`: `"UPDATE"`; `2`: `"DELETE"`; `3`: `"QUERY"`; `4`: `"COUNT"`; `COUNT`: `4`; `CREATE`: `0`; `DELETE`: `2`; `QUERY`: `3`; `UPDATE`: `1`; \}\>

Defined in: ridb\_core.js:1337

Retrieves the type of operation.

# Returns

* `OpType` - The type of operation.

##### Returns

`Readonly`\<\{ `0`: `"CREATE"`; `1`: `"UPDATE"`; `2`: `"DELETE"`; `3`: `"QUERY"`; `4`: `"COUNT"`; `COUNT`: `4`; `CREATE`: `0`; `DELETE`: `2`; `QUERY`: `3`; `UPDATE`: `1`; \}\>

***

### primaryKey

#### Get Signature

> **get** **primaryKey**(): `any`

Defined in: ridb\_core.js:1373

Retrieves the primary key value of the current data.

# Returns

* `Option<JsValue>` - The primary key value of the current data.

##### Returns

`any`

***

### primaryKeyField

#### Get Signature

> **get** **primaryKeyField**(): `any`

Defined in: ridb\_core.js:1361

Retrieves the primary key field of the current collection.

# Returns

* `Option<String>` - The primary key field of the current collection.

##### Returns

`any`

***

### primaryKeyIndex

#### Get Signature

> **get** **primaryKeyIndex**(): `string`

Defined in: ridb\_core.js:1380

##### Returns

`string`

## Methods

### \_\_destroy\_into\_raw()

> **\_\_destroy\_into\_raw**(): `undefined` \| `number`

Defined in: ridb\_core.js:1294

#### Returns

`undefined` \| `number`

***

### free()

> **free**(): `void`

Defined in: ridb\_core.js:1301

#### Returns

`void`

***

### \_\_wrap()

> `static` **\_\_wrap**(`ptr`): `any`

Defined in: ridb\_core.js:1286

#### Parameters

##### ptr

`any`

#### Returns

`any`
