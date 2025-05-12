import {
  SchemaTypeRecord,
  Collection,
  Schema
} from "@trust0/ridb-core";
import { RIDBAbstract, StartOptions } from "../types";
import { RIDBCore } from "../core";

/**
 * A direct adapter for database operations that runs in the main thread
 */
export class DirectDBAdapter<T extends SchemaTypeRecord> implements RIDBAbstract<T> {
  constructor(private core: RIDBCore<T>) {}

  async start(options?: StartOptions<T>): Promise<void> {
    await this.core.start(options);
  }

  async close(): Promise<void> {
    await this.core.close();
  }


  getCollections(): { [name in keyof T]: Collection<Schema<T[name]>> } {
    return this.core.collections;
  }

  isStarted(): boolean {
    return this.core.started;
  }
}