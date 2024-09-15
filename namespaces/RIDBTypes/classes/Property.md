[**@elribonazo/ridb**](../../../README.md) • **Docs**

***

[@elribonazo/ridb](../../../README.md) / [RIDBTypes](../README.md) / Property

# Class: Property

Represents a property within a schema, including various constraints and nested properties.

## Constructors

### new Property()

> **new Property**(): [`Property`](Property.md)

#### Returns

[`Property`](Property.md)

## Properties

### items?

> `readonly` `optional` **items**: [`Property`](Property.md)[]

An optional array of nested properties for array-type properties.

#### Defined in

pkg/ridb\_rust.d.ts:567

***

### maxItems?

> `readonly` `optional` **maxItems**: `number`

The maximum number of items for array-type properties, if applicable.

#### Defined in

pkg/ridb\_rust.d.ts:572

***

### maxLength?

> `readonly` `optional` **maxLength**: `number`

The maximum length for string-type properties, if applicable.

#### Defined in

pkg/ridb\_rust.d.ts:582

***

### minItems?

> `readonly` `optional` **minItems**: `number`

The minimum number of items for array-type properties, if applicable.

#### Defined in

pkg/ridb\_rust.d.ts:577

***

### minLength?

> `readonly` `optional` **minLength**: `number`

The minimum length for string-type properties, if applicable.

#### Defined in

pkg/ridb\_rust.d.ts:587

***

### primaryKey?

> `readonly` `optional` **primaryKey**: `string`

The primary key of the property, if applicable.

#### Defined in

pkg/ridb\_rust.d.ts:562

***

### properties?

> `readonly` `optional` **properties**: `object`

An optional map of nested properties for object-type properties.

#### Index Signature

 \[`name`: `string`\]: [`Property`](Property.md)

#### Defined in

pkg/ridb\_rust.d.ts:597

***

### required?

> `readonly` `optional` **required**: `boolean`

An optional array of required fields for object-type properties.

#### Defined in

pkg/ridb\_rust.d.ts:592

***

### type

> `readonly` **type**: `string`

The type of the property.

#### Defined in

pkg/ridb\_rust.d.ts:552

***

### version?

> `readonly` `optional` **version**: `number`

The version of the property, if applicable.

#### Defined in

pkg/ridb\_rust.d.ts:557
