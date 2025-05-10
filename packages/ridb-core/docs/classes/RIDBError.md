[**@trust0/ridb-core**](../README.md)

***

[@trust0/ridb-core](../README.md) / RIDBError

# Class: RIDBError

Defined in: ridb\_core.js:1820

## Constructors

### Constructor

> **new RIDBError**(`err_type`, `message`, `code`): `RIDBError`

Defined in: ridb\_core.js:1858

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

Defined in: ridb\_core.js:1844

## Accessors

### code

#### Get Signature

> **get** **code**(): `any`

Defined in: ridb\_core.js:1889

##### Returns

`any`

***

### message

#### Get Signature

> **get** **message**(): `string`

Defined in: ridb\_core.js:1896

##### Returns

`string`

***

### type

#### Get Signature

> **get** **type**(): `string`

Defined in: ridb\_core.js:1870

##### Returns

`string`

## Methods

### \_\_destroy\_into\_raw()

> **\_\_destroy\_into\_raw**(): `number`

Defined in: ridb\_core.js:1842

#### Returns

`number`

***

### free()

> **free**(): `void`

Defined in: ridb\_core.js:1849

#### Returns

`void`

***

### toJSON()

> **toJSON**(): `object`

Defined in: ridb\_core.js:1830

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

Defined in: ridb\_core.js:1838

#### Returns

`string`

***

### \_\_wrap()

> `static` **\_\_wrap**(`ptr`): `any`

Defined in: ridb\_core.js:1822

#### Parameters

##### ptr

`any`

#### Returns

`any`

***

### authentication()

> `static` **authentication**(`err`, `code`): `RIDBError`

Defined in: ridb\_core.js:1947

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

Defined in: ridb\_core.js:1925

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

Defined in: ridb\_core.js:1916

#### Parameters

##### err

`any`

#### Returns

`RIDBError`

***

### hook()

> `static` **hook**(`err`, `code`): `RIDBError`

Defined in: ridb\_core.js:1980

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

Defined in: ridb\_core.js:1936

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

Defined in: ridb\_core.js:1958

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

Defined in: ridb\_core.js:1969

#### Parameters

##### err

`string`

##### code

`number`

#### Returns

`RIDBError`
