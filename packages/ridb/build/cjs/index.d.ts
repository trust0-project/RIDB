// Generated by dts-bundle-generator v9.5.1

import { BaseStorage } from '@trust0/ridb-wasm';

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
 * await db.start({dbName: "demo"})
 * ```
 *
 * ### Using with encryption plugin
 * You can also optionally specify storageType with a compatible storage of your choice and an optional password to enable encryption plugin
 * ```typescript
 * await db.start({
 *     password: "my-password"
 *     db
 * })
 * ```
 *
 * A compatible storage should be a class implementing [StorageInternal<RIDBTypes.SchemaType> ](../docs/namespaces/RIDBTypes/classes/StorageInternal.md) and its methods.
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
 * await db.start({dbName: "demo"})
 * ```
 *
 * @class
 * @template T - The type of the schema record.
 */
export type StorageClass<T extends RIDBTypes.SchemaTypeRecord> = {
	create: (name: string, schemas: T, options: any) => Promise<BaseStorage<T>>;
};
export declare enum StorageType {
	InMemory = "InMemory",
	IndexDB = "IndexDB"
}
export type StartOptions<T extends RIDBTypes.SchemaTypeRecord> = {
	storageType?: StorageClass<T> | StorageType;
	password?: string;
	[name: string]: any;
};
export declare class RIDB<T extends RIDBTypes.SchemaTypeRecord = RIDBTypes.SchemaTypeRecord> {
	private schemas;
	private migrations;
	private plugins;
	private _db;
	private dbName;
	/**
	 * Creates an instance of RIDB.
	 * @param options
	 */
	constructor(options: {
		dbName: string;
		schemas: T;
		plugins?: Array<typeof RIDBTypes.BasePlugin>;
	} & RIDBTypes.MigrationsParameter<T>);
	private getStorageType;
	/**
	 * Gets the database instance. Throws an error if the database has not been started.
	 * @throws Will throw an error if the database is not started.
	 * @private
	 */
	private get db();
	get started(): boolean;
	/**
	 * Gets the collections from the database.
	 * @returns The collections object.
	 */
	get collections(): {
		[name in keyof T]: RIDBTypes.Collection<RIDBTypes.Schema<T[name]>>;
	};
	/**
	 * Loads the RIDB Rust module.
	 * @returns {Promise<typeof import("@trust0/ridb-wasm")>} A promise that resolves to the RIDB Rust module.
	 * @private
	 */
	static load(): Promise<typeof import("@trust0/ridb-wasm")>;
	/**
	 * Starts the database.
	 * @returns {Promise<RIDBTypes.Database<T>>} A promise that resolves to the database instance.
	 * @param options
	 */
	start(options?: StartOptions<T>): Promise<RIDBTypes.Database<T>>;
	close(): Promise<void>;
}
/**
 * An enumeration of schema field types.
 */
export declare const SchemaFieldType: {
	string: "string";
	number: "number";
	boolean: "boolean";
	array: "array";
	object: "object";
};

export {};
