[**@trust0/ridb-core**](../README.md)

***

[@trust0/ridb-core](../README.md) / CoreStorage

# Class: CoreStorage

Defined in: ridb\_core.js:825

## Constructors

### Constructor

> **new CoreStorage**(): `CoreStorage`

Defined in: ridb\_core.js:848

#### Returns

`CoreStorage`

## Properties

### \_\_wbg\_ptr

> **\_\_wbg\_ptr**: `number`

Defined in: ridb\_core.js:837

## Methods

### \_\_destroy\_into\_raw()

> **\_\_destroy\_into\_raw**(): `number`

Defined in: ridb\_core.js:835

#### Returns

`number`

***

### free()

> **free**(): `void`

Defined in: ridb\_core.js:842

#### Returns

`void`

***

### getIndexes()

> **getIndexes**(`schema`, `op`): `string`[]

Defined in: ridb\_core.js:886

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

Defined in: ridb\_core.js:857

#### Parameters

##### value

`any`

#### Returns

`string`

***

### matchesQuery()

> **matchesQuery**(`document`, `query`): `boolean`

Defined in: ridb\_core.js:911

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

Defined in: ridb\_core.js:827

#### Parameters

##### ptr

`any`

#### Returns

`any`
