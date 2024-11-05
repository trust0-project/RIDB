[**@trust0/ridb**](../README.md) • **Docs**

***

[@trust0/ridb](../README.md) / RIDB

# Class: RIDB\<T\>

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
await db.start()
```

### Using with encryption plugin
You can also optionally specify storageType with a compatible storage of your choice and an optional password to enable encryption plugin
```typescript
await db.start({
    password: "my-password"
})
```

A compatible storage should be a class implementing [StorageInternal<RIDBTypes.SchemaType> ](../namespaces/RIDBTypes/classes/StorageInternal.md) and its methods.

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

await db.start({storageType: storage})
```

## Type Parameters

• **T** *extends* [`SchemaTypeRecord`](../namespaces/RIDBTypes/type-aliases/SchemaTypeRecord.md)

The type of the schema record.

## Constructors

### new RIDB()

> **new RIDB**\<`T`\>(`options`): [`RIDB`](RIDB.md)\<`T`\>

Creates an instance of RIDB.

#### Parameters

• **options**: `object` & `MigrationsParameter`\<`T`\>

#### Returns

[`RIDB`](RIDB.md)\<`T`\>

#### Defined in

[ts/src/index.ts:145](https://github.com/elribonazo/RIDB/blob/18ff16c9e22fc67c657be965ea0b226dde8a7772/ts/src/index.ts#L145)

## Accessors

### collections

#### Get Signature

> **get** **collections**(): \{ \[name in string \| number \| symbol\]: Collection\<Schema\<T\[name\]\>\> \}

Gets the collections from the database.

##### Returns

\{ \[name in string \| number \| symbol\]: Collection\<Schema\<T\[name\]\>\> \}

The collections object.

#### Defined in

[ts/src/index.ts:184](https://github.com/elribonazo/RIDB/blob/18ff16c9e22fc67c657be965ea0b226dde8a7772/ts/src/index.ts#L184)

## Methods

### start()

> **start**(`options`?): `Promise`\<[`Database`](../namespaces/RIDBTypes/classes/Database.md)\<`T`\>\>

Starts the database.

#### Parameters

• **options?**

• **options.password?**: `string`

• **options.storageType?**: *typeof* [`BaseStorage`](../namespaces/RIDBTypes/classes/BaseStorage.md)

#### Returns

`Promise`\<[`Database`](../namespaces/RIDBTypes/classes/Database.md)\<`T`\>\>

A promise that resolves to the database instance.

#### Defined in

[ts/src/index.ts:225](https://github.com/elribonazo/RIDB/blob/18ff16c9e22fc67c657be965ea0b226dde8a7772/ts/src/index.ts#L225)
