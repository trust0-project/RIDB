import { describe, it, expect } from 'vitest';
import { v4 as uuidv4 } from 'uuid';
import { SchemaFieldType, RIDB } from '../..';
import { StoragesType } from '../shared';


export default (platform: string, storages: StoragesType[]) => {

    return describe(`[${platform}] Testing`, () => {

        storages.forEach(({ name, storage }) => {
            describe(`[${platform}][${storage ? 'Typescript' : 'Wasm'} ${name}] Testing Storage`, () => {
                it('Performance test: Create records', async () => {
                    const db = new RIDB({
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
                                    },
                                    name: {
                                        type: SchemaFieldType.string,
                                        maxLength: 100
                                    }
                                }
                            }
                        } as const
                    });

                    await db.start({
                        storageType: storage,
                        password: "test"
                    });

                    const recordCounts = [100, 1000, 10000]; // Different record counts
                    for (const count of recordCounts) {
                        const records: typeof db['schemas']['demo'][] = [];
                        for (let i = 0; i < count; i++) {
                            records.push({ id: `id_${i}`, name: `name_${i}` });
                        }

                        const startTime = performance.now();

                        for (const record of records) {
                            await db.collections.demo.create(record);
                        }

                        const endTime = performance.now();
                        console.log(`Time to create ${count} records on ${name}: ${endTime - startTime} ms`);
                    }
                }, {timeout: 100000});

                it('Performance test: Query records', async () => {
                    const db = new RIDB({
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
                                    },
                                    name: {
                                        type: SchemaFieldType.string,
                                        maxLength: 100
                                    },
                                    value: {
                                        type: SchemaFieldType.number
                                    }
                                }
                            }
                        } as const
                    });

                    await db.start({
                        storageType: storage,
                        password: "test"
                    });

                    // Insert records for querying
                    const totalRecords = 10000;
                    for (let i = 0; i < totalRecords; i++) {
                        await db.collections.demo.create({
                            id: `id_${i}`,
                            name: `name_${i}`,
                            value: i
                        });
                    }

                    const queryCounts = [100, 1000, 5000]; // Different query sizes

                    for (const qCount of queryCounts) {
                        const startTime = performance.now();

                        const results = await db.collections.demo.find({
                            value: {
                                $lt: qCount
                            }
                        });

                        const endTime = performance.now();
                        console.log(`Time to query ${qCount} records on ${name}: ${endTime - startTime} ms`);
                        expect(results.length).to.eq(qCount);
                    }
                }, {timeout: 100000});
            })
        })
    });
}