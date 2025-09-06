[**Documentation**](../../../README.md)

***

[Documentation](../../../README.md) / [@trust0/ridb-core](../README.md) / ExtractType

# Type Alias: ExtractType\<T\>

> **ExtractType**\<`T`\> = `T` *extends* `"string"` ? `string` : `T` *extends* `"number"` ? `number` : `T` *extends* `"boolean"` ? `boolean` : `T` *extends* `"object"` ? `object` : `T` *extends* `"array"` ? `any`[] : `undefined`

Defined in: ridb\_core.d.ts:207

ExtractType is a utility type that maps a string representing a basic data type to the actual TypeScript type.

## Type Parameters

### T

`T` *extends* `string`

A string literal type representing the basic data type ('string', 'number', 'boolean', 'object', 'array').

## Example

```ts
type StringType = ExtractType<'string'>; // StringType is string
type NumberType = ExtractType<'number'>; // NumberType is number
type BooleanType = ExtractType<'boolean'>; // BooleanType is boolean
type ObjectType = ExtractType<'object'>; // ObjectType is object
type ArrayType = ExtractType<'array'>; // ArrayType is Array<any>
```
