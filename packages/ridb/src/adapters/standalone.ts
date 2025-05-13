import {
  SchemaTypeRecord,
  Collection,
  Schema
} from "@trust0/ridb-core";
import { RIDBAbstract, StartOptions } from "../types";
import { RIDBCore } from "../core";

/**
 * A direct adapter for database operations that runs in the main thread.
 * 
 * This adapter provides a wrapper around the RIDBCore implementation,
 * allowing database operations to be performed directly in the main thread
 * without worker-based concurrency.
 * 
 * @template T The schema type record defining the database structure
 */
export class DirectDBAdapter<T extends SchemaTypeRecord> implements RIDBAbstract<T> {
  /**
   * Creates a new DirectDBAdapter instance.
   * 
   * @param core The RIDBCore instance to use for database operations
   */
  constructor(private core: RIDBCore<T>) {}

  /**
   * Starts the database with the provided options.
   * 
   * @param options Optional configuration for startup including storage type and encryption
   * @returns A Promise that resolves when the database has successfully started
   */
  async start(options?: StartOptions<T>): Promise<void> {
    await this.core.start(options);
  }

  /**
   * Closes the database connection and releases resources.
   * 
   * @returns A Promise that resolves when the database has been successfully closed
   */
  async close(): Promise<void> {
    await this.core.close();
  }

  /**
   * Gets the collections for this database.
   * 
   * @returns An object containing all collections defined in the schema
   */
  getCollections(): { [name in keyof T]: Collection<Schema<T[name]>> } {
    return this.core.collections;
  }

  /**
   * Checks if the database has been started.
   * 
   * @returns True if the database is started, false otherwise
   */
  isStarted(): boolean {
    return this.core.started;
  }
}