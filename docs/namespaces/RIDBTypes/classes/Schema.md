[**@trust0/ridb**](../../../README.md) • **Docs**

***

[@trust0/ridb](../../../README.md) / [RIDBTypes](../README.md) / Schema

# Class: Schema\<T\>

Represents a schema, including its definition and related methods.

## Type Parameters

• **T** *extends* [`SchemaType`](../type-aliases/SchemaType.md)

The schema type.

## Properties

### indexes?

> `readonly` `optional` **indexes**: `string`[]

An optional array of indexes.

#### Defined in

pkg/ridb\_rust.d.ts:469

***

### primaryKey

> `readonly` **primaryKey**: `string`

The primary key of the schema.

#### Defined in

pkg/ridb\_rust.d.ts:459

***

### properties

> `readonly` **properties**: \{ \[K in string \| number \| symbol as T\["properties"\]\[K\]\["required"\] extends false ? K : never\]?: T\["properties"\]\[K\] \} & \{ \[K in string \| number \| symbol as T\["properties"\]\[K\]\["required"\] extends false ? never : K\]: T\["properties"\]\[K\] \}

The properties defined in the schema.

#### Defined in

pkg/ridb\_rust.d.ts:478

***

### schema

> **schema**: [`Schema`](Schema.md)\<`T`\>

The schema definition.

#### Defined in

pkg/ridb\_rust.d.ts:440

***

### type

> `readonly` **type**: `string`

The type of the schema.

#### Defined in

pkg/ridb\_rust.d.ts:464

***

### version

> `readonly` **version**: `number`

The version of the schema.

#### Defined in

pkg/ridb\_rust.d.ts:454

## Methods

### toJSON()

> **toJSON**(): [`SchemaType`](../type-aliases/SchemaType.md)

Converts the schema to a JSON representation.

#### Returns

[`SchemaType`](../type-aliases/SchemaType.md)

The JSON representation of the schema.

#### Defined in

pkg/ridb\_rust.d.ts:488

***

### create()

> `static` **create**\<`TS`\>(`definition`): [`Schema`](Schema.md)\<`TS`\>

Creates a new `Schema` instance from the provided definition.

#### Type Parameters

• **TS** *extends* [`SchemaType`](../type-aliases/SchemaType.md)

The schema type.

#### Parameters

• **definition**: `TS`

#### Returns

[`Schema`](Schema.md)\<`TS`\>

The created `Schema` instance.

#### Defined in

pkg/ridb\_rust.d.ts:449
