[**@trust0/ridb**](../README.md)

***

[@trust0/ridb](../README.md) / AnyVersionGreaterThan1

# Type Alias: AnyVersionGreaterThan1\<T\>

> **AnyVersionGreaterThan1**\<`T`\>: `true` *extends* `{ [K in keyof T]: IsVersionGreaterThan0<T[K]["version"]> }`\[keyof `T`\] ? `true` : `false`

Defined in: ridb-core/pkg/ridb\_core.d.ts:579

## Type Parameters

• **T** *extends* `Record`\<`string`, [`SchemaType`](SchemaType.md)\>
