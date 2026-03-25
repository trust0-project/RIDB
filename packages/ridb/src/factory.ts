import type { SchemaTypeRecord } from "@trust0/ridb-core";
import { DirectDBAdapter } from "./adapters/standalone";
import { RIDBCore } from "./core";
import type { DBOptions, RIDBAbstract, StartOptions } from "./types";


async function load<T extends SchemaTypeRecord>(options: DBOptions<T>) {
  const { WorkerDBAdapter } = await import("./adapters/worker");
  return new WorkerDBAdapter<T>(options);
}

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
  let workerAdapter: RIDBAbstract<T> | undefined;
  const useWorker = options.worker && typeof SharedWorker !== "undefined";
  if (useWorker) {
    const lazyWorker: RIDBAbstract<T> = {
      async start(startOptions) {
        workerAdapter ??= await load(options);
        return workerAdapter.start(startOptions);
      },
      async close() {
        return workerAdapter?.close();
      },
      getCollections() {
        if (!workerAdapter) throw new Error("Start the database first");
        return workerAdapter.getCollections();
      },
      isStarted() {
        return workerAdapter?.isStarted() ?? false;
      },
    };
    return lazyWorker;
  } else {
    return new DirectDBAdapter<T>(new RIDBCore<T>(options));
  }
}
