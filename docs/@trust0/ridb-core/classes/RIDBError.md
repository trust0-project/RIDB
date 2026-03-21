[**Documentation**](../../../README.md)

***

[Documentation](../../../README.md) / [@trust0/ridb-core](../README.md) / RIDBError

# Class: RIDBError

Defined in: ridb\_core.d.ts:761

## Constructors

### Constructor

> **new RIDBError**(`err_type`, `message`, `code`): `RIDBError`

Defined in: ridb\_core.d.ts:794

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

Defined in: ridb\_core.d.ts:788

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

Defined in: ridb\_core.d.ts:806

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

Defined in: ridb\_core.d.ts:817

#### Parameters

##### err

`any`

#### Returns

`RIDBError`

***

### hook()

> `static` **hook**(`err`, `code`): `RIDBError`

Defined in: ridb\_core.d.ts:800

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

Defined in: ridb\_core.d.ts:812

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

Defined in: ridb\_core.d.ts:782

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

Defined in: ridb\_core.d.ts:776

#### Parameters

##### err

`string`

##### code

`number`

#### Returns

`RIDBError`
