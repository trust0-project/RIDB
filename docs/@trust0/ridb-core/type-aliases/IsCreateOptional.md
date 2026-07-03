[**Documentation**](../../../README.md)

***

[Documentation](../../../README.md) / [@trust0/ridb-core](../README.md) / IsCreateOptional

# Type Alias: IsCreateOptional\<T, K\>

> **IsCreateOptional**\<`T`, `K`\> = `T`\[`"properties"`\]\[`K`\] *extends* `object` ? `true` : [`FlagRequiredness`](FlagRequiredness.md)\<`T`\[`"properties"`\]\[`K`\]\> *extends* `"optional"` ? `true` : [`FlagRequiredness`](FlagRequiredness.md)\<`T`\[`"properties"`\]\[`K`\]\> *extends* `"required"` ? `false` : `K` *extends* [`RequiredFieldNames`](RequiredFieldNames.md)\<`T`\> ? `false` : `true`

Defined in: ridb\_core.d.ts:655

IsCreateOptional decides whether a property may be omitted when creating a document.
Precedence (mirrors the runtime validator):
 1. a declared `default` makes the field optional;
 2. a boolean `required` flag wins (`false` -> optional, `true`/`boolean` -> required;
    see [FlagRequiredness](FlagRequiredness.md));
 3. otherwise it is required iff listed in the schema-level `required` array;
 4. otherwise it is optional.

## Type Parameters

### T

`T` *extends* [`SchemaType`](SchemaType.md)

### K

`K` *extends* keyof `T`\[`"properties"`\]
