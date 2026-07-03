[**Documentation**](../../../README.md)

***

[Documentation](../../../README.md) / [@trust0/ridb-core](../README.md) / RIDBError

# Class: RIDBError

Defined in: ridb\_core.d.ts:883

## Constructors

### Constructor

> **new RIDBError**(`err_type`, `message`, `code`): `RIDBError`

Defined in: ridb\_core.d.ts:916

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

### code

> `readonly` **code**: `any`

Defined in: ridb\_core.d.ts:942

***

### message

> `readonly` **message**: `string`

Defined in: ridb\_core.d.ts:945

***

### type

> `readonly` **type**: `string`

Defined in: ridb\_core.d.ts:948

## Methods

### free()

> **free**(): `void`

Defined in: ridb\_core.d.ts:892

#### Returns

`void`

***

### toJSON()

> **toJSON**(): `Object`

Defined in: ridb\_core.d.ts:887

* Return copy of self without private attributes.

#### Returns

`Object`

***

### toString()

> **toString**(): `string`

Defined in: ridb\_core.d.ts:891

Return stringified version of self.

#### Returns

`string`

***

### authentication()

> `static` **authentication**(`err`, `code`): `RIDBError`

Defined in: ridb\_core.d.ts:910

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

Defined in: ridb\_core.d.ts:928

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

Defined in: ridb\_core.d.ts:939

#### Parameters

##### err

`any`

#### Returns

`RIDBError`

***

### hook()

> `static` **hook**(`err`, `code`): `RIDBError`

Defined in: ridb\_core.d.ts:922

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

Defined in: ridb\_core.d.ts:934

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

Defined in: ridb\_core.d.ts:904

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

Defined in: ridb\_core.d.ts:898

#### Parameters

##### err

`string`

##### code

`number`

#### Returns

`RIDBError`
