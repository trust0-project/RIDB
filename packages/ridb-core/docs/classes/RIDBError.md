[**@trust0/ridb-core**](../README.md)

***

[@trust0/ridb-core](../README.md) / RIDBError

# Class: RIDBError

Defined in: ridb\_core.js:1821

## Constructors

### Constructor

> **new RIDBError**(`err_type`, `message`, `code`): `RIDBError`

Defined in: ridb\_core.js:1859

#### Parameters

##### err\_type

`string`

##### message

`string`

##### code

`number`

#### Returns

`RIDBError`

## Properties

### \_\_wbg\_ptr

> **\_\_wbg\_ptr**: `number`

Defined in: ridb\_core.js:1845

## Accessors

### code

#### Get Signature

> **get** **code**(): `any`

Defined in: ridb\_core.js:1890

##### Returns

`any`

***

### message

#### Get Signature

> **get** **message**(): `string`

Defined in: ridb\_core.js:1897

##### Returns

`string`

***

### type

#### Get Signature

> **get** **type**(): `string`

Defined in: ridb\_core.js:1871

##### Returns

`string`

## Methods

### \_\_destroy\_into\_raw()

> **\_\_destroy\_into\_raw**(): `number`

Defined in: ridb\_core.js:1843

#### Returns

`number`

***

### free()

> **free**(): `void`

Defined in: ridb\_core.js:1850

#### Returns

`void`

***

### toJSON()

> **toJSON**(): `object`

Defined in: ridb\_core.js:1831

#### Returns

`object`

##### code

> **code**: `any`

##### message

> **message**: `string`

##### type

> **type**: `string`

***

### toString()

> **toString**(): `string`

Defined in: ridb\_core.js:1839

#### Returns

`string`

***

### \_\_wrap()

> `static` **\_\_wrap**(`ptr`): `any`

Defined in: ridb\_core.js:1823

#### Parameters

##### ptr

`any`

#### Returns

`any`

***

### authentication()

> `static` **authentication**(`err`, `code`): `RIDBError`

Defined in: ridb\_core.js:1948

#### Parameters

##### err

`string`

##### code

`number`

#### Returns

`RIDBError`

***

### error()

> `static` **error**(`err`, `code`): `RIDBError`

Defined in: ridb\_core.js:1926

#### Parameters

##### err

`string`

##### code

`number`

#### Returns

`RIDBError`

***

### from()

> `static` **from**(`err`): `RIDBError`

Defined in: ridb\_core.js:1917

#### Parameters

##### err

`any`

#### Returns

`RIDBError`

***

### hook()

> `static` **hook**(`err`, `code`): `RIDBError`

Defined in: ridb\_core.js:1981

#### Parameters

##### err

`string`

##### code

`number`

#### Returns

`RIDBError`

***

### query()

> `static` **query**(`err`, `code`): `RIDBError`

Defined in: ridb\_core.js:1937

#### Parameters

##### err

`string`

##### code

`number`

#### Returns

`RIDBError`

***

### serialisation()

> `static` **serialisation**(`err`, `code`): `RIDBError`

Defined in: ridb\_core.js:1959

#### Parameters

##### err

`string`

##### code

`number`

#### Returns

`RIDBError`

***

### validation()

> `static` **validation**(`err`, `code`): `RIDBError`

Defined in: ridb\_core.js:1970

#### Parameters

##### err

`string`

##### code

`number`

#### Returns

`RIDBError`
