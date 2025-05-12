import { SchemaTypeRecord } from "@trust0/ridb-core";
import { DBOptions, RIDBAbstract } from "./types";
import { DirectDBAdapter } from "./adapters/standalone";
import { WorkerDBAdapter } from "./adapters/worker";
import { RIDBCore } from "./core";

/**
 * Factory to create the appropriate database adapter based on options
 */
export class RIDBFactory {
  /**
   * Creates a database adapter instance based on the provided options
   */
  static createAdapter<T extends SchemaTypeRecord>(options: DBOptions<T>): RIDBAbstract<T> {
    const useWorker = options.worker && typeof SharedWorker !== 'undefined';
    if (useWorker) {
        return new WorkerDBAdapter<T>(options);
    } else {
      return new DirectDBAdapter<T>(new RIDBCore<T>(options));
    }
  }
} 