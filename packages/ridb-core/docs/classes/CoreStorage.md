[**@trust0/ridb-core**](../README.md)

***

[@trust0/ridb-core](../README.md) / CoreStorage

# Class: CoreStorage

Defined in: ridb\_core.js:826

## Constructors

### Constructor

> **new CoreStorage**(): `CoreStorage`

Defined in: ridb\_core.js:849

#### Returns

`CoreStorage`

## Properties

### \_\_wbg\_ptr

> **\_\_wbg\_ptr**: `number`

Defined in: ridb\_core.js:838

## Methods

### \_\_destroy\_into\_raw()

> **\_\_destroy\_into\_raw**(): `number`

Defined in: ridb\_core.js:836

#### Returns

`number`

***

### free()

> **free**(): `void`

Defined in: ridb\_core.js:843

#### Returns

`void`

***

### getIndexes()

> **getIndexes**(`schema`, `op`): `string`[]

Defined in: ridb\_core.js:887

#### Parameters

##### schema

[`Schema`](Schema.md)

##### op

[`Operation`](Operation.md)

#### Returns

`string`[]

***

### getPrimaryKeyTyped()

> **getPrimaryKeyTyped**(`value`): `string`

Defined in: ridb\_core.js:858

#### Parameters

##### value

`any`

#### Returns

`string`

***

### matchesQuery()

> **matchesQuery**(`document`, `query`): `boolean`

Defined in: ridb\_core.js:912

#### Parameters

##### document

`any`

##### query

[`Query`](Query.md)

#### Returns

`boolean`

***

### \_\_wrap()

> `static` **\_\_wrap**(`ptr`): `any`

Defined in: ridb\_core.js:828

#### Parameters

##### ptr

`any`

#### Returns

`any`
