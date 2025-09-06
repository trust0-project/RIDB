[**Documentation**](../../../README.md)

***

[Documentation](../../../README.md) / [@trust0/ridb-core](../README.md) / Schema

# Class: Schema\<T\>

Defined in: ridb\_core.d.ts:400

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

### T

`T` *extends* [`SchemaType`](../type-aliases/SchemaType.md)

The schema type.

## Constructors

### Constructor

> **new Schema**\<`T`\>(): `Schema`\<`T`\>

#### Returns

`Schema`\<`T`\>

## Properties

### encrypted?

> `readonly` `optional` **encrypted**: `Extract`\<keyof `T`, `string`\>[]

Defined in: ridb\_core.d.ts:441

An optional array of encrypted fields.

***

### indexes?

> `readonly` `optional` **indexes**: `Extract`\<keyof `T`, `string`\>[]

Defined in: ridb\_core.d.ts:436

An optional array of indexes.

***

### primaryKey

> `readonly` **primaryKey**: `string`

Defined in: ridb\_core.d.ts:423

The primary key of the schema.

***

### properties

> `readonly` **properties**: \{ \[K in string \| number \| symbol as T\["properties"\]\[K\]\["required"\] extends false \| (T\["properties"\]\[K\]\["default"\] extends undefined ? true : false) ? K : never\]?: T\["properties"\]\[K\] \} & \{ \[K in string \| number \| symbol as T\["properties"\]\[K\]\["required"\] extends false ? never : K\]: T\["properties"\]\[K\] \}

Defined in: ridb\_core.d.ts:446

The properties defined in the schema.

***

### schema

> **schema**: `Schema`\<`T`\>

Defined in: ridb\_core.d.ts:404

The schema definition.

***

### type

> `readonly` **type**: `SchemaFieldType`

Defined in: ridb\_core.d.ts:428

The type of the schema.

***

### version

> `readonly` **version**: `number`

Defined in: ridb\_core.d.ts:418

The version of the schema.

## Methods

### toJSON()

> **toJSON**(): [`SchemaType`](../type-aliases/SchemaType.md)

Defined in: ridb\_core.d.ts:456

Converts the schema to a JSON representation.

#### Returns

[`SchemaType`](../type-aliases/SchemaType.md)

The JSON representation of the schema.

***

### validate()

> **validate**(`document`): `boolean`

Defined in: ridb\_core.d.ts:458

#### Parameters

##### document

[`Doc`](../type-aliases/Doc.md)\<`Schema`\<`T`\>\>

#### Returns

`boolean`

***

### create()

> `static` **create**\<`TS`\>(`definition`): `Schema`\<`TS`\>

Defined in: ridb\_core.d.ts:413

Creates a new `Schema` instance from the provided definition.

#### Type Parameters

##### TS

`TS` *extends* [`SchemaType`](../type-aliases/SchemaType.md)

The schema type.

#### Parameters

##### definition

`TS`

#### Returns

`Schema`\<`TS`\>

The created `Schema` instance.
