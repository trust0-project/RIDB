[**Documentation**](../../../README.md)

***

[Documentation](../../../README.md) / [@trust0/ridb-core](../README.md) / RIDBError

# Class: RIDBError

Defined in: ridb\_core.d.ts:761

## Constructors

### Constructor

> **new RIDBError**(`err_type`, `message`, `code`): `RIDBError`

Defined in: ridb\_core.d.ts:776

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

Defined in: ridb\_core.d.ts:820

***

### message

> `readonly` **message**: `string`

Defined in: ridb\_core.d.ts:823

***

### type

> `readonly` **type**: `string`

Defined in: ridb\_core.d.ts:826

## Methods

### free()

> **free**(): `void`

Defined in: ridb\_core.d.ts:770

#### Returns

`void`

***

### toJSON()

> **toJSON**(): `Object`

Defined in: ridb\_core.d.ts:765

* Return copy of self without private attributes.

#### Returns

`Object`

***

### toString()

> **toString**(): `string`

Defined in: ridb\_core.d.ts:769

Return stringified version of self.

#### Returns

`string`

***

### authentication()

> `static` **authentication**(`err`, `code`): `RIDBError`

Defined in: ridb\_core.d.ts:799

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

Defined in: ridb\_core.d.ts:787

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

Defined in: ridb\_core.d.ts:781

#### Parameters

##### err

`any`

#### Returns

`RIDBError`

***

### hook()

> `static` **hook**(`err`, `code`): `RIDBError`

Defined in: ridb\_core.d.ts:817

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

Defined in: ridb\_core.d.ts:793

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

Defined in: ridb\_core.d.ts:805

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

Defined in: ridb\_core.d.ts:811

#### Parameters

##### err

`string`

##### code

`number`

#### Returns

`RIDBError`
