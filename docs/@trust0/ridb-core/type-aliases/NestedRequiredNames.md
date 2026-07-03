[**Documentation**](../../../README.md)

***

[Documentation](../../../README.md) / [@trust0/ridb-core](../README.md) / NestedRequiredNames

# Type Alias: NestedRequiredNames\<P\>

> **NestedRequiredNames**\<`P`\> = `P` *extends* `object` ? `R` *extends* readonly `string`[] ? `R`\[`number`\] : `never` : `never`

Defined in: ridb\_core.d.ts:582

NestedRequiredNames extracts the union of nested property names listed in an object
property's `required` array (JSON Schema semantics), or `never` when no array is
present. Note it only matches the array form; a boolean `required` flag yields `never`.

## Type Parameters

### P

`P`
