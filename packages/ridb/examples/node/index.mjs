
import { RIDB, SchemaFieldType, StorageType } from "@trust0/ridb";

(async () => {
    const db = new RIDB(
        {
            dbName: "testdb", 
            schemas: {
                demo: {
                    version: 0,
                    primaryKey: 'id',
                    type: SchemaFieldType.object,
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
            }
        }
    );
    await db.start({ 
        password: "123456", 
        storageType: StorageType.InMemory
    });
    console.log("started")
})()