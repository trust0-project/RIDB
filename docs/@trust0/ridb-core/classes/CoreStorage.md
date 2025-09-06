[**Documentation**](../../../README.md)

***

[Documentation](../../../README.md) / [@trust0/ridb-core](../README.md) / CoreStorage

# Class: CoreStorage

Defined in: ridb\_core.d.ts:616

## Constructors

### Constructor

> **new CoreStorage**(): `CoreStorage`

#### Returns

`CoreStorage`

## Methods

### getIndexes()

> **getIndexes**(`schema`, `op`): `string`[]

Defined in: ridb\_core.d.ts:624

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

Defined in: ridb\_core.d.ts:623

#### Parameters

##### value

`any`

#### Returns

`string` \| `number`

***

### matchesQuery()

> **matchesQuery**(`document`, `query`): `boolean`

Defined in: ridb\_core.d.ts:622

#### Parameters

##### document

`any`

##### query

[`Query`](Query.md)\<`any`\>

#### Returns

`boolean`
