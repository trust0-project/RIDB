[**@trust0/ridb**](../README.md)

***

[@trust0/ridb](../README.md) / SchemaFieldType

# Variable: SchemaFieldType

> `const` **SchemaFieldType**: `object`

Defined in: [index.ts:286](https://github.com/trust0-project/RIDB/blob/c6b66934724268652b2a7a7223d024e63cb2d559/packages/ridb/src/index.ts#L286)

An enumeration of schema field types for defining document structures.

These types correspond to JavaScript primitive types and are used when
defining schemas for RIDB collections.

## Type declaration

### array

> **array**: `"array"`

Array type for ordered collections of items

### boolean

> **boolean**: `"boolean"`

Boolean type for true/false values

### number

> **number**: `"number"`

Number type for numeric data (integers and floats)

### object

> **object**: `"object"`

Object type for nested document structures

### string

> **string**: `"string"`

String type for text data

## Example

```typescript
// Define a schema with different field types
const schema = {
  version: 1,
  primaryKey: 'id',
  type: SchemaFieldType.object,
  properties: {
    id: { type: SchemaFieldType.string },
    age: { type: SchemaFieldType.number },
    isActive: { type: SchemaFieldType.boolean },
    tags: { type: SchemaFieldType.array },
    address: { type: SchemaFieldType.object }
  }
};
```
