<p align="center">
  <img src="https://cdn.jsdelivr.net/gh/trust0-project/ridb@latest/docs/logo.svg" alt="JavaScript Database" />
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

## Documentation
This package provides everything u need to use RIDB on react easily

## Install
```
npm i @trust0/ridb-react
```

## Usage
```typescript
import React from 'react'
import { Database, useDatabase } from '@trust0/ridb-react'
```

Create your schemas and type them for better inference.

```typescript
const users = {
  version: 0 as const,
  primaryKey: 'id',
  type: SchemaFieldType.object,
  properties: {
      id: {
          type: SchemaFieldType.string,
          maxLength: 60
      }
  }
} as const
const schemas = {
  users: users
}
type DatabaseSchemas = typeof schemas;
```

Now just create your component and use the `useDatabase` hook to get the database instance.

```typescript
const MyComponent: React.FC = () => {
    const db = useDatabase<DatabaseSchemas>();
    const [isDbReady, setIsDbReady] = React.useState(false);

    React.useEffect(() => {
        const startDb = async () => {
            if (db) {
                await db.start();
                setIsDbReady(true);
            }
        };
        startDb();
    }, [db]);

    if (!db) {
        return <div>No database available</div>;
    }

    if (!isDbReady) {
        return <div>Loading...</div>;
    }

    return (
        <div> <h1>My Component</h1> </div>
    );
};
```

All the database methods and operations from RIDB are supported, for more details check the [RIDB documentation](https://github.com/trust0-project/RIDB/packages/ridb/README.md)