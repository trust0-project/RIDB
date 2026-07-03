[**Documentation**](../../../README.md)

***

[Documentation](../../../README.md) / [@trust0/ridb-core](../README.md) / IsNestedOptional

# Type Alias: IsNestedOptional\<PR, K, R\>

> **IsNestedOptional**\<`PR`, `K`, `R`\> = `PR`\[`K`\] *extends* `object` ? `true` : [`FlagRequiredness`](FlagRequiredness.md)\<`PR`\[`K`\]\> *extends* `"optional"` ? `true` : [`FlagRequiredness`](FlagRequiredness.md)\<`PR`\[`K`\]\> *extends* `"required"` ? `false` : `K` *extends* `R` ? `false` : `true`

Defined in: ridb\_core.d.ts:615

IsNestedOptional decides whether a nested property `K` (within an object property's
`properties` map `PR`, given that object's required-name union `R`) may be omitted.
Precedence mirrors the runtime validator and [IsCreateOptional](IsCreateOptional.md):
 1. a declared `default` makes the field optional;
 2. a boolean `required` flag wins (`false` -> optional, `true`/`boolean` -> required);
 3. otherwise it is required iff listed in the object's `required` array;
 4. otherwise it is optional.

## Type Parameters

### PR

`PR`

### K

`K` *extends* keyof `PR`

### R

`R`
