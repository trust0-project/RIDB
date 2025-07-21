[**Documentation**](../../../README.md)

***

[Documentation](../../../README.md) / [@trust0/ridb](../README.md) / RIDB

# Class: RIDB\<T\>

Defined in: [index.ts:160](https://github.com/trust0-project/RIDB/blob/ec1e43d2e9f91f06fa489f0836fc75985d814cf0/packages/ridb/src/index.ts#L160)

Main RIDB class that provides database functionality with optional worker support.

This class serves as the primary entry point for interacting with the RIDB database.
It manages the lifecycle of the database connection and provides access to collections.

## Type Parameters

### T

`T` *extends* [`SchemaTypeRecord`](https://github.com/trust0-project/RIDB/blob/main/docs/%40trust0/ridb-core/type-aliases/SchemaTypeRecord.md) = [`SchemaTypeRecord`](https://github.com/trust0-project/RIDB/blob/main/docs/%40trust0/ridb-core/type-aliases/SchemaTypeRecord.md)

Schema type record defining the database schema structure

## Constructors

### Constructor

> **new RIDB**\<`T`\>(`options`): `RIDB`\<`T`\>

Defined in: [index.ts:184](https://github.com/trust0-project/RIDB/blob/ec1e43d2e9f91f06fa489f0836fc75985d814cf0/packages/ridb/src/index.ts#L184)

Creates a new RIDB instance.

#### Parameters

##### options

[`DBOptions`](../type-aliases/DBOptions.md)\<`T`\>

Database configuration options including schemas and optional worker settings

#### Returns

`RIDB`\<`T`\>

#### Example

```typescript
const db = new RIDB({
  schemas: {
    users: {
      version: 1,
      primaryKey: 'id',
      type: SchemaFieldType.object,
      properties: {
        id: { type: SchemaFieldType.string },
        name: { type: SchemaFieldType.string }
      }
    }
  }
});
```

## Accessors

### collections

#### Get Signature

> **get** **collections**(): \{ \[name in string \| number \| symbol\]: Collection\<Schema\<T\[name\]\>\> \}

Defined in: [index.ts:201](https://github.com/trust0-project/RIDB/blob/ec1e43d2e9f91f06fa489f0836fc75985d814cf0/packages/ridb/src/index.ts#L201)

Access the database collections.

##### Example

```typescript
// Get the users collection
const usersCollection = db.collections.users;

// Query documents
const allUsers = await usersCollection.find({}).exec();
```

##### Returns

\{ \[name in string \| number \| symbol\]: Collection\<Schema\<T\[name\]\>\> \}

An object containing all collections defined in the schema

***

### started

#### Get Signature

> **get** **started**(): `boolean`

Defined in: [index.ts:258](https://github.com/trust0-project/RIDB/blob/ec1e43d2e9f91f06fa489f0836fc75985d814cf0/packages/ridb/src/index.ts#L258)

Checks if the database has been successfully started.

##### Example

```typescript
if (db.started) {
  // Database is ready for use
  const docs = await db.collections.users.find({}).exec();
} else {
  // Database needs to be started first
  await db.start();
}
```

##### Returns

`boolean`

True if the database is started, false otherwise

## Methods

### close()

> **close**(): `Promise`\<`void`\>

Defined in: [index.ts:239](https://github.com/trust0-project/RIDB/blob/ec1e43d2e9f91f06fa489f0836fc75985d814cf0/packages/ridb/src/index.ts#L239)

Closes the database connection and releases resources.

#### Returns

`Promise`\<`void`\>

A promise that resolves when the database has been successfully closed

#### Example

```typescript
// Close the database connection
await db.close();
```

***

### start()

> **start**(`options?`): `Promise`\<`void`\>

Defined in: [index.ts:225](https://github.com/trust0-project/RIDB/blob/ec1e43d2e9f91f06fa489f0836fc75985d814cf0/packages/ridb/src/index.ts#L225)

Starts the database and initializes all collections.

#### Parameters

##### options?

[`StartOptions`](../type-aliases/StartOptions.md)\<`T`\>

Optional configuration for startup including storage type and encryption

#### Returns

`Promise`\<`void`\>

A promise that resolves when the database has successfully started

#### Example

```typescript
// Start with default options
await db.start();

// Start with encryption
await db.start({ password: "secure-password" });

// Start with custom storage
await db.start({ 
  storageType: StorageType.IndexDB,
  dbName: "myApp"
});
```
