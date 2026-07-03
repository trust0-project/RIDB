[**Documentation**](../../../README.md)

***

[Documentation](../../../README.md) / [@trust0/ridb-core](../README.md) / FlagRequiredness

# Type Alias: FlagRequiredness\<P\>

> **FlagRequiredness**\<`P`\> = `P` *extends* `object` ? \[`F`\] *extends* \[readonly `string`[]\] ? `"defer"` : \[`F`\] *extends* \[`false`\] ? `"optional"` : \[`true`\] *extends* \[`F`\] ? `"required"` : `"defer"` : `"defer"`

Defined in: ridb\_core.d.ts:598

FlagRequiredness interprets a property's `required` declaration when it is used as a
legacy boolean flag. Because `Property.required` is typed `boolean | string[]`, a
schema literal written without `as const` widens `true`/`false` to `boolean`; the
tuple-wrapped checks below classify each case:
 - a literal `false` -> `"optional"`;
 - a literal `true`, or a widened `boolean` (whose literal was lost) -> `"required"`.
   Treating the ambiguous `boolean` as required matches the Rust validator and turns a
   would-be runtime "missing required property" error into a compile-time one instead;
 - the array form, or no `required` key -> `"defer"` to the container `required` array.

## Type Parameters

### P

`P`
