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
 * )
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

// @ts-ignore @ignore
import { BasePlugin, BaseStorage, Collection, Database, MigrationPathsForSchemas, MigrationsParameter, Schema, SchemaTypeRecord } from "@trust0/ridb-core";
import { v4 as uuidv4 } from 'uuid';

export type StorageClass<T extends SchemaTypeRecord> = {
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
  storageType?: StorageClass<T> | StorageType;
  password?: string;
  dbName?: string;
  [name: string]: any
}

/**
 * Options for the RIDB constructor.
 *
 * @typedef {DBOptions}
 * @template {SchemaTypeRecord} [T=SchemaTypeRecord] 
 */
export type DBOptions<T extends SchemaTypeRecord = SchemaTypeRecord> = {
  /**
   * @deprecated Use the dbName option in the start method instead.
   */
  dbName?: string,
  schemas: T,
  plugins?: Array<typeof BasePlugin>,
  worker?: boolean
} & MigrationsParameter<T>

// @ts-ignore @ignore
import wasmBuffer from "@trust0/ridb-core/wasm";

let loaded : typeof import("@trust0/ridb-core") | undefined;

export async function WasmInternal() {
    if (!loaded) {
        const module = await import("@trust0/ridb-core");
        const wasmInstance = module.initSync(wasmBuffer);
        await module.default(wasmInstance);
        loaded = module;
    }
    return loaded;
};

export type PendingRequests = Map<
  string,
  { resolve: (resp: any) => void; reject: (err: any) => void }
>;


export class RIDB<T extends SchemaTypeRecord = SchemaTypeRecord> {
  private _db: Database<T> | undefined;
  private _worker: SharedWorker | undefined;
  private _sessionId: string | undefined;
  public started: boolean = false;

  private pendingRequests:PendingRequests = new Map();

  private get dbName() {
    return this.options.dbName;
  }

  private get schemas() {
    return this.options.schemas;
  }

  private get migrations() {
    return this.options.migrations ?? {} as MigrationPathsForSchemas<T>;
  }

  private get plugins() {
    return this.options.plugins ?? [];
  }

  get useWorker() {
    const useWorker = this.options.worker ?? false;
    const supportsWorker = typeof SharedWorker !== 'undefined';
    return useWorker && supportsWorker;
  }

  public authenticate(password: string) {
    return this.db?.authenticate(password) ?? false;
  }

  /**
   * Creates an instance of RIDB.
   * @param options
   */
  constructor(private options: DBOptions<T>) { }

  private async getStorageType<T extends StorageType>(storageType: T) {
    const { InMemory, IndexDB } = await WasmInternal();
    return storageType === StorageType.InMemory ?
      InMemory :
      IndexDB;
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

  private createWorker() {
    let worker: SharedWorker | undefined;
    try {
      worker = new SharedWorker(new URL('@trust0/ridb/worker', import.meta.url), { type: 'module' });
    } catch (err) {
      const workerPath = require.resolve('@trust0/ridb/worker');
      worker = new SharedWorker(workerPath, { type: 'module' });
    }
    worker.port.onmessage = this.handleWorkerMessage.bind(this);
    return worker;
  }

  private async handleWorkerMessage(event: MessageEvent) {
    const { RIDBError } = await WasmInternal();
    const { requestId, status, data } = event.data || {};
    console.log('[RIDBWorker] Received message from worker:', event.data);
    if (requestId && this.pendingRequests.has(requestId)) {
      const pendingRequest = this.pendingRequests.get(requestId)!;
      if (status === 'success') {
        console.log(`[RIDBWorker] Request ${requestId} successful. Data:`, data);
        pendingRequest.resolve(data);
      } else {
        console.error(`[RIDBWorker] Request ${requestId} failed. Error:`, data);
        const error = RIDBError.from(data);
        pendingRequest.reject(error);
      }
      this.pendingRequests.delete(requestId);
    } 
  }

  private async createDatabase(options?: StartOptions<T>) {
    await WasmInternal();
    const { storageType, password } = options ?? {};
    const StorageClass = typeof storageType === "string" ?
      await this.getStorageType(storageType) :
      storageType ?? undefined;
    if (StorageClass && !StorageClass.create) {
      throw new Error("Your storage does not have an async create function, please check documentation")
    }
    const dbName = options?.dbName ?? this.dbName;
    if (!dbName) {
      throw new Error("dbName is required");
    }
    const storage = StorageClass ?
      await StorageClass.create(dbName, this.schemas, options) :
      undefined;

    return Database.create<T>(
      dbName,
      this.schemas,
      this.migrations,
      this.plugins,
      {
        apply: (plugins: Array<typeof BasePlugin> = []) => plugins.map((Plugin) => new Plugin()),
      },
      password,
      storage
    );
  }

  /**
   * Starts the database.
   * @returns {Promise<Database<T>>} A promise that resolves to the database instance.
   * @param options
   */
  async start(
    options?: StartOptions<T>
  ): Promise<void> {
    const withWorker = this.useWorker;
    if (withWorker) {
      this._sessionId ??= uuidv4();
      this._worker ??= this.createWorker();
      return new Promise((resolve, reject) => {
        this.pendingRequests.set(this._sessionId!, { resolve, reject });
        this.worker.port.postMessage({
          action: 'start',
          requestId:this._sessionId!,
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
        requestId:this._sessionId!,
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

