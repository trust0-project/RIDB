[**@trust0/ridb**](../README.md)

***

[@trust0/ridb](../README.md) / Property

# Class: Property

Defined in: ridb-core/pkg/ridb\_core.d.ts:647

Represents a property within a schema, including various constraints and nested properties.

## Constructors

### new Property()

> **new Property**(): [`Property`](Property.md)

#### Returns

[`Property`](Property.md)

## Properties

### default?

> `readonly` `optional` **default**: `any`

Defined in: ridb-core/pkg/ridb\_core.d.ts:696

An optional default value for the property.

***

### items?

> `readonly` `optional` **items**: [`Property`](Property.md)

Defined in: ridb-core/pkg/ridb\_core.d.ts:666

An optional array of nested properties for array-type properties.

***

### maxItems?

> `readonly` `optional` **maxItems**: `number`

Defined in: ridb-core/pkg/ridb\_core.d.ts:671

The maximum number of items for array-type properties, if applicable.

***

### maxLength?

> `readonly` `optional` **maxLength**: `number`

Defined in: ridb-core/pkg/ridb\_core.d.ts:681

The maximum length for string-type properties, if applicable.

***

### minItems?

> `readonly` `optional` **minItems**: `number`

Defined in: ridb-core/pkg/ridb\_core.d.ts:676

The minimum number of items for array-type properties, if applicable.

***

### minLength?

> `readonly` `optional` **minLength**: `number`

Defined in: ridb-core/pkg/ridb\_core.d.ts:686

The minimum length for string-type properties, if applicable.

***

### primaryKey?

> `readonly` `optional` **primaryKey**: `string`

Defined in: ridb-core/pkg/ridb\_core.d.ts:661

The primary key of the property, if applicable.

***

### properties?

> `readonly` `optional` **properties**: `object`

Defined in: ridb-core/pkg/ridb\_core.d.ts:701

An optional map of nested properties for object-type properties.

#### Index Signature

\[`name`: `string`\]: [`Property`](Property.md)

***

### required?

> `readonly` `optional` **required**: `boolean`

Defined in: ridb-core/pkg/ridb\_core.d.ts:691

An optional array of required fields for object-type properties.

***

### type

> `readonly` **type**: `string`

Defined in: ridb-core/pkg/ridb\_core.d.ts:651

The type of the property.

***

### version?

> `readonly` `optional` **version**: `number`

Defined in: ridb-core/pkg/ridb\_core.d.ts:656

The version of the property, if applicable.
