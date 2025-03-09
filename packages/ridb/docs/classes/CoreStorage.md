[**@trust0/ridb**](../README.md)

***

[@trust0/ridb](../README.md) / CoreStorage

# Class: CoreStorage

Defined in: ridb-core/pkg/ridb\_core.d.ts:112

## Constructors

### new CoreStorage()

> **new CoreStorage**(): [`CoreStorage`](CoreStorage.md)

#### Returns

[`CoreStorage`](CoreStorage.md)

## Methods

### getIndexes()

> **getIndexes**(`schema`, `op`): `string`[]

Defined in: ridb-core/pkg/ridb\_core.d.ts:120

#### Parameters

##### schema

[`Schema`](Schema.md)\<`any`\>

##### op

[`Operation`](../type-aliases/Operation.md)

#### Returns

`string`[]

***

### getPrimaryKeyTyped()

> **getPrimaryKeyTyped**(`value`): `string` \| `number`

Defined in: ridb-core/pkg/ridb\_core.d.ts:119

#### Parameters

##### value

`any`

#### Returns

`string` \| `number`

***

### matchesQuery()

> **matchesQuery**(`document`, `query`): `boolean`

Defined in: ridb-core/pkg/ridb\_core.d.ts:118

#### Parameters

##### document

`any`

##### query

[`Query`](Query.md)\<`any`\>

#### Returns

`boolean`
