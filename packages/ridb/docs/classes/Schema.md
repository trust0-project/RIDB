[**@trust0/ridb**](../README.md)

***

[@trust0/ridb](../README.md) / Schema

# Class: Schema\<T\>

Represents a schema, including its definition and related methods.

## Type Parameters

• **T** *extends* [`SchemaType`](../type-aliases/SchemaType.md)

The schema type.

## Constructors

### new Schema()

> **new Schema**\<`T`\>(): [`Schema`](Schema.md)\<`T`\>

#### Returns

[`Schema`](Schema.md)\<`T`\>

## Properties

### encrypted?

> `readonly` `optional` **encrypted**: `string`[]

#### Defined in

node\_modules/@trust0/ridb-wasm/ridb\_wasm.d.ts:497

***

### indexes?

> `readonly` `optional` **indexes**: `string`[]

An optional array of indexes.

#### Defined in

node\_modules/@trust0/ridb-wasm/ridb\_wasm.d.ts:493

***

### primaryKey

> `readonly` **primaryKey**: `string`

The primary key of the schema.

#### Defined in

node\_modules/@trust0/ridb-wasm/ridb\_wasm.d.ts:483

***

### properties

> `readonly` **properties**: \{ \[K in string \| number \| symbol as T\["properties"\]\[K\]\["required"\] extends false \| (T\["properties"\]\[K\]\["default"\] extends undefined ? true : false) ? K : never\]?: T\["properties"\]\[K\] \} & \{ \[K in string \| number \| symbol as T\["properties"\]\[K\]\["required"\] extends false ? never : K\]: T\["properties"\]\[K\] \}

The properties defined in the schema.

#### Defined in

node\_modules/@trust0/ridb-wasm/ridb\_wasm.d.ts:502

***

### required?

> `readonly` `optional` **required**: `string`[]

#### Defined in

node\_modules/@trust0/ridb-wasm/ridb\_wasm.d.ts:495

***

### schema

> **schema**: [`Schema`](Schema.md)\<`T`\>

The schema definition.

#### Defined in

node\_modules/@trust0/ridb-wasm/ridb\_wasm.d.ts:464

***

### type

> `readonly` **type**: `string`

The type of the schema.

#### Defined in

node\_modules/@trust0/ridb-wasm/ridb\_wasm.d.ts:488

***

### version

> `readonly` **version**: `number`

The version of the schema.

#### Defined in

node\_modules/@trust0/ridb-wasm/ridb\_wasm.d.ts:478

## Methods

### toJSON()

> **toJSON**(): [`SchemaType`](../type-aliases/SchemaType.md)

Converts the schema to a JSON representation.

#### Returns

[`SchemaType`](../type-aliases/SchemaType.md)

The JSON representation of the schema.

#### Defined in

node\_modules/@trust0/ridb-wasm/ridb\_wasm.d.ts:512

***

### create()

> `static` **create**\<`TS`\>(`definition`): [`Schema`](Schema.md)\<`TS`\>

Creates a new `Schema` instance from the provided definition.

#### Type Parameters

• **TS** *extends* [`SchemaType`](../type-aliases/SchemaType.md)

The schema type.

#### Parameters

##### definition

`TS`

#### Returns

[`Schema`](Schema.md)\<`TS`\>

The created `Schema` instance.

#### Defined in

node\_modules/@trust0/ridb-wasm/ridb\_wasm.d.ts:473
