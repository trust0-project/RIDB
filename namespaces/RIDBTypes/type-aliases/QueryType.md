[**@elribonazo/ridb**](../../../README.md) • **Docs**

***

[@elribonazo/ridb](../../../README.md) / [RIDBTypes](../README.md) / QueryType

# Type Alias: QueryType\<T\>

> **QueryType**\<`T`\>: `{ [K in keyof T["properties"]]: OperatorOrType<ExtractType<T["properties"][K]["type"]>> }` & [`LogicalOperators`](LogicalOperators.md)\<`T`\> \| [`LogicalOperators`](LogicalOperators.md)\<`T`\>[]

## Type Parameters

• **T** *extends* [`SchemaType`](SchemaType.md)

## Defined in

pkg/ridb\_rust.d.ts:229
