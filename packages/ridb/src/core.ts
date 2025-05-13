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
 * Core implementation of RIDB functionality that powers both direct database operations
 * and worker-based operations.
 * 
 * This class provides the fundamental database operations including initialization,
 * authentication, and lifecycle management. It serves as the central implementation
 * that can be used by both the main RIDB class and worker adapters.
 * 
 * @template T The schema type record defining the database structure
 */
export class RIDBCore<T extends SchemaTypeRecord = SchemaTypeRecord> {
  /**
   * The underlying database instance
   */
  protected _db: Database<T> | undefined;
  
  /**
   * Flag indicating whether the database has been started
   */
  public started: boolean = false;
  
  /**
   * Map of pending requests for asynchronous operations
   */
  protected pendingRequests: PendingRequests = new Map();

  /**
   * Creates a new RIDBCore instance.
   * 
   * @param options Database configuration options
   */
  constructor(protected options: DBOptions<T>) { }

  /**
   * Gets the configured database name from options.
   * 
   * @returns The database name or undefined if not set
   */
  protected get dbName() {
    return this.options.dbName;
  }

  /**
   * Gets the schema definitions for the database.
   * 
   * @returns Schema definitions for collections
   */
  protected get schemas() {
    return this.options.schemas;
  }

  /**
   * Gets the migration definitions for database schema updates.
   * 
   * @returns Migration paths for schemas or an empty object if not defined
   */
  protected get migrations() {
    return this.options.migrations ?? {} as MigrationPathsForSchemas<T>;
  }

  /**
   * Gets the plugins to extend database functionality.
   * 
   * @returns Array of plugins or empty array if none defined
   */
  protected get plugins() {
    return this.options.plugins ?? [];
  }

  /**
   * Resolves a storage type enum to its implementation.
   * 
   * @template T The storage type
   * @param storageType The storage type enum value to resolve
   * @returns The corresponding storage implementation
   */
  protected async getStorageType<T extends StorageType>(storageType: T) {
    const { InMemory, IndexDB } = await WasmInternal();
    return storageType === StorageType.InMemory ?
      InMemory :
      IndexDB;
  }

  /**
   * Gets the database instance, ensuring it has been initialized.
   * 
   * @throws Error if the database has not been started
   * @returns The initialized database instance
   */
  get db() {
    if (!this._db) {
      throw new Error("Start the database first");
    }
    return this._db;
  }

  /**
   * Gets the collections from the database.
   * 
   * @throws Error if the database has not been started
   * @returns Object containing all collections defined in the schema
   */
  get collections() {
    return this.db.collections;
  }

  /**
   * Authenticates with the database using the provided password.
   * 
   * @param password The password for encryption/decryption
   * @returns True if authentication is successful, false otherwise
   */
  public authenticate(password: string) {
    return this.db?.authenticate(password) ?? false;
  }

  /**
   * Creates a new database instance based on the provided options.
   * 
   * @param options Optional configuration for storage and initialization
   * @returns A Promise resolving to the created Database instance
   * @throws Error if the storage class is invalid or if dbName is missing
   */
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
   * Starts the database with the provided options.
   * 
   * This method initializes the database if needed, starts it, and performs authentication
   * if a password is provided.
   * 
   * @param options Optional configuration for startup including storage type and encryption
   * @returns A Promise that resolves when the database has successfully started
   * @example
   * ```typescript
   * const core = new RIDBCore({
   *   dbName: "myDatabase",
   *   schemas: { 
   *     users: {
   *       version: 1,
   *       primaryKey: 'id',
   *       type: 'object'
   *     }
   *   }
   * });
   * 
   * // Start with default options
   * await core.start();
   * 
   * // Start with encryption
   * await core.start({ password: "my-password" });
   * ```
   */
  async start(options?: StartOptions<T>): Promise<void> {
    this._db ??= await this.createDatabase(options);
    await this.db.start();
    await this.db.authenticate(options?.password ?? "");
    this.started = true;
  }

  /**
   * Closes the database and releases resources.
   * 
   * @returns A Promise that resolves when the database has been successfully closed
   * @example
   * ```typescript
   * // Close the database to release resources
   * await core.close();
   * ```
   */
  async close() {
    await this.db.close();
    this._db = undefined;
    this.started = false;
  }
}

/**
 * An enumeration of schema field types for defining document structures.
 * 
 * @deprecated Use SchemaFieldType from index.ts instead
 */
export const SchemaFieldType = {
  string: 'string' as const,
  number: 'number' as const,
  boolean: 'boolean' as const,
  array: 'array' as const,
  object: 'object' as const,
}; 