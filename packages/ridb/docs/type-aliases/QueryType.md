[**@trust0/ridb**](../README.md)

***

[@trust0/ridb](../README.md) / QueryType

# Type Alias: QueryType\<T\>

> **QueryType**\<`T`\>: `Partial`\<`{ [K in keyof T["properties"]]: OperatorOrType<ExtractType<T["properties"][K]["type"]>> }`\> & [`LogicalOperators`](LogicalOperators.md)\<`T`\> \| [`LogicalOperators`](LogicalOperators.md)\<`T`\>[]

Defined in: node\_modules/@trust0/ridb-core/pkg/ridb\_core.d.ts:87

## Type Parameters

• **T** *extends* [`SchemaType`](SchemaType.md)
