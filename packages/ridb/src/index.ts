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
 * ### Using with SharedWorker
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
 * ### Using with encryption plugin
 * You can also optionally specify storageType with a compatible storage of your choice and an optional password to enable encryption plugin
 * ```typescript
 * await db.start({
 *     password: "my-password"
 * })
 * ```
 * 
 * A compatible storage should be a class implementing [BaseStorage<SchemaType> ](../../ridb-core/docs/classes/BaseStorage.md) and its methods.
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
export * from './types';
export * from './wasm';

/**
 * Main RIDB class that provides database functionality with optional worker support
 */
export class RIDB<T extends SchemaTypeRecord = SchemaTypeRecord> {
  private adapter: RIDBAbstract<T>;
  
  /**
   * Creates a new RIDB instance
   */
  constructor(private options: DBOptions<T>) {
    this.adapter = RIDBFactory.createAdapter<T>(options);
  }

  /**
   * Get the collections from the database
   */
  get collections(): { [name in keyof T]: Collection<Schema<T[name]>> } {
    return this.adapter.getCollections();
  }

  /**
   * Starts the database
   */
  async start(options?: StartOptions<T>): Promise<void> {
    await this.adapter.start(options);
  }

  /**
   * Closes the database
   */
  async close(): Promise<void> {
    await this.adapter.close();
  }

  /**
   * Whether the database has been started
   */
  get started(): boolean {
    return this.adapter.isStarted();
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

