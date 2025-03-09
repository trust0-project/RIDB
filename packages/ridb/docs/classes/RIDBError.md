[**@trust0/ridb**](../README.md)

***

[@trust0/ridb](../README.md) / RIDBError

# Class: RIDBError

Defined in: ridb-core/pkg/ridb\_core.d.ts:709

## Constructors

### new RIDBError()

> **new RIDBError**(): [`RIDBError`](RIDBError.md)

#### Returns

[`RIDBError`](RIDBError.md)

## Properties

### code

> `readonly` **code**: `any`

Defined in: ridb-core/pkg/ridb\_core.d.ts:754

***

### message

> `readonly` **message**: `string`

Defined in: ridb-core/pkg/ridb\_core.d.ts:757

***

### type

> `readonly` **type**: `string`

Defined in: ridb-core/pkg/ridb\_core.d.ts:760

## Methods

### free()

> **free**(): `void`

Defined in: ridb-core/pkg/ridb\_core.d.ts:710

#### Returns

`void`

***

### authentication()

> `static` **authentication**(`err`, `code`): [`RIDBError`](RIDBError.md)

Defined in: ridb-core/pkg/ridb\_core.d.ts:733

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

Defined in: ridb-core/pkg/ridb\_core.d.ts:721

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

Defined in: ridb-core/pkg/ridb\_core.d.ts:715

#### Parameters

##### err

`any`

#### Returns

[`RIDBError`](RIDBError.md)

***

### hook()

> `static` **hook**(`err`, `code`): [`RIDBError`](RIDBError.md)

Defined in: ridb-core/pkg/ridb\_core.d.ts:751

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

Defined in: ridb-core/pkg/ridb\_core.d.ts:727

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

Defined in: ridb-core/pkg/ridb\_core.d.ts:739

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

Defined in: ridb-core/pkg/ridb\_core.d.ts:745

#### Parameters

##### err

`string`

##### code

`number`

#### Returns

[`RIDBError`](RIDBError.md)
