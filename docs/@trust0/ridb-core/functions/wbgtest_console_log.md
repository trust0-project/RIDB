[**Documentation**](../../../README.md)

***

[Documentation](../../../README.md) / [@trust0/ridb-core](../README.md) / \_\_wbgtest\_console\_log

# Function: \_\_wbgtest\_console\_log()

> **\_\_wbgtest\_console\_log**(`args`): `void`

Defined in: ridb\_core.d.ts:19

Handler for `console.log` invocations.

If a test is currently running it takes the `args` array and stringifies
it and appends it to the current output of the test. Otherwise it passes
the arguments to the original `console.log` function, psased as
`original`.

## Parameters

### args

`any`[]

## Returns

`void`
