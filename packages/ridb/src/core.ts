import {
  BasePlugin,
  Database,
  MigrationPathsForSchemas,
  SchemaTypeRecord
} from "@trust0/ridb-core";

import { 
  DBOptions, 
  PendingRequests, 
  StartOptions, 
  StorageType 
} from "./types";

import { WasmInternal } from "./wasm";

/**
 * Core implementation of RIDB functionality that can be used
 * directly by both the main RIDB class and the worker.
 */
export class RIDBCore<T extends SchemaTypeRecord = SchemaTypeRecord> {
  protected _db: Database<T> | undefined;
  public started: boolean = false;
  protected pendingRequests:PendingRequests = new Map();

  /**
   * Creates an instance of RIDBImplementation.
   * @param options
   */
  constructor(protected options: DBOptions<T>) { }

  protected get dbName() {
    return this.options.dbName;
  }

  protected get schemas() {
    return this.options.schemas;
  }

  protected get migrations() {
    return this.options.migrations ?? {} as MigrationPathsForSchemas<T>;
  }

  protected get plugins() {
    return this.options.plugins ?? [];
  }

  protected async getStorageType<T extends StorageType>(storageType: T) {
    const { InMemory, IndexDB } = await WasmInternal();
    return storageType === StorageType.InMemory ?
      InMemory :
      IndexDB;
  }

  /**
   * Gets the database instance. Throws an error if the database has not been started.
   * @throws Will throw an error if the database is not started.
   */
  get db() {
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

  public authenticate(password: string) {
    return this.db?.authenticate(password) ?? false;
  }

  protected async createDatabase(options?: StartOptions<T>) {
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
   * @returns {Promise<void>} A promise that resolves when the database is started.
   * @param options
   */
  async start(options?: StartOptions<T>): Promise<void> {
    this._db ??= await this.createDatabase(options);
    await this.db.start();
    await this.db.authenticate(options?.password ?? "");
    this.started = true;
  }

  async close() {
    await this.db.close();
    this._db = undefined;
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