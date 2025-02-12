/**
 * @packageDocumentation
 *
 * <p align="center">
 *  <img src="https://cdn.jsdelivr.net/gh/trust0-project/ridb@latest/docs/logo.svg" alt="JavaScript Database" />
 *  <br />
 *  <br />
 *  <h3 align="center">A secure light-weight and dependency free database wrapper for the web.</h3>
 * </p>
 * <p align="center">
 *   <a href="https://github.com/trust0-project/RIDB/releases"><img src="https://img.shields.io/github/v/release/trust0-project/ridb?color=%23ff00a0&include_prereleases&label=version&sort=semver&style=flat-square"></a>
 *   <a href="#"><img src="https://img.shields.io/npm/types/rxdb?style=flat-square"></a>
 *   <a href="https://raw.githubusercontent.com/trust0-project/RIDB/refs/heads/main/LICENSE"><img src="https://img.shields.io/github/license/trust0-project/ridb?style=flat-square"></a>
 *   <a href="https://www.npmjs.com/package/@trust0/ridb"><img src="https://img.shields.io/npm/dm/@trust0/ridb?color=c63a3b&style=flat-square"></a>   
 * </p>
 * <h1>Introduction</h1>
 * 
 * ### Usage
 * ```typescript
 * const db = new RIDB(
 *     {
 *         dbName: "demo",
 *         schemas: {
 *             demo: {
 *                 version: 0,
 *                 primaryKey: 'id',
 *                 type: SchemaFieldType.object,
 *                 properties: {
 *                     id: {
 *                         type: SchemaFieldType.string,
 *                         maxLength: 60
 *                     }
 *                 }
 *             }
 *         } as const
 *     }
 * )
 * ```
 * 
 * ### Starting the database
 * ```typescript    
 * await db.start()
 * ```
 * 
 * ### Using with encryption plugin
 * You can also optionally specify storageType with a compatible storage of your choice and an optional password to enable encryption plugin
 * ```typescript
 * await db.start({
 *     password: "my-password"
 * })
 * ```
 * 
 * A compatible storage should be a class implementing [StorageInternal<SchemaType> ](../docs/namespaces/RIDBTypes/classes/StorageInternal.md) and its methods.
 * 
 * ### Using with migration plugin
 * The migration plugin will automatically migrate your documents for you as you upgrade and change your schemas over the time. 
 * 
 * ```typescript
 * const db = new RIDB(
 *     {
 *         dbName: "demo",
 *         schemas: {
 *             demo: {
 *                 version: 1,
 *                 primaryKey: 'id',
 *                 type: SchemaFieldType.object,
 *                 required:['id', 'age'],
 *                 properties: {
 *                     id: {
 *                         type: SchemaFieldType.string,
 *                         maxLength: 60
 *                     },
 *                     age: {
 *                         type: SchemaFieldType.number,
 *                     }
 *                 }
 *             }
 *         } as const,
 *         migrations: {
 *             demo: {
 *                 1: function (doc) {
 *                     return doc
 *                 }
 *             }
 *         }
 *     }
 * )
 * 
 * await db.start()
 * ```
 * # SDK Rerefence
 */

// @ts-ignore @ignore
import wasmBuffer from "@trust0/ridb-core/pkg/ridb_core_bg.wasm";
import { BasePlugin, BaseStorage, Database, MigrationPathsForSchemas, MigrationsParameter, SchemaTypeRecord } from "@trust0/ridb-core";

let internal: typeof import("@trust0/ridb-core") | undefined;

type StorageClass<T extends SchemaTypeRecord> = {
    create: (
        name: string, 
        schemas: T, 
        options: any
    ) => Promise<BaseStorage<T>>;
}

export enum StorageType {
    InMemory = "InMemory",
    IndexDB = "IndexDB"
}

export type StartOptions<T extends SchemaTypeRecord> = {
    storageType?: StorageClass<T> | StorageType,
    password?: string,
    [name: string]: any
}

export type {
    OpType, 
    IndexDB, 
    Operators, 
    InOperator, 
    OperatorOrType, 
    LogicalOperators, 
    QueryType, 
    Query, 
    InternalsRecord, 
    ExtractType, 
    Doc, 
    Collection, 
    InMemory, 
    Operation, 
    Property, 
    CoreStorage, 
    EnumerateUpTo, 
    EnumerateFrom1To, 
    IsVersionGreaterThan0, 
    AnyVersionGreaterThan1, 
    MigrationFunction, 
    MigrationPathsForSchema, 
    MigrationPathsForSchemas, 
    MigrationsParameter, 
    BaseStorageOptions, 
    BaseStorage, 
    SchemaType, 
    Schema, 
    Database, 
    CreateStorage, 
    RIDBModule, 
    Hook, 
    BasePluginOptions, 
    BasePlugin, 
    SchemaTypeRecord, 
    StorageInternal
} from "@trust0/ridb-core";


export class RIDB<T extends SchemaTypeRecord = SchemaTypeRecord> {

    private schemas: T;
    private migrations: MigrationPathsForSchemas<T>
    private plugins: Array<typeof BasePlugin> = [];
    private _db: Database<T> | undefined;
    private dbName: string;
    
    /**
     * Creates an instance of RIDB.
     * @param options
     */
    constructor(
        options: {
            dbName: string,
            schemas: T,
            plugins?: Array<typeof BasePlugin>
        } & MigrationsParameter<T>
    ) {
        const {
            dbName,
            schemas,
            migrations = {} as MigrationPathsForSchemas<T>,
            plugins = []
        } = options;

        this.schemas = schemas;
        this.plugins = plugins;
        this.migrations = migrations;
        this.dbName = dbName;
    }


    private getStorageType<T extends StorageType>(
        storageType: T
    ) {
        return storageType === StorageType.InMemory ? internal!.InMemory : internal!.IndexDB;
    }


    /**
     * Gets the database instance. Throws an error if the database has not been started.
     * @throws Will throw an error if the database is not started.
     * @private
     */
    private get db() {
        if (!this._db) {
            throw new Error("Start the database first");
        }
        return this._db;
    }

    get started() {
        return this._db?.started ?? false;
    }

    /**
     * Gets the collections from the database.
     * @returns The collections object.
     */
    get collections() {
        return this.db.collections;
    }

    /**
     * Loads the RIDB Rust module.
     * @returns {Promise<typeof import("@trust0/ridb")>} A promise that resolves to the RIDB Rust module.
     * @private
     */
    static async load() {
        internal ??= await import("@trust0/ridb-core").then(async (module) => {
            const wasmInstance = module.initSync(wasmBuffer);
            await module.default(wasmInstance);
            return module;
        });
        return internal!;
    }

    /**
     * Starts the database.
     * @returns {Promise<Database<T>>} A promise that resolves to the database instance.
     * @param options
     */
    async start(
        options?: StartOptions<T>
    ): Promise<Database<T>> {
        if (!this._db) {
            const { storageType, password } = options ?? {};
            const { Database } = await RIDB.load();
            const StorageClass = typeof storageType === "string" ?
                this.getStorageType(storageType) :
                storageType ?? undefined;
            
            if (StorageClass && !StorageClass.create) {
                throw new Error("Your storage does not have an async create function, please check documentation")
            }

            
            const storage = StorageClass ? 
                await StorageClass.create(this.dbName, this.schemas, options) :
                undefined;

            this._db ??= await Database.create<T>(
                this.dbName,
                this.schemas,
                this.migrations,
                this.plugins,
                {
                    apply: (plugins: Array<typeof BasePlugin> = []) => plugins.map((Plugin) => new Plugin()),
                },
                password,
                storage
            );

        } else {
            await this.db.start();
        }
        return this.db;
    }


    async close() {
        await this.db.close();
        this._db = undefined;
    }

}

/**
 * An enumeration of schema field types.
 */
export const SchemaFieldType = {
    string: 'string' as const,
    number: 'number' as const,
    boolean: 'boolean' as const,
    array: 'array' as const,
    object: 'object' as const,
};

