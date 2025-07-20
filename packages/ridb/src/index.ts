import {
  Collection,
  Schema,
  SchemaTypeRecord
} from "@trust0/ridb-core";

import { DBOptions, RIDBAbstract, StartOptions } from "./types";
import { RIDBFactory } from "./factory";

/**
 * @packageDocumentation
 *
 * <p align="center">
 *      <img src="../../../resources/ridb-dark.svg" alt="JavaScript Database" />
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
 * RIDB is a secure, lightweight database designed for the web with a focus on performance and ease of use.
 * It supports multiple storage options, encryption, and migrations while maintaining a simple API.
 * 
 * ### Basic Usage
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
 * 
 * // Start the database
 * await db.start();
 * 
 * // Access collections
 * const demoCollection = db.collections.demo;
 * 
 * // Perform operations
 * await demoCollection.insert({ id: "doc1", title: "Example" });
 * ```
 * 
 * ### Using with SharedWorker
 * 
 * Use the `worker` option to run RIDB in a SharedWorker for improved performance and concurrency:
 * 
 * ```typescript
 * const db = new RIDB({
 *     dbName: "demo",
 *     schemas: {
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
 *         } as const
 *     },
 *     worker: true
 * })
 * ```
 * 
 * ### Using with Encryption Plugin
 * 
 * Enable data encryption by providing a password when starting the database:
 * 
 * ```typescript
 * await db.start({
 *     password: "my-password"
 * })
 * ```
 * 
 * You can also specify a custom storage implementation:
 * 
 * ```typescript
 * import { MyCustomStorage } from './my-storage';
 * 
 * await db.start({
 *     storageType: MyCustomStorage,
 *     password: "my-password"
 * })
 * ```
 * 
 * A compatible storage should be a class implementing [BaseStorage<SchemaType>](../../ridb-core/docs/classes/BaseStorage.md) and its methods.
 * 
 * ### Using with Migration Plugin
 * 
 * The migration plugin automatically migrates documents as you upgrade schemas:
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
 *                     // Transform document for version 1
 *                     doc.age = doc.age || 0;
 *                     return doc;
 *                 }
 *             }
 *         }
 *     }
 * )
 * 
 * await db.start()
 * ```
 * # SDK Reference
 */
export * from './types';
export * from './wasm';

/**
 * Main RIDB class that provides database functionality with optional worker support.
 * 
 * This class serves as the primary entry point for interacting with the RIDB database.
 * It manages the lifecycle of the database connection and provides access to collections.
 * 
 * @template T Schema type record defining the database schema structure
 */
export class RIDB<T extends SchemaTypeRecord = SchemaTypeRecord> {
  private adapter: RIDBAbstract<T>;
  
  /**
   * Creates a new RIDB instance.
   * 
   * @param options Database configuration options including schemas and optional worker settings
   * @example
   * ```typescript
   * const db = new RIDB({
   *   schemas: {
   *     users: {
   *       version: 1,
   *       primaryKey: 'id',
   *       type: SchemaFieldType.object,
   *       properties: {
   *         id: { type: SchemaFieldType.string },
   *         name: { type: SchemaFieldType.string }
   *       }
   *     }
   *   }
   * });
   * ```
   */
  constructor(private options: DBOptions<T>) {
    this.adapter = RIDBFactory.createAdapter<T>(options);
  }

  /**
   * Access the database collections.
   * 
   * @returns An object containing all collections defined in the schema
   * @example
   * ```typescript
   * // Get the users collection
   * const usersCollection = db.collections.users;
   * 
   * // Query documents
   * const allUsers = await usersCollection.find({}).exec();
   * ```
   */
  get collections(): { [name in keyof T]: Collection<Schema<T[name]>> } {
    return this.adapter.getCollections();
  }

  /**
   * Starts the database and initializes all collections.
   * 
   * @param options Optional configuration for startup including storage type and encryption
   * @returns A promise that resolves when the database has successfully started
   * @example
   * ```typescript
   * // Start with default options
   * await db.start();
   * 
   * // Start with encryption
   * await db.start({ password: "secure-password" });
   * 
   * // Start with custom storage
   * await db.start({ 
   *   storageType: StorageType.IndexDB,
   *   dbName: "myApp"
   * });
   * ```
   */
  async start(options?: StartOptions<T>): Promise<void> {
    await this.adapter.start(options);
  }

  /**
   * Closes the database connection and releases resources.
   * 
   * @returns A promise that resolves when the database has been successfully closed
   * @example
   * ```typescript
   * // Close the database connection
   * await db.close();
   * ```
   */
  async close(): Promise<void> {
    await this.adapter.close();
  }

  /**
   * Checks if the database has been successfully started.
   * 
   * @returns True if the database is started, false otherwise
   * @example
   * ```typescript
   * if (db.started) {
   *   // Database is ready for use
   *   const docs = await db.collections.users.find({}).exec();
   * } else {
   *   // Database needs to be started first
   *   await db.start();
   * }
   * ```
   */
  get started(): boolean {
    return this.adapter.isStarted();
  }
}