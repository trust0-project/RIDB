[**@trust0/ridb**](../README.md)

***

[@trust0/ridb](../README.md) / Schema

# Class: Schema\<T\>

Defined in: ridb-core/pkg/ridb\_core.d.ts:434

Represents a schema, including its definition and related methods.
You may be trying to build a storage, in any other can u won't need access tho this class.
Check this example 

```typescript
class MyStorage extends <T extends SchemaTypeRecord> extends BaseStorage<T> {
 example() {
   const schema: Schema<any> = this.getSchema("mySchema")
 }
}
```
You alwayswill have access to getSchema through the Storage class.

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

> `readonly` `optional` **encrypted**: `Extract`\<keyof `T`, `string`\>[]

Defined in: ridb-core/pkg/ridb\_core.d.ts:475

An optional array of encrypted fields.

***

### indexes?

> `readonly` `optional` **indexes**: `Extract`\<keyof `T`, `string`\>[]

Defined in: ridb-core/pkg/ridb\_core.d.ts:470

An optional array of indexes.

***

### primaryKey

> `readonly` **primaryKey**: `string`

Defined in: ridb-core/pkg/ridb\_core.d.ts:457

The primary key of the schema.

***

### properties

> `readonly` **properties**: \{ \[K in string \| number \| symbol as T\["properties"\]\[K\]\["required"\] extends false \| (T\["properties"\]\[K\]\["default"\] extends undefined ? true : false) ? K : never\]?: T\["properties"\]\[K\] \} & \{ \[K in string \| number \| symbol as T\["properties"\]\[K\]\["required"\] extends false ? never : K\]: T\["properties"\]\[K\] \}

Defined in: ridb-core/pkg/ridb\_core.d.ts:480

The properties defined in the schema.

***

### schema

> **schema**: [`Schema`](Schema.md)\<`T`\>

Defined in: ridb-core/pkg/ridb\_core.d.ts:438

The schema definition.

***

### type

> `readonly` **type**: `string`

Defined in: ridb-core/pkg/ridb\_core.d.ts:462

The type of the schema.

***

### version

> `readonly` **version**: `number`

Defined in: ridb-core/pkg/ridb\_core.d.ts:452

The version of the schema.

## Methods

### toJSON()

> **toJSON**(): [`SchemaType`](../type-aliases/SchemaType.md)

Defined in: ridb-core/pkg/ridb\_core.d.ts:490

Converts the schema to a JSON representation.

#### Returns

[`SchemaType`](../type-aliases/SchemaType.md)

The JSON representation of the schema.

***

### validate()

> **validate**(`document`): `boolean`

Defined in: ridb-core/pkg/ridb\_core.d.ts:492

#### Parameters

##### document

[`Doc`](../type-aliases/Doc.md)\<[`Schema`](Schema.md)\<`T`\>\>

#### Returns

`boolean`

***

### create()

> `static` **create**\<`TS`\>(`definition`): [`Schema`](Schema.md)\<`TS`\>

Defined in: ridb-core/pkg/ridb\_core.d.ts:447

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
