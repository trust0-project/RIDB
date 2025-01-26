import { describe, it, expect, afterEach, beforeEach, afterAll } from 'vitest';
import path from 'path';
import fs from 'fs';
import { v4 as uuidv4 } from 'uuid';
import { RIDB, SchemaFieldType, Doc } from '@trust0/ridb';
import { StoragesType, TestPlatform } from '..';


export default (platform: string, storages: StoragesType[]) => {
    return describe(`[${platform}] Testing`, () => {
        let dbName: string;

        beforeEach(() => {
            dbName = "test" + uuidv4();
        })

        afterAll(() => {
            if (platform === TestPlatform.NODE) {
                fs.rmSync(path.resolve(process.cwd(), `./.db`), { recursive: true, force: true });
            }
        })

        storages.forEach(({ name, storage }) => {

            describe(`[${platform}][${storage ? 'Typescript' : 'Wasm'} ${name}] Testing Storage`, () => {
  
                it('It should be able to create a new document from JSON schema', async () => {
                    const db = new RIDB(
                        {
                            dbName,
                            schemas: {
                                demo: {
                                    version: 0 as const,
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
                            dbName,
                            schemas: {
                                demo: {
                                    version: 0 as const,
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
                            dbName,
                            schemas: {
                                demo2: {
                                    version: 0 as const,
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
                                    version: 0 as const,
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
                            dbName,
                            schemas: {
                                demo: {
                                    version: 0 as const,
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
                            dbName,
                            schemas: {
                                demo: {
                                    version: 0 as const,
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
                            dbName,
                            schemas: {
                                demo: {
                                    version: 0 as const,
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
                            dbName,
                            schemas: {
                                demo: {
                                    version: 0 as const,
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
                            dbName,
                            schemas: {
                                demo: {
                                    version: 0 as const,
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
                            dbName,
                            schemas: {
                                demo: {
                                    version: 0 as const,
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
                            dbName,
                            schemas: {
                                demo: {
                                    version: 0 as const,
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
                            dbName,
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
                            dbName,
                            schemas: {
                                users: {
                                    version: 0 as const,
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
                                    version: 0 as const,
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
                            dbName,
                            schemas: {
                                demo: {
                                    version: 0 as const,
                                    primaryKey: 'id',
                                    type: SchemaFieldType.object,
                                    properties: {
                                        id: {
                                            type: SchemaFieldType.string,
                                            maxLength: 60
                                        },
                                        age: {
                                            type: SchemaFieldType.number,
                                            default: 18
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
                            dbName,
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
                        __version: 0 as const
                    })

                    expect(created).to.not.be.undefined;
                    expect(created).to.haveOwnProperty("id");
                    expect(created).to.haveOwnProperty("age");
                    expect(created).to.haveOwnProperty("__version");

                    expect(created.__version).to.eq(1);
                })
                it('Should handle array types in schema', async () => {
                    const db = new RIDB({
                        dbName,
                        schemas: {
                            demo: {
                                version: 0 as const,
                                primaryKey: 'id',
                                type: SchemaFieldType.object,
                                properties: {
                                    id: { type: SchemaFieldType.string },
                                    tags: {
                                        type: SchemaFieldType.array,
                                        items: { type: SchemaFieldType.string }
                                    }
                                }
                            }
                        } as const
                    });
                    await db.start({
                        storageType: storage,
                        password: "test"
                    });
                    const created = await db.collections.demo.create({
                        id: "12345",
                        tags: ["tag1", "tag2"]
                    });
                    expect(created).to.not.be.undefined;
                    expect(created.tags).to.deep.equal(["tag1", "tag2"]);
                });

                it('Should handle nested object properties', async () => {
                    const db = new RIDB({
                        dbName,
                        schemas: {
                            demo: {
                                version: 0 as const,
                                primaryKey: 'id',
                                type: SchemaFieldType.object,
                                properties: {
                                    id: { type: SchemaFieldType.string },
                                    profile: {
                                        type: SchemaFieldType.object,
                                        properties: {
                                            firstName: { type: SchemaFieldType.string },
                                            lastName: { type: SchemaFieldType.string }
                                        }
                                    }
                                }
                            }
                        } as const
                    });
                    await db.start({
                        storageType: storage,
                        password: "test"
                    });
                    const created = await db.collections.demo.create({
                        id: "12345",
                        profile: {
                            firstName: "John",
                            lastName: "Doe"
                        }
                    });
                    expect(created).to.not.be.undefined;
                    expect(created.profile).to.deep.equal({
                        firstName: "John",
                        lastName: "Doe"
                    });
                });

                it('Should throw error when required property is missing', async () => {
                    const db = new RIDB({
                        dbName,
                        schemas: {
                            demo: {
                                version: 0 as const,
                                primaryKey: 'id',
                                type: SchemaFieldType.object,
                                properties: {
                                    id: { type: SchemaFieldType.string },
                                    name: { type: SchemaFieldType.string }
                                }
                            }
                        } as const
                    });
                    await db.start({
                        storageType: storage,
                        password: "test"
                    });
                    await expect(db.collections.demo.create({
                        id: "missing_property_12345"
                    } as any)).rejects.toThrowError("Validation Error: Missing required property 'name'");
                });

                it('Should support boolean types in schema', async () => {
                    const db = new RIDB({
                        dbName,
                        schemas: {
                            demo: {
                                version: 0 as const,
                                primaryKey: 'id',
                                type: SchemaFieldType.object,
                                properties: {
                                    id: { type: SchemaFieldType.string },
                                    isActive: { type: SchemaFieldType.boolean }
                                }
                            }
                        } as const
                    });
                    await db.start({
                        storageType: storage,
                        password: "test"
                    });
                    const created = await db.collections.demo.create({
                        id: "12345",
                        isActive: true
                    });
                    expect(created).to.not.be.undefined;
                    expect(created.isActive).to.be.true;
                });

                it('Should apply default values when creating documents', async () => {
                    const db = new RIDB({
                        dbName,
                        schemas: {
                            demo: {
                                version: 0 as const,
                                primaryKey: 'id',
                                type: SchemaFieldType.object,
                                properties: {
                                    id: { type: SchemaFieldType.string },
                                    status: { type: SchemaFieldType.string, default: 'active' }
                                }
                            }
                        } as const
                    });
                    await db.start({
                        storageType: storage,
                        password: "test"
                    });
                    const created = await db.collections.demo.create({
                        id: "12345"
                    });
                    expect(created).to.not.be.undefined;
                    expect(created.status).to.equal('active');
                });

                it('Should handle updates without affecting unspecified fields', async () => {
                    const db = new RIDB({
                        dbName,
                        schemas: {
                            demo: {
                                version: 0 as const,
                                primaryKey: 'id',
                                type: SchemaFieldType.object,
                                properties: {
                                    id: { type: SchemaFieldType.string },
                                    name: { type: SchemaFieldType.string },
                                    age: { type: SchemaFieldType.number }
                                }
                            }
                        } as const
                    });
                    await db.start({
                        storageType: storage,
                        password: "test"
                    });
                    await db.collections.demo.create({
                        id: "12345",
                        name: "Alice",
                        age: 30
                    });
                    await db.collections.demo.update({
                        id: "12345",
                        name: "Bob"
                    });
                    const updated = await db.collections.demo.findById("12345");
                    expect(updated).to.not.be.undefined;
                    expect(updated?.name).to.equal("Bob");
                    expect(updated?.age).to.equal(30);
                });

                it('Should validate maxItems constraint on arrays', async () => {
                    const db = new RIDB({
                        dbName,
                        schemas: {
                            demo: {
                                version: 0 as const,
                                primaryKey: 'id',
                                type: SchemaFieldType.object,
                                properties: {
                                    id: { type: SchemaFieldType.string },
                                    tags: {
                                        type: SchemaFieldType.array,
                                        items: { type: SchemaFieldType.string, maxItems: 1 },
                                        maxItems: 2
                                    }
                                }
                            }
                        } as const
                    });
                    await db.start({
                        storageType: storage,
                        password: "test"
                    });
                    await expect(db.collections.demo.create({
                        id: "12345",
                        tags: ["tag1", "tag2", "tag3"]
                    })).rejects.toThrowError("Validation Error: Property 'tags' exceeds maximum items of '2'");
                });

                it('Should handle deletion of documents', async () => {
                    const db = new RIDB({
                        dbName,
                        schemas: {
                            demo: {
                                version: 0 as const,
                                primaryKey: 'id',
                                type: SchemaFieldType.object,
                                properties: {
                                    id: { type: SchemaFieldType.string },
                                    data: { type: SchemaFieldType.string }
                                }
                            }
                        } as const
                    });
                    await db.start({
                        storageType: storage,
                        password: "test"
                    });
                    await db.collections.demo.create({
                        id: "12345",
                        data: "Sample data"
                    });
                    await db.collections.demo.delete("12345");
                    const found = await db.collections.demo.findById("12345");
                    expect(found).to.be.undefined;
                });

                it('Should enforce maxLength on string properties', async () => {
                    const db = new RIDB({
                        dbName,
                        schemas: {
                            demo: {
                                version: 0 as const,
                                primaryKey: 'id',
                                type: SchemaFieldType.object,
                                properties: {
                                    id: {
                                        type: SchemaFieldType.string,
                                        maxLength: 5
                                    }
                                }
                            }
                        } as const
                    });
                    await db.start({
                        storageType: storage,
                        password: "test"
                    });
                    await expect(db.collections.demo.create({
                        id: "213123123123"
                    })).rejects.toThrowError("Validation Error: Property 'id' exceeds maximum length of '5'");
                });

                it('Should handle querying with complex conditions', async () => {
                    const db = new RIDB({
                        dbName,
                        schemas: {
                            users: {
                                version: 0 as const,
                                primaryKey: 'id',
                                type: SchemaFieldType.object,
                                properties: {
                                    id: { type: SchemaFieldType.string },
                                    age: { type: SchemaFieldType.number },
                                    isActive: { type: SchemaFieldType.boolean }
                                }
                            }
                        } as const
                    });
                    await db.start({
                        storageType: storage,
                        password: "test"
                    });
                    await db.collections.users.create({ id: 'u1', age: 25, isActive: true });
                    await db.collections.users.create({ id: 'u2', age: 30, isActive: false });
                    await db.collections.users.create({ id: 'u3', age: 35, isActive: true });

                    const results = await db.collections.users.find({
                        $and: [
                            { age: { $gte: 30 } },
                            { isActive: true }
                        ]
                    });
                    expect(results.length).to.equal(1);
                    expect(results[0].id).to.equal('u3');
                });

                it('Should create and verify index collections', async () => {
                    const usersSchema = {
                        version: 0 as const,
                        primaryKey: 'id',
                        type: SchemaFieldType.object,
                        indexes: ['age'],
                        properties: {
                            id: {
                                type: SchemaFieldType.string,
                                maxLength: 60
                            },
                            name: {
                                type: SchemaFieldType.string,
                                maxLength: 100
                            },
                            age: {
                                type: SchemaFieldType.number
                            }
                        }
                    }
               
                    const db = new RIDB({
                        dbName,
                        schemas: {
                            users: usersSchema
                        }
                    });
                    await db.start({
                        storageType: storage,
                        password: "test"
                    });

                    // Verify main collection exists
                    expect(db.collections).to.haveOwnProperty("users");

                    // Create a user and verify indexes are maintained
                    await db.collections.users.create({
                        id: "user1",
                        name: "John Doe",
                        age: 30
                    });

                    await db.collections.users.create({
                        id: "user2",
                        name: "Doe John",
                        age: 35
                    });

                    const usersAge30 = await db.collections.users.find({
                        age: 30
                    });

                    expect(usersAge30.length).to.eq(1);

                    const usersAgeOlderThan20 = await db.collections.users.find({
                        age: { $gt: 20 }
                    });

                    expect(usersAgeOlderThan20.length).to.eq(2);
                });

                it('Should work correctly without indexes', async () => {
                    const db = new RIDB({
                        dbName,
                        schemas: {
                            users: {
                                version: 0 as const,
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

                    // Verify main collection exists
                    expect(db.collections).to.haveOwnProperty("users");

                    // Create a user
                    const user = await db.collections.users.create({
                        id: "user1",
                        name: "John Doe"
                    });

                    expect(user).to.not.be.undefined;
                    expect(user.id).to.eq("user1");
                });

                it('Should maintain index integrity during CRUD operations', async () => {
                    const db = new RIDB({
                        dbName,
                        schemas: {
                            users: {
                                version: 0 as const,
                                primaryKey: 'id',
                                type: SchemaFieldType.object,
                                indexes: ['name'],
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

                    // Create
                    const user = await db.collections.users.create({
                        id: "user1",
                        name: "John Doe"
                    });
                    expect(user).to.not.be.undefined;

                    // Update
                    await db.collections.users.update({
                        id: "user1",
                        name: "Jane Doe"
                    });

                    // Find by index
                    const found = await db.collections.users.find({
                        name: "Jane Doe"
                    });
                    expect(found.length).to.eq(1);
                    expect(found[0].id).to.eq("user1");

                    // Delete
                    await db.collections.users.delete("user1");
                    const notFound = await db.collections.users.find({
                        name: "Jane Doe"
                    });
                    expect(notFound.length).to.eq(0);
                });

                it('Should correctly count documents using advanced indexing', async () => {
                    // Define the schema with indexes
                    const db = new RIDB({
                        dbName,
                        schemas: {
                            users: {
                                version: 0 as const,
                                primaryKey: 'id',
                                type: SchemaFieldType.object,
                                indexes: ['age', 'name'],
                                properties: {
                                    id: { type: SchemaFieldType.string },
                                    age: { type: SchemaFieldType.number },
                                    name: { type: SchemaFieldType.string }
                                }
                            }
                        } as const
                    });

                    await db.start({
                        storageType: storage,
                        password: "test"
                    });

                    const usersCollection = db.collections.users;
                    // Insert multiple users with different ages and names
                    await usersCollection.create({ id: 'u1', age: 25, name: 'Alice' });
                    await usersCollection.create({ id: 'u2', age: 30, name: 'Bob' });
                    await usersCollection.create({ id: 'u3', age: 35, name: 'Charlie' });
                    await usersCollection.create({ id: 'u4', age: 30, name: 'David' });
                    await usersCollection.create({ id: 'u5', age: 25, name: 'Eve' });
                    const countAndAge25 = await usersCollection.find({
                        $and: [
                            { age: { $gte: 0 } },
                            { age: { $lte: 25 } }
                        ]
                    });
                    expect(countAndAge25.length).to.eq(2);

                    // Use count method with advanced queries utilizing indexes
                    const countAge25 = await usersCollection.find({ age: 25 });
                    expect(countAge25.length).to.eq(2);

                    const countAge30 = await usersCollection.count({ age: 30 });
                    expect(countAge30).to.eq(2);

                    const countNameBob = await usersCollection.count({ name: 'Bob' });
                    expect(countNameBob).to.eq(1);

                    const countAgeGreaterThan25 = await usersCollection.count({ age: { $gt: 25 } });
                    expect(countAgeGreaterThan25).to.eq(3);

                    const countAge25OrNameEve = await usersCollection.count({
                        $or: [
                            { age: 25 },
                            { name: 'Eve' }
                        ]
                    });
                    expect(countAge25OrNameEve).to.eq(2);

                    const countComplexQuery = await usersCollection.count({
                        $and: [
                            { age: { $gte: 30 } },
                            { name: 'David'  }
                        ]
                    });
                    expect(countComplexQuery).to.eq(1); // Only 'u3' matches

                    // Clean up
                    await db.close();
                });
            });


            describe(`[${platform}][${storage ? 'Typescript' : 'Wasm'} ${name}] Performance & Stress Tests`, () => {

                // Step 1: Measure bulk insertion performance
                it('should measure bulk insertion performance', async () => {
                    // Setup DB instance
                    const db = new RIDB({
                        dbName,
                        schemas: {
                            demo: {
                                version: 0 as const,
                                primaryKey: 'id',
                                type: SchemaFieldType.object,
                                properties: {
                                    id: { type: SchemaFieldType.string },
                                    data: { type: SchemaFieldType.string }
                                }
                            }
                        } as const
                    });

                    await db.start({
                        storageType: storage,
                        password: "test"
                    });

                    const collection = db.collections.demo;
                    const testCount = 1000;

                    // Measure creation time
                    const startTime = performance.now();
                    for (let i = 0; i < testCount; i++) {
                        await collection.create({ id: `doc_${i}`, data: `Some data #${i}` });
                    }
                    const endTime = performance.now();

                    const durationMs = endTime - startTime;
                    const opsPerSecond = (testCount / (durationMs / 1000)).toFixed(2);

                    console.log(
                        `[${platform}][${storage ? 'Typescript' : 'Wasm'} ${name}] Inserted ${testCount} documents in ${durationMs.toFixed(2)} ms ` +
                        `(${opsPerSecond} ops/sec)`
                    );

                    const countInDb = await collection.count({});
                    expect(countInDb).toEqual(testCount);
                });

                // Step 2: Measure query performance on a large dataset
                it('should measure query performance on large dataset', async () => {
                    const db = new RIDB({
                        dbName,
                        schemas: {
                            demo: {
                                version: 0 as const,
                                primaryKey: 'id',
                                type: SchemaFieldType.object,
                                indexes: ['age'],
                                properties: {
                                    id: { type: SchemaFieldType.string },
                                    age: { type: SchemaFieldType.number }
                                }
                            }
                        } as const
                    });

                    await db.start({
                        storageType: storage,
                        password: "test"
                    });

                    const collection = db.collections.demo;
                    const testCount = 5000;
                    const possibleAges = [10, 20, 30, 40, 50, 60, 70, 80, 90];

                    // Bulk insert
                    for (let i = 0; i < testCount; i++) {
                        // Cycle through the age array
                        let age = possibleAges[i % possibleAges.length];
                
                        await collection.create({
                            id: `doc_${i}`,
                            age: age
                        });
                    }

                    // Measure query performance
                    const startTime = performance.now();
                    const results = await collection.find( { age: 30 });
                    const endTime = performance.now();

                    const queryTimeMs = (endTime - startTime).toFixed(2);
                    console.log(`[${platform}][${storage ? 'Typescript' : 'Wasm'} ${name}] Query completed in ${queryTimeMs} ms. Found ${results.length} docs.`);

                    expect(results).toBeDefined();
                });

                // Step 3: Ensure accurate deletion of documents
                it('should handle deletion of documents', async () => {
                    const db = new RIDB({
                        dbName,
                        schemas: {
                            demo: {
                                version: 0 as const,
                                primaryKey: 'id',
                                type: SchemaFieldType.object,
                                properties: {
                                    id: { type: SchemaFieldType.string },
                                    data: { type: SchemaFieldType.string }
                                }
                            }
                        } as const
                    });

                    await db.start({
                        storageType: storage,
                        password: "test"
                    });

                    const collection = db.collections.demo;
                    await collection.create({ id: "12345", data: "Sample data" });

                    await collection.delete("12345");
                    const found = await collection.findById("12345");

                    expect(found).toBeUndefined();
                });

                // Step 4: Measure performance queries on multiple indexes with large dataset
                it('should measure advanced query performance with multiple indexes on a large dataset', async () => {
                    // Define the DB with multiple indexes
                    const db = new RIDB({
                        dbName,
                        schemas: {
                            perfTest: {
                                version: 0 as const,
                                primaryKey: 'id',
                                type: SchemaFieldType.object,
                                // Index on multiple fields to test query complexity
                                indexes: ['category', 'score', 'group'],
                                properties: {
                                    id: { type: SchemaFieldType.string },
                                    category: { type: SchemaFieldType.string, maxLength: 20 },
                                    score: { type: SchemaFieldType.number },
                                    group: { type: SchemaFieldType.string },
                                }
                            }
                        } as const
                    });

                    await db.start({
                        storageType: storage,
                        password: "test"
                    });

                    const collection = db.collections.perfTest;

                    // Insert a large volume of documents
                    const totalDocs = 10000;
                    const categories = ['catA', 'catB', 'catC', 'catD'];
                    const groups = ['group1', 'group2', 'group3'];

                    for (let i = 0; i < totalDocs; i++) {
                        await collection.create({
                            id: `doc_${i}`,
                            category: categories[i % categories.length],
                            score: Math.floor(Math.random() * 1000),
                            group: groups[i % groups.length]
                        });
                    }

                    // Perform advanced queries using multiple indexed fields
                    const queryStart = performance.now();
                    const results = await collection.find({
                        $and: [
                            { category: 'catB' },
                            { score: { $gte: 500 } },
                            {
                                $or: [
                                    { group: 'group2' },
                                    { group: 'group3' }
                                ]
                            }
                        ]
                    });
                    const queryEnd = performance.now();

                    console.log(
                        `[${platform}][${storage ? 'Typescript' : 'Wasm'} ${name}] ` +
                        `Advanced query with multiple indexes completed in ${(queryEnd - queryStart).toFixed(2)} ms. Found ${results.length} docs.`
                    );

                    // Basic checks
                    expect(results).toBeDefined();
                    for (const doc of results) {
                        expect(doc.category).toBe('catB');
                        expect(doc.score).toBeGreaterThanOrEqual(500);
                        expect(['group2', 'group3']).toContain(doc.group);
                    }

                    await db.close();
                });

                describe("Index usage with small data sets", () => {

                    const docs = [
                        { id: "doc1", age: 18, status: "active" },
                        { id: "doc2", age: 25, status: "inactive" },
                        { id: "doc3", age: 30, status: "active" },
                        { id: "doc4", age: 22, status: "pending" },
                        { id: "doc5", age: 40, status: "active" },
                        { id: "doc6", age: 35, status: "inactive" },
                        { id: "doc7", age: 25, status: "pending" },
                        { id: "doc8", age: 18, status: "pending" },
                        { id: "doc9", age: 45, status: "active" },
                        { id: "doc10", age: 28, status: "inactive" },
                    ];

                    /**
                     * 1) $gt condition on an indexed field
                     */
                    it('should retrieve documents with age > 30', async () => {
                        const db = new RIDB({
                            dbName,
                            schemas: {
                                smallIndexTest: {
                                    version: 0 as const,
                                    primaryKey: "id",
                                    type: SchemaFieldType.object,
                                    indexes: ["age", "status"],
                                    properties: {
                                        id: { type: SchemaFieldType.string },
                                        age: { type: SchemaFieldType.number },
                                        status: { type: SchemaFieldType.string },
                                    },
                                },
                            },
                        });
                        await db.start({
                            storageType: storage,
                            password: "test",
                        });

                        // Insert test documents
                        for (const doc of docs) {
                            await db.collections.smallIndexTest.create(doc);
                        }

                        // Query
                        const found = await db.collections.smallIndexTest.find({
                            age: { $gt: 30 },
                        });

                        expect(found.length).to.eq(3); // doc5, doc6, doc9
                        const ids = found.map((doc) => doc.id);
                        expect(ids).to.include.members(["doc5", "doc6", "doc9"]);

                        await db.close();
                    });

                    /**
                     * 2) $gte combined with $and
                     */
                    it('should retrieve documents with age >= 25 AND status = "active"', async () => {
                        const db = new RIDB({
                            dbName,
                            schemas: {
                                smallIndexTest: {
                                    version: 0 as const,
                                    primaryKey: "id",
                                    type: SchemaFieldType.object,
                                    indexes: ["age", "status"],
                                    properties: {
                                        id: { type: SchemaFieldType.string },
                                        age: { type: SchemaFieldType.number },
                                        status: { type: SchemaFieldType.string },
                                    },
                                },
                            },
                        });
                        await db.start({ storageType: storage, password: "test" });

                        for (const doc of docs) {
                            await db.collections.smallIndexTest.create(doc);
                        }

                        const found = await db.collections.smallIndexTest.find({
                            $and: [
                                { age: { $gte: 25 } },
                                { status: "active" }
                            ],
                        });

                        // Should match doc3(30, active), doc5(40, active), doc9(45, active)
                        expect(found.length).to.eq(3);
                        const ids = found.map((doc) => doc.id);
                        expect(ids).to.include.members(["doc3", "doc5", "doc9"]);

                        await db.close();
                    });

                    /**
                     * 3) $lt condition
                     */
                    it('should retrieve documents with age < 20', async () => {
                        const db = new RIDB({
                            dbName,
                            schemas: {
                                smallIndexTest: {
                                    version: 0 as const,
                                    primaryKey: "id",
                                    type: SchemaFieldType.object,
                                    indexes: ["age", "status"],
                                    properties: {
                                        id: { type: SchemaFieldType.string },
                                        age: { type: SchemaFieldType.number },
                                        status: { type: SchemaFieldType.string },
                                    },
                                },
                            },
                        });
                        await db.start({ storageType: storage, password: "test" });

                        for (const doc of docs) {
                            await db.collections.smallIndexTest.create(doc);
                        }

                        const found = await db.collections.smallIndexTest.find({
                            age: { $lt: 20 },
                        });

                        // Should match doc1(18, active) and doc8(18, pending)
                        expect(found.length).to.eq(2);
                        const ids = found.map((doc) => doc.id);
                        expect(ids).to.include.members(["doc1", "doc8"]);

                        await db.close();
                    });

                    /**
                     * 4) $lte combined with $or
                     */
                    it('should retrieve documents with (age <= 25) OR (status = "pending")', async () => {
                        const db = new RIDB({
                            dbName,
                            schemas: {
                                smallIndexTest: {
                                    version: 0 as const,
                                    primaryKey: "id",
                                    type: SchemaFieldType.object,
                                    indexes: ["age", "status"],
                                    properties: {
                                        id: { type: SchemaFieldType.string },
                                        age: { type: SchemaFieldType.number },
                                        status: { type: SchemaFieldType.string },
                                    },
                                },
                            },
                        });
                        await db.start({ storageType: storage, password: "test" });

                        for (const doc of docs) {
                            await db.collections.smallIndexTest.create(doc);
                        }

                        const found = await db.collections.smallIndexTest.find({
                            $or: [
                                { age: { $lte: 25 } },
                                { status: "pending" },
                            ],
                        });

                        // doc1(18,active), doc2(25,inactive),
                        // doc4(22,pending), doc7(25,pending),
                        // doc8(18,pending)
                        expect(found.length).to.eq(5);
                        const ids = found.map((doc) => doc.id);
                        expect(ids).to.include.members(["doc1", "doc2", "doc4", "doc7", "doc8"]);

                        await db.close();
                    });

                    /**
                     * 5) $in condition
                     */
                    it('should retrieve documents with age in [18, 25, 28]', async () => {
                        const db = new RIDB({
                            dbName,
                            schemas: {
                                smallIndexTest: {
                                    version: 0 as const,
                                    primaryKey: "id",
                                    type: SchemaFieldType.object,
                                    indexes: ["age", "status"],
                                    properties: {
                                        id: { type: SchemaFieldType.string },
                                        age: { type: SchemaFieldType.number },
                                        status: { type: SchemaFieldType.string },
                                    },
                                },
                            },
                        });
                        await db.start({ storageType: storage, password: "test" });

                        for (const doc of docs) {
                            await db.collections.smallIndexTest.create(doc);
                        }

                        const found = await db.collections.smallIndexTest.find({
                            age: { $in: [18, 25, 28] },
                        });

                        // doc1(18,active), doc2(25,inactive),
                        // doc7(25,pending), doc8(18,pending),
                        // doc10(28,inactive)
                        expect(found.length).to.eq(5);
                        const ids = found.map((doc) => doc.id);
                        expect(ids).to.include.members(["doc1", "doc2", "doc7", "doc8", "doc10"]);

                        await db.close();
                    });

                    /**
                     * 8) $and condition with multiple fields
                     */
                    it('should retrieve documents (age >= 25) AND (status in ["active","pending"])', async () => {
                        const db = new RIDB({
                            dbName,
                            schemas: {
                                smallIndexTest: {
                                    version: 0 as const,
                                    primaryKey: "id",
                                    type: SchemaFieldType.object,
                                    indexes: ["age", "status"],
                                    properties: {
                                        id: { type: SchemaFieldType.string },
                                        age: { type: SchemaFieldType.number },
                                        status: { type: SchemaFieldType.string },
                                    },
                                },
                            },
                        });
                        await db.start({ storageType: storage, password: "test" });

                        for (const doc of docs) {
                            await db.collections.smallIndexTest.create(doc);
                        }

                        const found = await db.collections.smallIndexTest.find({
                            $and: [
                                { age: { $gte: 25 } },
                                { status: { $in: ["active", "pending"] } },
                            ],
                        });

                        // Matches doc3(30,active), doc5(40,active),
                        // doc7(25,pending), doc9(45,active)
                        expect(found.length).to.eq(4);
                        const ids = found.map((doc) => doc.id);
                        expect(ids).to.include.members(["doc3", "doc5", "doc7", "doc9"]);

                        await db.close();
                    });

                    /**
                     * 9) $or condition with multiple fields
                     */
                    it('should retrieve documents where (age < 20) OR (status = "pending")', async () => {
                        const db = new RIDB({
                            dbName,
                            schemas: {
                                smallIndexTest: {
                                    version: 0 as const,
                                    primaryKey: "id",
                                    type: SchemaFieldType.object,
                                    indexes: ["age", "status"],
                                    properties: {
                                        id: { type: SchemaFieldType.string },
                                        age: { type: SchemaFieldType.number },
                                        status: { type: SchemaFieldType.string },
                                    },
                                },
                            },
                        });
                        await db.start({ storageType: storage, password: "test" });

                        for (const doc of docs) {
                            await db.collections.smallIndexTest.create(doc);
                        }

                        const found = await db.collections.smallIndexTest.find({
                            $or: [
                                { age: { $lt: 20 } },
                                { status: "pending" },
                            ],
                        });

                        // doc1(18,active) => yes
                        // doc4(22,pending) => yes
                        // doc7(25,pending) => yes
                        // doc8(18,pending) => yes
                        expect(found.length).to.eq(4);
                        const ids = found.map((doc) => doc.id);
                        expect(ids).to.include.members(["doc1", "doc4", "doc7", "doc8"]);

                        await db.close();
                    });

                    /**
                     * 10) Complex nested condition
                     */
                    it('should retrieve documents with status in ["active","pending"] AND ( age >= 30 OR age < 20 )', async () => {
                        const db = new RIDB({
                            dbName,
                            schemas: {
                                smallIndexTest: {
                                    version: 0 as const,
                                    primaryKey: "id",
                                    type: SchemaFieldType.object,
                                    indexes: ["age", "status"],
                                    properties: {
                                        id: { type: SchemaFieldType.string },
                                        age: { type: SchemaFieldType.number },
                                        status: { type: SchemaFieldType.string },
                                    },
                                },
                            },
                        });
                        await db.start({ storageType: storage, password: "test" });

                        // Insert test documents
                        for (const doc of docs) {
                            await db.collections.smallIndexTest.create(doc);
                        }

                        const found = await db.collections.smallIndexTest.find({
                            $and: [
                                { status: { $in: ["active", "pending"] } },
                                {
                                    $or: [
                                        { age: { $gte: 30 } },
                                        { age: { $lt: 20 } },
                                    ],
                                },
                            ],
                        });
                        expect(found.length).to.eq(5);
                        const ids = found.map((doc) => doc.id);
                        expect(ids).to.include.members(["doc1", "doc3", "doc8", "doc9"]);

                        await db.close();
                    });

                    /**
                     * 11) $nin operator on a numeric field
                     */
                    it('should retrieve documents where age not in [25, 30]', async () => {
                        const db = new RIDB({
                            dbName,
                            schemas: {
                                smallIndexTest: {
                                    version: 0 as const,
                                    primaryKey: "id",
                                    type: SchemaFieldType.object,
                                    indexes: ["age", "status"],
                                    properties: {
                                        id: { type: SchemaFieldType.string },
                                        age: { type: SchemaFieldType.number },
                                        status: { type: SchemaFieldType.string },
                                    },
                                },
                            },
                        });
                        await db.start({ storageType: storage, password: "test" });

                        // Insert test documents
                        for (const doc of docs) {
                            await db.collections.smallIndexTest.create(doc);
                        }

                        // Query for documents where age is not in the array [25, 30]
                        const found = await db.collections.smallIndexTest.find({
                            age: { $nin: [25, 30] },
                        });

                        // docs:
                        // * doc2 (25)   -> excluded
                        // * doc3 (30)   -> excluded
                        // * doc7 (25)   -> excluded
                        // Others have ages: 18, 22, 28, 35, 40, 45
                        // We should end up with 7 matches
                        expect(found.length).to.eq(7);

                        // Ensure none of them contain the excluded ages
                        for (const item of found) {
                            expect([25, 30]).to.not.include(item.age);
                        }

                        await db.close();
                    });

                    /**
                     * 12) $nin operator on a string field
                     */
                    it('should retrieve documents where status not in ["active", "pending"]', async () => {
                        const db = new RIDB({
                            dbName,
                            schemas: {
                                smallIndexTest: {
                                    version: 0 as const,
                                    primaryKey: "id",
                                    type: SchemaFieldType.object,
                                    indexes: ["age", "status"],
                                    properties: {
                                        id: { type: SchemaFieldType.string },
                                        age: { type: SchemaFieldType.number },
                                        status: { type: SchemaFieldType.string },
                                    },
                                },
                            },
                        });
                        await db.start({ storageType: storage, password: "test" });

                        // Insert test documents
                        for (const doc of docs) {
                            await db.collections.smallIndexTest.create(doc);
                        }

                        // Query for documents where status is neither "active" nor "pending"
                        const found = await db.collections.smallIndexTest.find({
                            status: { $nin: ["active", "pending"] },
                        });

                        // By checking our sample data:
                        // * "inactive" docs should appear: doc2, doc6, doc10
                        // * "active" or "pending" docs should be excluded
                        expect(found.length).to.eq(3);
                        const ids = found.map((doc) => doc.id);
                        expect(ids).to.include.members(["doc2", "doc6", "doc10"]);

                        // Also ensure none of them have a status of active or pending
                        for (const item of found) {
                            expect(["active", "pending"]).to.not.include(item.status);
                        }

                        await db.close();
                    });

                    /**
                     * 13) $eq operator on a numeric field
                     */
                    it('should retrieve documents where age = 25', async () => {
                        const db = new RIDB({
                            dbName,
                            schemas: {
                                smallIndexTest: {
                                    version: 0 as const,
                                    primaryKey: "id",
                                    type: SchemaFieldType.object,
                                    indexes: ["age", "status"],
                                    properties: {
                                        id: { type: SchemaFieldType.string },
                                        age: { type: SchemaFieldType.number },
                                        status: { type: SchemaFieldType.string },
                                    },
                                },
                            },
                        });
                        await db.start({ storageType: storage, password: "test" });

                        // Insert test documents
                        for (const doc of docs) {
                            await db.collections.smallIndexTest.create(doc);
                        }

                        // Query documents where age = 25
                        const found = await db.collections.smallIndexTest.find({
                            age: { $eq: 25 },
                        });

                        // In our docs array, doc2(age=25, status="inactive") and doc7(age=25, status="pending")
                        // are the only docs with age=25.
                        expect(found.length).to.eq(2);
                        const ids = found.map((doc) => doc.id);
                        expect(ids).to.include.members(["doc2", "doc7"]);

                        await db.close();
                    });

                    /**
                     * 14) $ne operator on a numeric field
                     */
                    it('should retrieve documents where age != 25', async () => {
                        const db = new RIDB({
                            dbName,
                            schemas: {
                                smallIndexTest: {
                                    version: 0 as const,
                                    primaryKey: "id",
                                    type: SchemaFieldType.object,
                                    indexes: ["age", "status"],
                                    properties: {
                                        id: { type: SchemaFieldType.string },
                                        age: { type: SchemaFieldType.number },
                                        status: { type: SchemaFieldType.string },
                                    },
                                },
                            },
                        });
                        await db.start({ storageType: storage, password: "test" });

                        // Insert test documents
                        for (const doc of docs) {
                            await db.collections.smallIndexTest.create(doc);
                        }

                        // Query documents where age != 25
                        const found = await db.collections.smallIndexTest.find({
                            age: { $ne: 25 },
                        });

                        // Excludes doc2 and doc7 (both have age=25).
                        expect(found.length).to.eq(docs.length - 2);
                        const excludedIds = ["doc2", "doc7"];
                        for (const record of found) {
                            expect(excludedIds).to.not.include(record.id);
                            expect(record.age).to.not.eq(25);
                        }

                        await db.close();
                    });

                });
            });
        });
    });
}