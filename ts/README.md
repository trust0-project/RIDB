<p align="center">
  <img src="https://cdn.jsdelivr.net/gh/trust0-project/ridb@0.4.3/docs/logo.svg" alt="JavaScript Database" />
  <br />
  <br />
  <h3 align="center">A secure light-weight and dependency free database wrapper for the web.</h3>
</p>


<p align="center">
    <a href="https://github.com/trust0-project/RIDB/releases"><img src="https://img.shields.io/github/v/release/trust0-project/ridb?color=%23ff00a0&include_prereleases&label=version&sort=semver&style=flat-square"></a>
    &nbsp;
    <a href="#"><img src="https://img.shields.io/npm/types/rxdb?style=flat-square"></a>
    &nbsp;
    <a href="https://raw.githubusercontent.com/trust0-project/RIDB/refs/heads/main/LICENSE"><img src="https://img.shields.io/github/license/trust0-project/ridb?style=flat-square"></a>
    &nbsp;
    <a href="https://www.npmjs.com/package/@trust0/ridb"><img src="https://img.shields.io/npm/dm/@trust0/ridb?color=c63a3b&style=flat-square"></a>   
</p>



## Install
In order to install simply run the following command
npm:
``` 
npm i @elribonazo/ridb --save
```

yarn:

``` 
yarn add @elribonazo/ridb
```

## Usage
Creating your own database is pretty straight forward.

```javascript
import {
    RIDB,
    SchemaFieldType
} from '@elribonazo/ridb';

(async () => {
    const db =  new RIDB({
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
    });
    console.log("Starting the database");
    await db.start();
    console.log("Ok :)");
})()
```

## Specification

### Storage
A valid storage must extend [BaseStorage class](https://github.com/trust0-project/RIDB/blob/main/docsnamespaces/RIDBTypes/classes/BaseStorage.md)
here's some example:

```typescript
export class InMemory<T extends SchemaType> extends BaseStorage<T>   {
    async write(operation:Operation<T>): Promise<Doc<T>> {
        if (operation.opType === OpType.CREATE) {
            return operation.data;
        }
        throw new Error("Method not implemented.");
    }

    query(): Promise<void> {
        throw new Error("Method not implemented.");
    }

    findDocumentById(id: string): Promise<null> {
        throw new Error("Method not implemented.");
    }

    count(): Promise<number> {
        throw new Error("Method not implemented.");
    }

    close(): Promise<void> {
        throw new Error("Method not implemented.");
    }

}
```