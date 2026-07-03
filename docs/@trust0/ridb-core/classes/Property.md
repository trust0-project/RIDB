[**Documentation**](../../../README.md)

***

[Documentation](../../../README.md) / [@trust0/ridb-core](../README.md) / Property

# Class: Property

Defined in: ridb\_core.d.ts:278

Represents a property within a schema, including various constraints and nested properties.

## Constructors

### Constructor

> **new Property**(): `Property`

#### Returns

`Property`

## Properties

### default?

> `readonly` `optional` **default?**: `any`

Defined in: ridb\_core.d.ts:331

An optional default value for the property.

***

### items?

> `readonly` `optional` **items?**: `Property`

Defined in: ridb\_core.d.ts:297

An optional array of nested properties for array-type properties.

***

### maxItems?

> `readonly` `optional` **maxItems?**: `number`

Defined in: ridb\_core.d.ts:302

The maximum number of items for array-type properties, if applicable.

***

### maxLength?

> `readonly` `optional` **maxLength?**: `number`

Defined in: ridb\_core.d.ts:312

The maximum length for string-type properties, if applicable.

***

### minItems?

> `readonly` `optional` **minItems?**: `number`

Defined in: ridb\_core.d.ts:307

The minimum number of items for array-type properties, if applicable.

***

### minLength?

> `readonly` `optional` **minLength?**: `number`

Defined in: ridb\_core.d.ts:317

The minimum length for string-type properties, if applicable.

***

### primaryKey?

> `readonly` `optional` **primaryKey?**: `string`

Defined in: ridb\_core.d.ts:292

The primary key of the property, if applicable.

***

### properties?

> `readonly` `optional` **properties?**: `object`

Defined in: ridb\_core.d.ts:336

An optional map of nested properties for object-type properties.

#### Index Signature

\[`name`: `string`\]: `Property`

***

### required?

> `readonly` `optional` **required?**: `boolean` \| `string`[]

Defined in: ridb\_core.d.ts:326

Controls requiredness. Two forms are supported and interoperate:
 - `boolean`: a per-property flag (legacy). `false` forces the property optional,
   `true` forces it required, overriding any container-level `required` array.
 - `string[]`: for object-type properties, the JSON Schema list of required
   nested properties.

***

### type

> `readonly` **type**: `SchemaFieldType`

Defined in: ridb\_core.d.ts:282

The type of the property.

***

### version?

> `readonly` `optional` **version?**: `number`

Defined in: ridb\_core.d.ts:287

The version of the property, if applicable.
