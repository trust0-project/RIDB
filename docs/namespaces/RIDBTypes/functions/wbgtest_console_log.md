[**@trust0/ridb**](../../../README.md) • **Docs**

***

[@trust0/ridb](../../../README.md) / [RIDBTypes](../README.md) / \_\_wbgtest\_console\_log

# Function: \_\_wbgtest\_console\_log()

> **\_\_wbgtest\_console\_log**(`args`): `void`

Handler for `console.log` invocations.

If a test is currently running it takes the `args` array and stringifies
it and appends it to the current output of the test. Otherwise it passes
the arguments to the original `console.log` function, psased as
`original`.

## Parameters

• **args**: `any`[]

## Returns

`void`

## Defined in

pkg/ridb\_rust.d.ts:15
