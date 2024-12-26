import { describe, it, expect } from 'vitest';
import { v4 as uuidv4 } from 'uuid';
import { RIDB, SchemaFieldType } from '@trust0/ridb';

import { LevelDB } from '../src/index';



describe(`Testing Storage`, () => {
    it('It should be able to create a new document from JSON schema', async () => {
        const db = new RIDB(
            {
                dbName: "test" + uuidv4(),
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
            storageType: LevelDB,
            password: "test"
        });
        expect(db).to.not.be.undefined;
        expect(db.collections).to.not.be.undefined;
        expect(db.collections).to.haveOwnProperty("demo");
        expect(db.collections.demo).to.not.be.undefined;
        expect(db.collections.demo.find).to.not.be.undefined;
        const created = await db.collections.demo.create({
            id: "12345"
        })
        expect(created).to.not.be.undefined;
        expect(created).to.haveOwnProperty("id");
        expect(created.id).to.eq("12345")

        const found = await db.collections.demo.find({})
    })

})
