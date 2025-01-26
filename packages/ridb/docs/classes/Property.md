[**@trust0/ridb**](../README.md)

***

[@trust0/ridb](../README.md) / Property

# Class: Property

Defined in: ridb-core/pkg/ridb\_core.d.ts:421

Represents a property within a schema, including various constraints and nested properties.

## Constructors

### new Property()

> **new Property**(): [`Property`](Property.md)

#### Returns

[`Property`](Property.md)

## Properties

### default?

> `readonly` `optional` **default**: `any`

Defined in: ridb-core/pkg/ridb\_core.d.ts:470

An optional default value for the property.

***

### items?

> `readonly` `optional` **items**: [`Property`](Property.md)

Defined in: ridb-core/pkg/ridb\_core.d.ts:440

An optional array of nested properties for array-type properties.

***

### maxItems?

> `readonly` `optional` **maxItems**: `number`

Defined in: ridb-core/pkg/ridb\_core.d.ts:445

The maximum number of items for array-type properties, if applicable.

***

### maxLength?

> `readonly` `optional` **maxLength**: `number`

Defined in: ridb-core/pkg/ridb\_core.d.ts:455

The maximum length for string-type properties, if applicable.

***

### minItems?

> `readonly` `optional` **minItems**: `number`

Defined in: ridb-core/pkg/ridb\_core.d.ts:450

The minimum number of items for array-type properties, if applicable.

***

### minLength?

> `readonly` `optional` **minLength**: `number`

Defined in: ridb-core/pkg/ridb\_core.d.ts:460

The minimum length for string-type properties, if applicable.

***

### primaryKey?

> `readonly` `optional` **primaryKey**: `string`

Defined in: ridb-core/pkg/ridb\_core.d.ts:435

The primary key of the property, if applicable.

***

### properties?

> `readonly` `optional` **properties**: `object`

Defined in: ridb-core/pkg/ridb\_core.d.ts:475

An optional map of nested properties for object-type properties.

#### Index Signature

\[`name`: `string`\]: [`Property`](Property.md)

***

### required?

> `readonly` `optional` **required**: `boolean`

Defined in: ridb-core/pkg/ridb\_core.d.ts:465

An optional array of required fields for object-type properties.

***

### type

> `readonly` **type**: `string`

Defined in: ridb-core/pkg/ridb\_core.d.ts:425

The type of the property.

***

### version?

> `readonly` `optional` **version**: `number`

Defined in: ridb-core/pkg/ridb\_core.d.ts:430

The version of the property, if applicable.
