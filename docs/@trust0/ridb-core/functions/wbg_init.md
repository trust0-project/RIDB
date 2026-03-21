[**Documentation**](../../../README.md)

***

[Documentation](../../../README.md) / [@trust0/ridb-core](../README.md) / \_\_wbg\_init

# Function: \_\_wbg\_init()

> **\_\_wbg\_init**(`module_or_path?`): `Promise`\<[`InitOutput`](../interfaces/InitOutput.md)\>

Defined in: ridb\_core.d.ts:1053

If `module_or_path` is {RequestInfo} or {URL}, makes a request and
for everything else, calls `WebAssembly.instantiate` directly.

## Parameters

### module\_or\_path?

[`InitInput`](../type-aliases/InitInput.md) | `Promise`\<[`InitInput`](../type-aliases/InitInput.md)\>

## Returns

`Promise`\<[`InitOutput`](../interfaces/InitOutput.md)\>
