[**Documentation**](../../../README.md)

***

[Documentation](../../../README.md) / [@trust0/ridb-core](../README.md) / QueryType

# Type Alias: QueryType\<T\>

> **QueryType**\<`T`\> = `{ [K in keyof T["properties"] as ExtractType<T["properties"][K]["type"]> extends undefined ? never : K]?: OperatorOrType<ExtractType<T["properties"][K]["type"]>> }` & [`LogicalOperators`](LogicalOperators.md)\<`T`\> \| [`LogicalOperators`](LogicalOperators.md)\<`T`\>[]

Defined in: ridb\_core.d.ts:317

## Type Parameters

### T

`T` *extends* [`SchemaType`](SchemaType.md)
