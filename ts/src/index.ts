/**
 * @packageDocumentation
 *
 * <p align="center">
 *   <img src="https://cdn.jsdelivr.net/gh/trust0-project/ridb@0.4.3/docs/logo.svg" alt="JavaScript Database" />
 *   <br />
 *   <br />
 *   <h3 align="center">A secure light-weight and dependency free database wrapper for the web.</h3>
 * </p>
 *
 *
 * <p align="center">
 *     <a href="https://github.com/trust0-project/RIDB/releases"><img src="https://img.shields.io/github/v/release/trust0-project/ridb?color=%23ff00a0&include_prereleases&label=version&sort=semver&style=flat-square"></a>
 *     &nbsp;
 *     <a href="#"><img src="https://img.shields.io/npm/types/rxdb?style=flat-square"></a>
 *     &nbsp;
 *     <a href="https://raw.githubusercontent.com/trust0-project/RIDB/refs/heads/main/LICENSE"><img src="https://img.shields.io/github/license/trust0-project/ridb?style=flat-square"></a>
 *     &nbsp;
 *     <a href="https://www.npmjs.com/package/@trust0/ridb"><img src="https://img.shields.io/npm/dm/@trust0/ridb?color=c63a3b&style=flat-square"></a>
 * </p>
 * 
 * # SDK Rerefence
 */
import { BaseStorage, Doc, MigrationPathsForSchema, Operation, OpType, SchemaType, SchemaTypeRecord, StorageInternal } from "ridb-rust";
import wasmBuffer from "../../pkg/ridb_rust_bg.wasm";
import * as RIDBTypes from "ridb-rust";
export {
    // Enums
    OpType,
    // Classes
    Database,
    Collection,
    Schema,
    Query,
    Property,
    BasePlugin,
    StorageInternal,
    BaseStorage,
    // Types
    CreateStorage,
    RIDBModule,
    InternalsRecord,
    ExtractType,
    Doc,
    Operation,
    Hook,
    BasePluginOptions,
    SchemaType,
    EnumerateUpTo,
    EnumerateFrom1To,
    IsVersionGreaterThan0,
    AnyVersionGreaterThan1,
    MigrationFunction,
    MigrationPathsForSchema,
    MigrationPathsForSchemas,
    MigrationsParameter,
    Operators,
    InOperator,
    OperatorOrType,
    LogicalOperators,
    QueryType,
    SchemaTypeRecord
} from "ridb-rust";

export enum StorageType {
    InMemory = "InMemory",
    IndexDB = "IndexDB"
}

let internal: typeof import("ridb-rust") | undefined;

/**
 * Represents a RIDB (Rust IndexedDB) instance.
 * This is the main class exposed by the RIDB Storage sdk and is used to create a database instance.
 * 
 * ### Usage:
 * 
 * ```typescript
 * const db = new RIDB(
 *     {
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
 * A compatible storage should be a class implementing [StorageInternal<RIDBTypes.SchemaType> ](../namespaces/RIDBTypes/classes/StorageInternal.md) and its methods.
 * 
 * ### Using with migration plugin
 * The migration plugin will automatically migrate your documents for you as you upgrade and change your schemas over the time. 
 * 
 * ```typescript
 * const db = new RIDB(
 *     {
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
 * await db.start({storageType: storage})
 * ```
 *
 * @class
 * @template T - The type of the schema record.
 */
export class RIDB<T extends RIDBTypes.SchemaTypeRecord = RIDBTypes.SchemaTypeRecord> {

    private schemas: T;
    private migrations: RIDBTypes.MigrationPathsForSchemas<T>
    private plugins: Array<typeof RIDBTypes.BasePlugin> = [];
    private _db: RIDBTypes.Database<T> | undefined;
    private dbName: string;
    /**
     * Creates an instance of RIDB.
     * @param options
     */
    constructor(
        options: {
            dbName: string,
            schemas: T,
            plugins?: Array<typeof RIDBTypes.BasePlugin> 
        } & RIDBTypes.MigrationsParameter<T>
    ) {
        const {
            dbName,
            schemas,
            migrations = {} as RIDBTypes.MigrationPathsForSchemas<T>,
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

    /**
     * Gets the collections from the database.
     * @returns The collections object.
     */
    get collections() {
        return this.db.collections;
    }

    /**
     * Loads the RIDB Rust module.
     * @returns {Promise<typeof import("ridb-rust")>} A promise that resolves to the RIDB Rust module.
     * @private
     */
    static async load(): Promise<typeof import("ridb-rust")> {
        internal ??= await import("ridb-rust").then(async (module) => {
            const wasmInstance = module.initSync(wasmBuffer);
             await module.default(wasmInstance);
            return module;
        });
        return internal!;
    }

    /**
     * Gets the RIDB module with the provided storage type.
     * @param {typeof RIDBTypes.BaseStorage<RIDBTypes.SchemaType>} [storageType] - The storage type to use.
     * @returns An object containing createStorage and apply functions.
     * @private
     */
    private getRIDBModule(
        storageType?: typeof RIDBTypes.BaseStorage<T> | StorageType
    ) {
        return {
            createStorage: async (schemas: T) => this.createStorage(schemas, storageType),
            apply: (plugins: Array<typeof RIDBTypes.BasePlugin> = []) => plugins.map((Plugin) => new Plugin()),
        }  as RIDBTypes.RIDBModule;
    }

    /**
     * Starts the database.
     * @returns {Promise<RIDBTypes.Database<T>>} A promise that resolves to the database instance.
     * @param options
     */
    async start(
        options?: {
            storageType?: typeof RIDBTypes.BaseStorage<T>| StorageType,
            password?: string,
            [name:string]: any
        }
    ): Promise<RIDBTypes.Database<T>> {
        if (!this._db) {
            const {storageType, password} = options ?? {};
            const { Database } = await RIDB.load();
            this._db ??= await Database.create<T>(
                this.schemas,
                this.migrations,
                this.plugins,
                this.getRIDBModule(storageType),
                password
            );
        } else {
            await this.db.start();
        }
       
        return this.db;
    }


    async close() {
        await this.db.close();
    }

    /**
     * Creates storage instances for the provided schemas.
     * @template J - The type of the schema record.
     * @param {J} schemas - The schema definitions.
     * @param {typeof RIDBTypes.BaseStorage<RIDBTypes.SchemaType>} [storageConstructor] - The storage constructor to use.
     * @returns An object mapping collection names to storage instances.
     * @private
     */
    private async createStorage(
        schemas: T,
        storageConstructor?: typeof RIDBTypes.BaseStorage<T> | StorageType
    ) {
        if (!internal) {
            throw new Error("Start the database first");
        }

        const Storage = typeof storageConstructor === "string" ? 
             this.getStorageType(storageConstructor) : 
            storageConstructor ?? internal.InMemory;

        return Storage.create(
            this.dbName,
            schemas,
            this.migrations
        );
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
