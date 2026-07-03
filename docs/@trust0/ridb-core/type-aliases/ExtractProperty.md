[**Documentation**](../../../README.md)

***

[Documentation](../../../README.md) / [@trust0/ridb-core](../README.md) / ExtractProperty

# Type Alias: ExtractProperty\<P\>

> **ExtractProperty**\<`P`\> = `P` *extends* `object` ? `string` : `P` *extends* `object` ? `number` : `P` *extends* `object` ? `boolean` : `P` *extends* `object` ? `P` *extends* `object` ? `ExtractProperty`\<`I`\>[] : `any`[] : `P` *extends* `object` ? `P` *extends* `object` ? [`ExtractObject`](ExtractObject.md)\<`PR`, [`NestedRequiredNames`](NestedRequiredNames.md)\<`P`\>\> : `object` : `unknown`

Defined in: ridb\_core.d.ts:569

ExtractProperty maps a full Property definition to its document type. Unlike
[ExtractType](ExtractType.md) (which only looks at the `type` string), it recurses into
`items` for arrays and `properties` for objects, producing precise nested types.

## Type Parameters

### P

`P`

## Example

```ts
type Tags = ExtractProperty<{ type: "array"; items: { type: "string" } }>; // string[]
type Obj  = ExtractProperty<{ type: "object"; properties: { id: { type: "string" } } }>; // { id: string }
```
