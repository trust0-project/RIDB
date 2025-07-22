import type { SchemaTypeRecord } from "@trust0/ridb-core";
import { DirectDBAdapter } from "./adapters/standalone";
import { WorkerDBAdapter } from "./adapters/worker";
import { RIDBCore } from "./core";
import type { DBOptions, RIDBAbstract } from "./types";

/**
 * Creates a database adapter instance based on the provided options.
 *
 * This function determines whether to use a direct database adapter or a worker-based adapter
 * depending on the provided options and the browser's support for SharedWorker.
 *
 * @template T The schema type record defining the database structure
 * @param options Configuration options for the database
 * @returns An appropriate RIDBAbstract implementation:
 *          - WorkerDBAdapter if worker option is true and SharedWorker is supported
 *          - DirectDBAdapter otherwise
 */
export function createAdapter<T extends SchemaTypeRecord>(options: DBOptions<T>): RIDBAbstract<T> {
  const useWorker = options.worker && typeof SharedWorker !== "undefined";
  if (useWorker) {
    return new WorkerDBAdapter<T>(options);
  } else {
    return new DirectDBAdapter<T>(new RIDBCore<T>(options));
  }
}
