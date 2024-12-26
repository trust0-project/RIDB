[**@trust0/ridb**](../README.md)

***

[@trust0/ridb](../README.md) / Property

# Class: Property

Represents a property within a schema, including various constraints and nested properties.

## Properties

### default?

> `readonly` `optional` **default**: `any`

An optional default value for the property.

#### Defined in

ridb-wasm/pkg/ridb\_wasm.d.ts:432

***

### items?

> `readonly` `optional` **items**: [`Property`](Property.md)[]

An optional array of nested properties for array-type properties.

#### Defined in

ridb-wasm/pkg/ridb\_wasm.d.ts:402

***

### maxItems?

> `readonly` `optional` **maxItems**: `number`

The maximum number of items for array-type properties, if applicable.

#### Defined in

ridb-wasm/pkg/ridb\_wasm.d.ts:407

***

### maxLength?

> `readonly` `optional` **maxLength**: `number`

The maximum length for string-type properties, if applicable.

#### Defined in

ridb-wasm/pkg/ridb\_wasm.d.ts:417

***

### minItems?

> `readonly` `optional` **minItems**: `number`

The minimum number of items for array-type properties, if applicable.

#### Defined in

ridb-wasm/pkg/ridb\_wasm.d.ts:412

***

### minLength?

> `readonly` `optional` **minLength**: `number`

The minimum length for string-type properties, if applicable.

#### Defined in

ridb-wasm/pkg/ridb\_wasm.d.ts:422

***

### primaryKey?

> `readonly` `optional` **primaryKey**: `string`

The primary key of the property, if applicable.

#### Defined in

ridb-wasm/pkg/ridb\_wasm.d.ts:397

***

### properties?

> `readonly` `optional` **properties**: `object`

An optional map of nested properties for object-type properties.

#### Index Signature

 \[`name`: `string`\]: [`Property`](Property.md)

#### Defined in

ridb-wasm/pkg/ridb\_wasm.d.ts:437

***

### required?

> `readonly` `optional` **required**: `boolean`

An optional array of required fields for object-type properties.

#### Defined in

ridb-wasm/pkg/ridb\_wasm.d.ts:427

***

### type

> `readonly` **type**: `string`

The type of the property.

#### Defined in

ridb-wasm/pkg/ridb\_wasm.d.ts:387

***

### version?

> `readonly` `optional` **version**: `number`

The version of the property, if applicable.

#### Defined in

ridb-wasm/pkg/ridb\_wasm.d.ts:392
