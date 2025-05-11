import {
  BasePlugin,
  Collection,
  Database,
  MigrationPathsForSchemas,
  Schema,
  SchemaTypeRecord
} from "@trust0/ridb-core";

import { v4 as uuidv4 } from 'uuid';
import { WasmInternal } from "./wasm";
import { StartOptions } from "./types";
import { RIDBCore } from "./core";
import { createWorkerInstance, setupWorkerMessageHandler } from "./worker-factory";

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


export class RIDB<T extends SchemaTypeRecord = SchemaTypeRecord> extends RIDBCore<T> {
  
  private _worker: SharedWorker | undefined;
  private _sessionId: string | undefined;

  get useWorker() {
    const useWorker = this.options.worker ?? false;
    const supportsWorker = typeof SharedWorker !== 'undefined';
    return useWorker && supportsWorker;
  }

  public authenticate(password: string) {
    return this.db?.authenticate(password) ?? false;
  }

  private get worker() {
    if (!this._worker) {
      throw new Error("Start the worker first");
    }
    return this._worker;
  }

  /**
   * Gets the collections from the database.
   * @returns The collections object.
   */
  private get dbCollections() {
    return this.db.collections;
  }

  private get workerCollections(): { [name in keyof T]: Collection<Schema<T[name]>>; } {
    const { schemas } = this.options;
    const result = {} as { [name in keyof T]: Collection<Schema<T[name]>>; };
    const validOperations = ['find', 'count', 'findById', 'update', 'create', 'delete'];
    for (const key of Object.keys(schemas) as Array<keyof T>) {
      result[key] = new Proxy({} as any, {
        get: (target, prop, receiver) => {
          if (validOperations.includes(prop.toString())) {
            return async (data: any) => {
              const requestId = uuidv4();
              console.log(`[RIDBWorker] Posting message to worker. Operation: ${prop.toString()}, Collection: ${String(key)}, Data:`, data);
              return new Promise((resolve, reject) => {
                this.pendingRequests.set(requestId, { resolve, reject });
                this.worker.port.postMessage({
                  action: prop.toString(),
                  requestId,
                  data: {
                    dbName: this.options.dbName,
                    collection: key,
                    body: data
                  }
                });
              });
            };
          }
          return Reflect.get(target, prop, receiver);
        }
      });
    }
    return result;
  }

  get collections() {
    return this.useWorker ? this.workerCollections : this.dbCollections;
  }

  private async initializeWorker() {
    const { worker, sessionId } = createWorkerInstance();
    this._worker = worker;
    this._sessionId = sessionId;
    setupWorkerMessageHandler(worker, this.pendingRequests, async (event) => {
      const { RIDBError } = await WasmInternal();
      // If the event contains error data, convert it to a proper error object
      const { requestId, status, data } = event.data || {};
      if (status === 'error' && requestId && this.pendingRequests.has(requestId)) {
        const pendingRequest = this.pendingRequests.get(requestId)!;
        const error = RIDBError.from(data);
        pendingRequest.reject(error);
        this.pendingRequests.delete(requestId);
      }
    });
  }

  /**
   * Starts the database.
   * @returns {Promise<void>} A promise that resolves when the database is started.
   * @param options
   */
  async start(
    options?: StartOptions<T>
  ): Promise<void> {
    const withWorker = this.useWorker;
    if (withWorker) {
      if (!this._worker) {
        await this.initializeWorker();
      }
      
      return new Promise((resolve, reject) => {
        this.pendingRequests.set(this._sessionId!, { resolve, reject });
        this.worker.port.postMessage({
          action: 'start',
          requestId: this._sessionId!,
          data: {
            dbName: this.options.dbName ?? this.dbName,
            schemas: this.options.schemas,
            migrations: this.options.migrations || {},
            options
          }
        });
      });
    } else {
      this._db ??= await this.createDatabase(options);
      await this.db.start();
      await this.db.authenticate(options?.password ?? "");
    }
    this.started = true;
  }

  async close() {
    if (this.useWorker) {
      this._worker?.port.postMessage({
        action: 'close',
        requestId: this._sessionId!,
        data: {
          dbName: this.options.dbName,
        }
      });
      await this._worker?.port.close();
      this._worker = undefined;
    } else {
      await this.db.close();
      this._db = undefined;
    }
    this.started = false;
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

