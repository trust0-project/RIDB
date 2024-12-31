[**@trust0/ridb**](../README.md)

***

[@trust0/ridb](../README.md) / Property

# Class: Property

Represents a property within a schema, including various constraints and nested properties.

## Constructors

### new Property()

> **new Property**(): [`Property`](Property.md)

#### Returns

[`Property`](Property.md)

## Properties

### default?

> `readonly` `optional` **default**: `any`

An optional default value for the property.

#### Defined in

ridb-wasm/pkg/ridb\_wasm.d.ts:523

***

### items?

> `readonly` `optional` **items**: [`Property`](Property.md)

An optional array of nested properties for array-type properties.

#### Defined in

ridb-wasm/pkg/ridb\_wasm.d.ts:493

***

### maxItems?

> `readonly` `optional` **maxItems**: `number`

The maximum number of items for array-type properties, if applicable.

#### Defined in

ridb-wasm/pkg/ridb\_wasm.d.ts:498

***

### maxLength?

> `readonly` `optional` **maxLength**: `number`

The maximum length for string-type properties, if applicable.

#### Defined in

ridb-wasm/pkg/ridb\_wasm.d.ts:508

***

### minItems?

> `readonly` `optional` **minItems**: `number`

The minimum number of items for array-type properties, if applicable.

#### Defined in

ridb-wasm/pkg/ridb\_wasm.d.ts:503

***

### minLength?

> `readonly` `optional` **minLength**: `number`

The minimum length for string-type properties, if applicable.

#### Defined in

ridb-wasm/pkg/ridb\_wasm.d.ts:513

***

### primaryKey?

> `readonly` `optional` **primaryKey**: `string`

The primary key of the property, if applicable.

#### Defined in

ridb-wasm/pkg/ridb\_wasm.d.ts:488

***

### properties?

> `readonly` `optional` **properties**: `object`

An optional map of nested properties for object-type properties.

#### Index Signature

 \[`name`: `string`\]: [`Property`](Property.md)

#### Defined in

ridb-wasm/pkg/ridb\_wasm.d.ts:528

***

### required?

> `readonly` `optional` **required**: `boolean`

An optional array of required fields for object-type properties.

#### Defined in

ridb-wasm/pkg/ridb\_wasm.d.ts:518

***

### type

> `readonly` **type**: `string`

The type of the property.

#### Defined in

ridb-wasm/pkg/ridb\_wasm.d.ts:478

***

### version?

> `readonly` `optional` **version**: `number`

The version of the property, if applicable.

#### Defined in

ridb-wasm/pkg/ridb\_wasm.d.ts:483
