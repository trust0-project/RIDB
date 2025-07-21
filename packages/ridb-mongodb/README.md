<p align="center">
  <img src="../../resources/ridb-dark.svg" alt="JavaScript Database" />
  <br />
  <br />
  <h3 align="center">A secure light-weight and dependency free database wrapper for the web.</h3>
</p>
<p align="center">
    <a href="https://github.com/trust0-project/RIDB/releases"><img src="https://img.shields.io/github/v/release/trust0-project/ridb?color=%23ff00a0&include_prereleases&label=version&sort=semver&style=flat-square"></a>
    <a href="#"><img src="https://img.shields.io/npm/types/rxdb?style=flat-square"></a>
    <a href="https://raw.githubusercontent.com/trust0-project/RIDB/refs/heads/main/LICENSE"><img src="https://img.shields.io/github/license/trust0-project/ridb?style=flat-square"></a>
    <a href="https://www.npmjs.com/package/@trust0/ridb"><img src="https://img.shields.io/npm/dm/@trust0/ridb?color=c63a3b&style=flat-square"></a>   
</p>

### Description
This package is a MongoDB wrapper for the RIDB database, providing full CRUD operations and query support for MongoDB databases.

## Installation & usage (typescript)
```bash
npm install @trust0/ridb @trust0/ridb-mongodb -S 
```
Using yarn:
```bash
yarn add @trust0/ridb @trust0/ridb-mongodb
```

## Usage

The following example demonstrates how to create a database with a MongoDB storage engine and a simple schema with ID primary key as string.

```typescript
import { RIDB, SchemaFieldType, Doc } from '@trust0/ridb';
import createMongoDB from '@trust0/ridb-mongodb';

const MongoDB = await createMongoDB();

const db = new RIDB(
    {
        dbName: "test",
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

await db.start({
    storageType: MongoDB,
    password: "test",
    // MongoDB-specific options
    url: "mongodb://localhost:27017", // Optional: defaults to mongodb://localhost:27017
    dbName: "myapp", // Optional: defaults to the RIDB dbName
    mongoOptions: { // Optional: MongoDB client options
        maxPoolSize: 10,
        serverSelectionTimeoutMS: 5000,
    }
});
```

## MongoDB Configuration

The MongoDB storage accepts the following configuration options:

- `url`: MongoDB connection string (defaults to `mongodb://localhost:27017` or `process.env.MONGODB_URL`)
- `dbName`: Database name in MongoDB (defaults to the RIDB database name)
- `mongoOptions`: Additional MongoDB client options (optional)

## Environment Variables

You can also configure the MongoDB connection using environment variables:

- `MONGODB_URL`: MongoDB connection string

