[**@trust0/ridb**](../../../README.md) • **Docs**

***

[@trust0/ridb](../../../README.md) / [RIDBTypes](../README.md) / \_\_wbg\_init

# Function: \_\_wbg\_init()

> **\_\_wbg\_init**(`module_or_path`?): `Promise`\<`InitOutput`\>

If `module_or_path` is {RequestInfo} or {URL}, makes a request and
for everything else, calls `WebAssembly.instantiate` directly.

## Parameters

• **module\_or\_path?**: `InitInput` \| `Promise`\<`InitInput`\>

## Returns

`Promise`\<`InitOutput`\>

## Defined in

pkg/ridb\_rust.d.ts:849
