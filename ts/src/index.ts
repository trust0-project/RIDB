/**
 * @packageDocumentation
 *
 * <p align="center">
 *   <img src="../docs/logo.svg" alt="JavaScript Database" />
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
        this.docCreateHook = (
            schema,
            migration,
            docs
        ) => docs;
        this.docRecoverHook = (
            schema,
            migration,
            docs
        ) => docs;
    }
}




/**
 * Represents a RIDB (Rust IndexedDB) instance.
 * @class
 * @template T - The type of the schema record.
 */
export class RIDB<T extends RIDBTypes.SchemaTypeRecord> {

    private schemas: T;
    private migrations: RIDBTypes.MigrationPathsForSchemas<T>
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
            migrations = {} as RIDBTypes.MigrationPathsForSchemas<T>,
            plugins= this.defaultPlugins
        } = options;

        this.schemas = schemas;
        this.plugins = plugins;
        this.migrations = migrations;
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
     * @returns {Promise<RIDBTypes.Database<T>>} A promise that resolves to the database instance.
     * @param options
     */
    async start(
        options?: {
            storageType?: typeof RIDBTypes.BaseStorage<RIDBTypes.SchemaType>,
            password?: string,
            //Extra properties
            [name:string]: any
        }
    ): Promise<RIDBTypes.Database<T>> {
        const {storageType, password} = options ?? {};
        const { Database } = await this.load();
        this._db ??= await Database.create(
            this.schemas,
            this.migrations,
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
            (storages, name) => {
                return {
                    ...storages,
                    [name]: new Storage(
                        name,
                        schemas[name],
                        this.migrations[name]
                    ),
                }
            },
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
