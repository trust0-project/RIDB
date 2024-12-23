[**@trust0/ridb**](../README.md)

***

[@trust0/ridb](../README.md) / StartOptions

# Type Alias: StartOptions\<T\>

> **StartOptions**\<`T`\>: `object`

Represents a RIDB (Rust IndexedDB) instance.
This is the main class exposed by the RIDB Storage sdk and is used to create a database instance.

### Usage:

```typescript
const db = new RIDB(
    {
        schemas: {
            demo: {
                version: 0,
                primaryKey: 'id',
                type: SchemaFieldType.object,
                properties: {
                    id: {
                        type: SchemaFieldType.string,
                        maxLength: 60
                    }
                }
            }
        } as const
    }
)
```

### Starting the database
```typescript    
await db.start({dbName: "demo"})
```

### Using with encryption plugin
You can also optionally specify storageType with a compatible storage of your choice and an optional password to enable encryption plugin
```typescript
await db.start({
    password: "my-password"
    db
})
```

A compatible storage should be a class implementing [StorageInternal<RIDBTypes.SchemaType> ](../_media/StorageInternal.md) and its methods.

### Using with migration plugin
The migration plugin will automatically migrate your documents for you as you upgrade and change your schemas over the time. 

```typescript
const db = new RIDB(
    {
        schemas: {
            demo: {
                version: 1,
                primaryKey: 'id',
                type: SchemaFieldType.object,
                required:['id', 'age'],
                properties: {
                    id: {
                        type: SchemaFieldType.string,
                        maxLength: 60
                    },
                    age: {
                        type: SchemaFieldType.number,
                    }
                }
            }
        } as const,
        migrations: {
            demo: {
                1: function (doc) {
                    return doc
                }
            }
        }
    }
)

await db.start({dbName: "demo"})
```

## Type Parameters

• **T** *extends* `RIDBTypes.SchemaTypeRecord`

The type of the schema record.

## Type declaration

## Index Signature

 \[`name`: `string`\]: `any`

### password?

> `optional` **password**: `string`

### storageType?

> `optional` **storageType**: *typeof* `RIDBTypes.BaseStorage` \| `StorageType`

## Defined in

[index.ts:154](https://github.com/trust0-project/RIDB/blob/4fef281da10b105fc67cc4df20c81fa87000ec0b/src/index.ts#L154)
