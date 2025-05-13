import { SchemaTypeRecord, BaseStorage, BasePlugin, MigrationsParameter, Collection, Schema } from "@trust0/ridb-core";

/**
 * Represents a factory class for creating storage instances.
 * 
 * @template T The schema type record defining the database structure
 */
export type StorageClass<T extends SchemaTypeRecord> = {
  /**
   * Creates a storage instance with the specified parameters.
   * 
   * @param name The name of the database
   * @param schemas The schema definitions for the database collections
   * @param options Additional options for the storage implementation
   * @returns A Promise resolving to the created storage instance
   */
  create: (
    name: string,
    schemas: T,
    options: any
  ) => Promise<BaseStorage<T>>;
}

/**
 * Enumeration of built-in storage types supported by RIDB.
 */
export enum StorageType {
  /**
   * In-memory storage that doesn't persist data across page reloads
   */
  InMemory = "InMemory",
  
  /**
   * IndexedDB-based storage for persistent data in the browser
   */
  IndexDB = "IndexDB"
}

/**
 * Options for starting a database instance.
 * 
 * @template T The schema type record defining the database structure
 */
export type StartOptions<T extends SchemaTypeRecord> = {
  /**
   * The storage type or custom storage class implementation to use
   */
  storageType?: StorageClass<T> | StorageType;
  
  /**
   * Optional password for encrypting the database
   */
  password?: string;
  
  /**
   * Database name to use (overrides the name provided during initialization)
   */
  dbName?: string;
  
  /**
   * Additional custom options
   */
  [name: string]: any
}

/**
 * Options for initializing the RIDB database.
 *
 * @template T The schema type record defining the database structure
 */
export type DBOptions<T extends SchemaTypeRecord = SchemaTypeRecord> = {
  /**
   * Database name
   * 
   * @deprecated Use the dbName option in the start method instead.
   */
  dbName?: string,
  
  /**
   * Schema definitions for all collections in the database
   */
  schemas: T,
  
  /**
   * Optional plugins to extend database functionality
   */
  plugins?: Array<typeof BasePlugin>,
  
  /**
   * Whether to use a SharedWorker for database operations
   */
  worker?: boolean
} & MigrationsParameter<T>


/**
 * Map of pending requests used for worker communication.
 */
export type PendingRequests = Map<
  string,
  { resolve: (resp: any) => void; reject: (err: any) => void }
>;


/**
 * Abstract interface for RIDB implementations.
 * 
 * Defines the core operations that any RIDB adapter must implement.
 * 
 * @template T The schema type record defining the database structure
 */
export interface RIDBAbstract<T extends SchemaTypeRecord> {
  /**
   * Start the database with the given options.
   * 
   * @param options Optional configuration for startup
   * @returns A promise that resolves when the database has successfully started
   */
  start(options?: StartOptions<T>): Promise<void>;
  
  /**
   * Close the database connection.
   * 
   * @returns A promise that resolves when the database has been successfully closed
   */
  close(): Promise<void>;
  
  /**
   * Get the collections for this database.
   * 
   * @returns An object containing all collections defined in the schema
   */
  getCollections(): { [name in keyof T]: Collection<Schema<T[name]>> };
  
  /**
   * Check if the database has been started.
   * 
   * @returns True if the database is started, false otherwise
   */
  isStarted(): boolean;
} 

/**
 * Interface representing a shared worker instance and its session ID.
 */
export interface WorkerInstance {
  /**
   * The SharedWorker instance
   */
  worker: SharedWorker;
  
  /**
   * Unique session ID for this worker connection
   */
  sessionId: string;
}
  
  