import { describe, it, expect } from 'vitest';
import { v4 as uuidv4 } from 'uuid';
import { RIDB, SchemaFieldType, Doc } from '@trust0/ridb';
import { StoragesType } from '..';


export default (platform: string, storages: StoragesType[]) => {
    return describe(`[${platform}] Testing`, () => {
        storages.forEach(({ name, storage }) => {
            describe(`[${platform}][${storage ? 'Typescript' : 'Wasm'} ${name}] Testing Storage`, () => {
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
                        storageType: storage,
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
                })
                it("should allow optional fields", async () => {
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
                                        },
                                        name: {
                                            type: SchemaFieldType.string,
                                            maxLength: 20,
                                            required: false
                                        }
                                    }
                                }
                            } as const
                        }
                    )

                    await db.start({
                        storageType: storage,
                        password: "test"
                    });

                    expect(db).to.not.be.undefined;
                    expect(db.collections).to.not.be.undefined;
                    expect(db.collections).to.haveOwnProperty("demo");
                    expect(db.collections.demo).to.not.be.undefined;
                    expect(db.collections.demo.find).to.not.be.undefined;

                    const created = await db.collections.demo.create({
                        id: "12345",
                    });
                    expect(created).to.not.be.undefined;
                    expect(created).to.haveOwnProperty("id");
                    expect(created).to.not.haveOwnProperty("name");

                    expect(created.id).to.eq("12345")

                })
                it("Should allow updating multi model encrypted document", async () => {
                    const db = new RIDB(
                        {
                            dbName: "test" + uuidv4(),
                            schemas: {
                                demo2: {
                                    version: 0,
                                    primaryKey: 'id',
                                    type: SchemaFieldType.object,
                                    encrypted: [],
                                    properties: {
                                        id: {
                                            type: SchemaFieldType.string,
                                            maxLength: 60
                                        },
                                        name: {
                                            type: SchemaFieldType.string,
                                            maxLength: 20
                                        },
                                    }
                                },
                                demo: {
                                    version: 0,
                                    primaryKey: 'id',
                                    type: SchemaFieldType.object,
                                    encrypted: ['name'],
                                    properties: {
                                        id: {
                                            type: SchemaFieldType.string,
                                            maxLength: 60
                                        },
                                        name: {
                                            type: SchemaFieldType.string,
                                            maxLength: 20
                                        },
                                    }
                                }
                            } as const
                        }
                    )

                    await db.start({
                        storageType: storage,
                        password: "test"
                    });

                    expect(db).to.not.be.undefined;
                    expect(db.collections).to.not.be.undefined;
                    expect(db.collections).to.haveOwnProperty("demo");
                    expect(db.collections.demo).to.not.be.undefined;
                    expect(db.collections.demo.find).to.not.be.undefined;

                    const created = await db.collections.demo.create({
                        id: "12345",
                        name: "demo"
                    })
                    expect(created).to.not.be.undefined;
                    expect(created).to.haveOwnProperty("id");
                    expect(created).to.haveOwnProperty("name");
                    expect(created.id).to.eq("12345")
                    expect(created.name).to.eq("demo")

                    await db.collections.demo.update({
                        ...created,
                        name: "demo2"
                    })

                    const result = await db.collections.demo.findById(created.id);
                    expect(result).to.not.be.undefined;
                    expect(result).to.haveOwnProperty("id");
                    expect(result).to.haveOwnProperty("name");
                    expect(result.id).to.eq("12345")
                    expect(result.name).to.eq("demo2")

                })
                it("Should be able to create a default database with a valid schema", async () => {
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
                        storageType: storage,
                        password: "test"
                    })
                    expect(db).to.not.be.undefined;
                });
                it("Should be able to find a created schema entry", async () => {
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
                                        },
                                        age: {
                                            type: SchemaFieldType.number,
                                        }
                                    }
                                }
                            } as const
                        }
                    )
                    await db.start({
                        storageType: storage,
                        password: "test"
                    })
                    expect(db).to.not.be.undefined;

                    const created = await db.collections.demo.create({
                        id: "12345",
                        age: 18
                    })
                    expect(created).to.not.be.undefined;
                    expect(created).to.haveOwnProperty("id");
                    expect(created).to.haveOwnProperty("age");

                    const found = await db.collections.demo.find({
                        age: {
                            $gte: 2
                        },
                        $or: [
                            {
                                age: {
                                    $in: [2, 3, 4, 18, 19, 20],
                                    $gte: 2
                                }
                            }
                        ]
                    })

                    expect(found.length).to.eq(1)

                });
                it("Should be able to count a created schema entry", async () => {
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
                                        },
                                        age: {
                                            type: SchemaFieldType.number,
                                        }
                                    }
                                }
                            } as const,
                        }
                    )
                    await db.start({
                        storageType: storage,
                        password: "test"
                    })
                    expect(db).to.not.be.undefined;

                    const created = await db.collections.demo.create({
                        id: "12345",
                        age: 18
                    })
                    expect(created).to.not.be.undefined;
                    expect(created).to.haveOwnProperty("id");
                    expect(created).to.haveOwnProperty("age");

                    expect(created).to.haveOwnProperty("__version");
                    expect(created.__version).to.eq(0);

                    const found = await db.collections.demo.count({
                        age: {
                            $gte: 2
                        },
                        $or: [
                            {
                                age: {
                                    $in: [2, 3, 4, 18, 19, 20],
                                    $gte: 2
                                }
                            }
                        ]
                    })

                    expect(found).to.eq(1)

                });
                it("Should throw an error with a schema with invalid type", async () => {
                    const db = new RIDB(
                        {
                            dbName: "test" + uuidv4(),
                            schemas: {
                                demo: {
                                    version: 0,
                                    primaryKey: 'id',
                                    type: "wrong",
                                    properties: {}
                                }
                            } as const
                        }
                    )
                    await expect(async () => db.start({
                        storageType: storage,
                        password: "test"
                    })).to.rejects.toThrowError("Validation Error: Schema type is invalid (\"wrong\")")
                })
                it("Should throw an error when schema properties type is invalid", async () => {
                    const db = new RIDB(
                        {
                            dbName: "test" + uuidv4(),
                            schemas: {
                                demo: {
                                    version: 0,
                                    primaryKey: 'id',
                                    type: "obiect",
                                    properties: {
                                        id: {
                                            type: "....",
                                            minLength: -1
                                        }
                                    }
                                }
                            } as const
                        }
                    )
                    await expect(async () => db.start({
                        storageType: storage,
                        password: "test"
                    })).to.rejects.toThrowError("Serialization Error: invalid value: string \"....\", expected an PropertyType (String, Number, Boolean, Object or Array)")
                })
                it("Should throw an error when the minLength is lower than 0", async () => {
                    const db = new RIDB(
                        {
                            dbName: "test" + uuidv4(),
                            schemas: {
                                demo: {
                                    version: 0,
                                    primaryKey: 'id',
                                    type: "object",
                                    properties: {
                                        id: {
                                            type: "string",
                                            minLength: -1
                                        }
                                    }
                                }
                            } as const
                        }
                    )
                    await expect(async () => db.start({
                        storageType: storage,
                        password: "test"
                    })).to.rejects.toThrowError("Validation Error: Min property not valid")
                })
                it("Should throw an error when schemaType with a property that has min higher than max", async () => {
                    const db = new RIDB(
                        {
                            dbName: "test" + uuidv4(),
                            schemas: {
                                demo: {
                                    version: 0,
                                    primaryKey: 'id',
                                    type: "object",
                                    properties: {
                                        id: {
                                            type: "string",
                                            maxLength: 2,
                                            minLength: 3
                                        }
                                    }
                                }
                            } as const
                        }
                    )
                    await expect(async () => db.start({
                        storageType: storage,
                        password: "test"
                    })).to.rejects.toThrowError("Validation Error: Min higher than max")
                });
                it("Should throw an error if migrations are declared wrong", () => {
                    const db = new RIDB(
                        {
                            dbName: "test" + uuidv4(),
                            schemas: {
                                demo: {
                                    version: 1,
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
                            } as const,
                            migrations: {
                                demo: {
                                } as any
                            }
                        }
                    )
                    expect(
                        async () => db.start({
                            storageType: storage,
                            password: "test"
                        })
                    ).to.rejects.toThrowError("Required Schema demo migration path 1 to not be undefined")
                })
                it("Should handle multiple collections independently", async () => {
                    const db = new RIDB(
                        {
                            dbName: "test" + uuidv4(),
                            schemas: {
                                users: {
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
                                            maxLength: 20
                                        }
                                    }
                                },
                                posts: {
                                    version: 0,
                                    primaryKey: 'id',
                                    type: SchemaFieldType.object,
                                    properties: {
                                        id: {
                                            type: SchemaFieldType.string,
                                            maxLength: 60
                                        },
                                        title: {
                                            type: SchemaFieldType.string,
                                            maxLength: 100
                                        }
                                    }
                                }
                            } as const
                        }
                    )

                    await db.start({
                        storageType: storage,
                        password: "test"
                    });

                    // Create a user in the users collection
                    const user = await db.collections.users.create({
                        id: "user1",
                        name: "Test User"
                    });

                    expect(user).to.not.be.undefined;
                    expect(user.id).to.eq("user1");

                    // Verify posts collection exists but is empty
                    const postsCount = await db.collections.posts.count({});
                    expect(postsCount).to.eq(0);

                    const allPosts = await db.collections.posts.find({});
                    expect(allPosts.length).to.eq(0);
                });

                it("Should handle migrations and integrity with default fields", async () => {
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
                                        },
                                        age: {
                                            type: SchemaFieldType.number,
                                            default: 18,
                                            required: false
                                        }
                                    }
                                }
                            } as const,
                        }
                    )

                    await db.start({
                        storageType: storage,
                        password: "test"
                    });

                    await db.collections.demo.create({ id: "test-id" });

                    const found = await db.collections.demo.findById("test-id");
                    expect(found).to.not.be.undefined;
                    expect(found?.id).to.eq("test-id");
                    expect(found?.age).to.eq(18);


                });
                it("Should be able to create and migrate a schema from v1 to v2", async () => {
                    const schema = {
                        version: 1,
                        primaryKey: 'id',
                        type: SchemaFieldType.object,
                        required: ['id', 'age'],
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
                    
                    const db = new RIDB(
                        {
                            dbName: "test" + uuidv4(),
                            schemas: {
                                demo: schema
                            } as const,
                            migrations: {
                                demo: {
                                    1: function (doc: Doc<typeof schema>) {
                                        return doc
                                    }
                                }
                            }
                        }
                    )

                    await db.start({
                        storageType: storage,
                        password: "test"
                    })
                    expect(db).to.not.be.undefined;

                    const created = await db.collections.demo.create({
                        id: "12345",
                        age: 18,
                        __version: 0
                    })

                    expect(created).to.not.be.undefined;
                    expect(created).to.haveOwnProperty("id");
                    expect(created).to.haveOwnProperty("age");
                    expect(created).to.haveOwnProperty("__version");

                    expect(created.__version).to.eq(1);
                })
            })
        })
    });
}