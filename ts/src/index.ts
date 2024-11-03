/**
 * @packageDocumentation
 *
 * # Documentation
 * ## Package Description
 * RIDB secure database wrapper for the web with multiple storage engines, written in rust.
 * The project started after years of experience working with web projects in both browser and nodejs platforms, the project was born with some rules / objectives:
 * 1. Strong types + proper validation
 * 2. Declarative schemas & documents
 * 3. Configurable storages, inMemory, monogoDB, sqlite, indexdb
 * 4. Secure encryption
 * 5. Work seamlessly in browsers or nodejs applications.
 *
 * ## Supported features for InMemory Storage
 * The inMemory storage is used by default and is currently supporting the following features:
 * 1. Schemas: Creation of declararive schemas with required fields
 * 2. Schemas: Implement validation across all the flows extracting properties and required fields when needed
 * 3. Schemas: Manage Primary keys
 * 4. Internal Storage: write operation, create, update, fetch one, remove, find and count
 * 5. Internal Storage: Rust inMemory implementation
 * 6. Database default InMemory plugged in
 * 7. Plugin engine
 * 8. Encryption
 *
 * ## Install
 * In order to install simply run the following command
 * npm: 
 * ``` 
 * npm i @elribonazo/ridb --save
 * ```
 * 
 * yarn:
 * 
 * ``` 
 * yarn add @elribonazo/ridb
 * ```
 * 
 * ## Usage
 * Creating your own database is pretty straight forward.
 *
 * ```javascript
 * import {
 *     RIDB,
 *     SchemaFieldType
 * } from '@elribonazo/ridb';
 * 
 * (async () => {
 *     const db =  new RIDB({
 *         demo: {
 *             version: 0,
 *             primaryKey: 'id',
 *             type: SchemaFieldType.object,
 *             properties: {
 *                 id: {
 *                     type: SchemaFieldType.string,
 *                     maxLength: 60
 *                 }
 *             }
 *         }
 *     });
 *     console.log("Starting the database");
 *     await db.start();
 *     console.log("Ok :)");
 * })()
 * ```
 *
 * ## Specification
 *
 * ### Storage
 * A valid storage must extend [BaseStorage class](https://github.com/atala-community-projects/RIDB/blob/main/namespaces/RIDBTypes/classes/BaseStorage.md)
 * here's some example:
 *
 * ```typescript
 * export class InMemory<T extends SchemaType> extends BaseStorage<T>   {
 *     async write(operation:Operation<T>): Promise<Doc<T>> {
 *         if (operation.opType === OpType.CREATE) {
 *             return operation.data;
 *         }
 *         throw new Error("Method not implemented.");
 *     }
 *
 *     query(): Promise<void> {
 *         throw new Error("Method not implemented.");
 *     }
 *
 *     findDocumentById(id: string): Promise<null> {
 *         throw new Error("Method not implemented.");
 *     }
 *
 *     count(): Promise<number> {
 *         throw new Error("Method not implemented.");
 *     }
 *
 *     close(): Promise<void> {
 *         throw new Error("Method not implemented.");
 *     }
 *
 * }
 * ```
 *
 *
 * ## Build & Testing
 * 
 * ### How to build this project
 * Build requirements:
 * * Bash
 * * Have Rust ([cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)) and [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)) installed.
 * * Node JS Version (20/LTS Recommended)
 * 
 * ```bash
 * cd ts 
 * npm i
 * npm run build
 * ```
 * 
 * ### How to test the project
 * For now, we have enabled the implementation of the whole wasm + javascript integration.
 * In order to run it, write the following:
 * 
 * ```bash
 * cd ts 
 * npm i
 * npm run test
 * ```
 * 
 * # SDK Rerefence
 */
import wasmBuffer from "../../pkg/ridb_rust_bg.wasm";

import * as RIDBTypes from "ridb-rust";
export * as RIDBTypes from "ridb-rust";

export { BaseStorage } from 'ridb-rust';

/**
 * A simple plugin that overrides the docCreateHook and docRecoverHook methods.
 */
class MySimplePlugin extends RIDBTypes.BasePlugin {
    constructor() {
        super();
        this.docCreateHook = (schema, docs) => docs;
        this.docRecoverHook = (schema, docs) => docs;
    }
}




/**
 * Represents a RIDB (Rust IndexedDB) instance.
 * @class
 * @template T - The type of the schema record.
 */
export class RIDB<T extends RIDBTypes.SchemaTypeRecord> {
    private schemas: T;
    private plugins: Array<typeof RIDBTypes.BasePlugin> = [];

    private _internal: typeof import("ridb-rust") | undefined;
    private _db: RIDBTypes.Database<T> | undefined;

    /**
     * Creates an instance of RIDB.
     * @param options
     */
    constructor(
        options: {
            schemas: T,
            plugins?: Array<typeof RIDBTypes.BasePlugin>
        } & RIDBTypes.MigrationsParameter<T>
    ) {
        const {
            schemas,
            plugins= this.defaultPlugins
        } = options;

        this.schemas = schemas;
        this.plugins = plugins;
    }

    get defaultPlugins() {
        return [
            MySimplePlugin
        ]
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
    private async load(): Promise<typeof import("ridb-rust")> {
        this._internal ??= await import("ridb-rust").then(async (module) => {
            const wasmInstance = module.initSync(wasmBuffer);
            await module.default(wasmInstance);
            return module;
        });
        return this._internal!;
    }

    /**
     * Gets the RIDB module with the provided storage type.
     * @param {typeof RIDBTypes.BaseStorage<RIDBTypes.SchemaType>} [storageType] - The storage type to use.
     * @returns An object containing createStorage and apply functions.
     * @private
     */
    private getRIDBModule(
        storageType?: typeof RIDBTypes.BaseStorage<RIDBTypes.SchemaType>
    ) {
        return {
            createStorage: (schemas: RIDBTypes.SchemaTypeRecord) =>
                this.createStorage(schemas, storageType),
            apply: (
                plugins: Array<typeof RIDBTypes.BasePlugin> = []
            ) => plugins.map((Plugin) => new Plugin()),
        };
    }

    /**
     * Starts the database.
     * @param {typeof RIDBTypes.BaseStorage<RIDBTypes.SchemaType>} [storageType] - The storage type to use.
     * @returns {Promise<RIDBTypes.Database<T>>} A promise that resolves to the database instance.
     */
    async start(
        options?: {
            storageType?: typeof RIDBTypes.BaseStorage<RIDBTypes.SchemaType>,
            password?: string
        }
    ) {
        const {storageType, password} = options ?? {};
        const { Database } = await this.load();
        this._db ??= await Database.create(
            this.schemas,
            this.plugins,
            this.getRIDBModule(storageType),
            password
        );
        return this.db;
    }

    /**
     * Creates storage instances for the provided schemas.
     * @template J - The type of the schema record.
     * @param {J} schemas - The schema definitions.
     * @param {typeof RIDBTypes.BaseStorage<RIDBTypes.SchemaType>} [storageConstructor] - The storage constructor to use.
     * @returns An object mapping collection names to storage instances.
     * @private
     */
    private createStorage<J extends RIDBTypes.SchemaTypeRecord>(
        schemas: J,
        storageConstructor?: typeof RIDBTypes.BaseStorage<RIDBTypes.SchemaType>
    ) {
        if (!this._internal) {
            throw new Error("Start the database first");
        }
        const Storage = storageConstructor ?? this._internal.InMemory;
        return Object.keys(schemas).reduce(
            (storages, name) => ({
                ...storages,
                [name]: new Storage(name, schemas[name]),
            }),
            {}
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
