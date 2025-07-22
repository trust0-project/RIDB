import { RIDB } from '@trust0/ridb';
import { createMongoDB } from '@trust0/ridb-mongodb';
import { SchemaFieldType } from '@trust0/ridb-core';

;(async () => {
    try {

        const storage = await createMongoDB();
        const schemas = {
            demo: {
                version: 0,
                primaryKey: 'id',
                type: SchemaFieldType.object,
                indexes: ['email'],
                properties: {
                    id: { type: SchemaFieldType.string, required: true },
                    email: { type: SchemaFieldType.string, required: true }
                }
            }
        };
        const db = new RIDB({
            dbName: "test-index" + crypto.randomUUID(),
            schemas
        });

        await db.start({
            storageType: storage,
            password: "test"
        });

        db.collections.demo.create({        
            id: "1" + crypto.randomUUID(),
            email: "test@test.com"
        });

        await db.collections.demo.find({
            email: "test@test.com"
        });
        
    } catch (error) {
        console.error(error.message);
    }
})()
