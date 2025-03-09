[**@trust0/ridb**](../README.md)

***

[@trust0/ridb](../README.md) / RIDBError

# Class: RIDBError

Defined in: node\_modules/@trust0/ridb-core/pkg/ridb\_core.d.ts:711

## Constructors

### new RIDBError()

> **new RIDBError**(`err_type`, `message`, `code`): [`RIDBError`](RIDBError.md)

Defined in: node\_modules/@trust0/ridb-core/pkg/ridb\_core.d.ts:726

#### Parameters

##### err\_type

`string`

##### message

`string`

##### code

`number`

#### Returns

[`RIDBError`](RIDBError.md)

## Properties

### code

> `readonly` **code**: `any`

Defined in: node\_modules/@trust0/ridb-core/pkg/ridb\_core.d.ts:770

***

### message

> `readonly` **message**: `string`

Defined in: node\_modules/@trust0/ridb-core/pkg/ridb\_core.d.ts:773

***

### type

> `readonly` **type**: `string`

Defined in: node\_modules/@trust0/ridb-core/pkg/ridb\_core.d.ts:776

## Methods

### free()

> **free**(): `void`

Defined in: node\_modules/@trust0/ridb-core/pkg/ridb\_core.d.ts:720

#### Returns

`void`

***

### toJSON()

> **toJSON**(): `Object`

Defined in: node\_modules/@trust0/ridb-core/pkg/ridb\_core.d.ts:715

* Return copy of self without private attributes.

#### Returns

`Object`

***

### toString()

> **toString**(): `string`

Defined in: node\_modules/@trust0/ridb-core/pkg/ridb\_core.d.ts:719

Return stringified version of self.

#### Returns

`string`

***

### authentication()

> `static` **authentication**(`err`, `code`): [`RIDBError`](RIDBError.md)

Defined in: node\_modules/@trust0/ridb-core/pkg/ridb\_core.d.ts:749

#### Parameters

##### err

`string`

##### code

`number`

#### Returns

[`RIDBError`](RIDBError.md)

***

### error()

> `static` **error**(`err`, `code`): [`RIDBError`](RIDBError.md)

Defined in: node\_modules/@trust0/ridb-core/pkg/ridb\_core.d.ts:737

#### Parameters

##### err

`string`

##### code

`number`

#### Returns

[`RIDBError`](RIDBError.md)

***

### from()

> `static` **from**(`err`): [`RIDBError`](RIDBError.md)

Defined in: node\_modules/@trust0/ridb-core/pkg/ridb\_core.d.ts:731

#### Parameters

##### err

`any`

#### Returns

[`RIDBError`](RIDBError.md)

***

### hook()

> `static` **hook**(`err`, `code`): [`RIDBError`](RIDBError.md)

Defined in: node\_modules/@trust0/ridb-core/pkg/ridb\_core.d.ts:767

#### Parameters

##### err

`string`

##### code

`number`

#### Returns

[`RIDBError`](RIDBError.md)

***

### query()

> `static` **query**(`err`, `code`): [`RIDBError`](RIDBError.md)

Defined in: node\_modules/@trust0/ridb-core/pkg/ridb\_core.d.ts:743

#### Parameters

##### err

`string`

##### code

`number`

#### Returns

[`RIDBError`](RIDBError.md)

***

### serialisation()

> `static` **serialisation**(`err`, `code`): [`RIDBError`](RIDBError.md)

Defined in: node\_modules/@trust0/ridb-core/pkg/ridb\_core.d.ts:755

#### Parameters

##### err

`string`

##### code

`number`

#### Returns

[`RIDBError`](RIDBError.md)

***

### validation()

> `static` **validation**(`err`, `code`): [`RIDBError`](RIDBError.md)

Defined in: node\_modules/@trust0/ridb-core/pkg/ridb\_core.d.ts:761

#### Parameters

##### err

`string`

##### code

`number`

#### Returns

[`RIDBError`](RIDBError.md)
