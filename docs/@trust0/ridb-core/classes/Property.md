[**Documentation**](../../../README.md)

***

[Documentation](../../../README.md) / [@trust0/ridb-core](../README.md) / Property

# Class: Property

Defined in: ridb\_core.d.ts:523

Represents a property within a schema, including various constraints and nested properties.

## Constructors

### Constructor

> **new Property**(): `Property`

#### Returns

`Property`

## Properties

### default?

> `readonly` `optional` **default?**: `any`

Defined in: ridb\_core.d.ts:572

An optional default value for the property.

***

### items?

> `readonly` `optional` **items?**: `Property`

Defined in: ridb\_core.d.ts:542

An optional array of nested properties for array-type properties.

***

### maxItems?

> `readonly` `optional` **maxItems?**: `number`

Defined in: ridb\_core.d.ts:547

The maximum number of items for array-type properties, if applicable.

***

### maxLength?

> `readonly` `optional` **maxLength?**: `number`

Defined in: ridb\_core.d.ts:557

The maximum length for string-type properties, if applicable.

***

### minItems?

> `readonly` `optional` **minItems?**: `number`

Defined in: ridb\_core.d.ts:552

The minimum number of items for array-type properties, if applicable.

***

### minLength?

> `readonly` `optional` **minLength?**: `number`

Defined in: ridb\_core.d.ts:562

The minimum length for string-type properties, if applicable.

***

### primaryKey?

> `readonly` `optional` **primaryKey?**: `string`

Defined in: ridb\_core.d.ts:537

The primary key of the property, if applicable.

***

### properties?

> `readonly` `optional` **properties?**: `object`

Defined in: ridb\_core.d.ts:577

An optional map of nested properties for object-type properties.

#### Index Signature

\[`name`: `string`\]: `Property`

***

### required?

> `readonly` `optional` **required?**: `boolean`

Defined in: ridb\_core.d.ts:567

An optional array of required fields for object-type properties.

***

### type

> `readonly` **type**: `SchemaFieldType`

Defined in: ridb\_core.d.ts:527

The type of the property.

***

### version?

> `readonly` `optional` **version?**: `number`

Defined in: ridb\_core.d.ts:532

The version of the property, if applicable.
