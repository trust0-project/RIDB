/** biome-ignore-all lint/suspicious/noExplicitAny: Not needed here */
/** biome-ignore-all lint/style/noNonNullAssertion: Not needed here */

import type { Collection, Schema, SchemaTypeRecord } from "@trust0/ridb-core";
import { v4 as uuidv4 } from "uuid";
import type { DBOptions, PendingRequests, RIDBAbstract, StartOptions, WorkerInstance } from "../types";
import { WasmInternal } from "../wasm";

/**
 * An adapter for database operations that runs in a SharedWorker.
 *
 * This adapter provides a communication interface between the main thread and
 * a SharedWorker that handles actual database operations. This allows database
 * operations to be performed in a separate thread, improving performance and
 * avoiding main thread blocking.
 *
 * @template T The schema type record defining the database structure
 */
export class WorkerDBAdapter<T extends SchemaTypeRecord> implements RIDBAbstract<T> {
  /**
   * The SharedWorker instance handling database operations
   */
  private _worker: SharedWorker | undefined;

  /**
   * Unique session ID for this worker connection
   */
  private _sessionId: string | undefined;

  /**
   * Flag indicating whether the database has been started
   */
  private _started: boolean = false;

  /**
   * Map of pending requests awaiting worker responses
   */
  private _pendingRequests: PendingRequests = new Map();

  /**
   * Creates a new WorkerDBAdapter instance.
   *
   * @param options Database configuration options
   */
  constructor(protected options: DBOptions<T>) {}

  /**
   * Creates a new SharedWorker instance for RIDB.
   *
   * This method attempts to create a worker using the dynamic import URL first,
   * and falls back to require.resolve if that fails.
   *
   * @returns An object containing the worker instance and a unique session ID
   */
  createWorkerInstance(): WorkerInstance {
    const sessionId = uuidv4();
    let worker: SharedWorker;

    try {
      worker = new SharedWorker(new URL("@trust0/ridb/worker", import.meta.url), { type: "module" });
    } catch (_err) {
      const workerPath = require.resolve("@trust0/ridb/worker");
      worker = new SharedWorker(workerPath, { type: "module" });
    }

    return { worker, sessionId };
  }

  /**
   * Sets up message handling for worker communication.
   *
   * This method configures the worker port to handle message events and resolve
   * or reject pending promises based on the worker's response.
   *
   * @param worker The SharedWorker instance to configure
   * @param pendingRequests Map of pending requests awaiting responses
   * @param onMessage Optional callback for additional message handling
   */
  setupWorkerMessageHandler(worker: SharedWorker, pendingRequests: PendingRequests, onMessage?: (event: MessageEvent) => void) {
    worker.port.onmessage = (event: MessageEvent) => {
      const { requestId, status, data } = event.data || {};

      if (requestId && pendingRequests.has(requestId)) {
        const pendingRequest = pendingRequests.get(requestId)!;
        if (status === "success") {
          pendingRequest.resolve(data);
        } else {
          pendingRequest.reject(data);
        }
        pendingRequests.delete(requestId);
      }

      if (onMessage) {
        onMessage(event);
      }
    };
  }

  /**
   * Initializes the worker connection.
   *
   * Creates a SharedWorker instance and sets up message handling
   * for communication with the worker.
   *
   * @private
   */
  private async initializeWorker() {
    const { worker, sessionId } = this.createWorkerInstance();
    this._worker = worker;
    this._sessionId = sessionId;
    this.setupWorkerMessageHandler(worker, this._pendingRequests, async (event) => {
      const { RIDBError } = await WasmInternal();
      const { requestId, status, data } = event.data || {};
      if (status === "error" && requestId && this._pendingRequests.has(requestId)) {
        const pendingRequest = this._pendingRequests.get(requestId)!;
        const error = RIDBError.from(data);
        pendingRequest.reject(error);
        this._pendingRequests.delete(requestId);
      }
    });
  }

  /**
   * Gets the initialized worker, ensuring it has been created.
   *
   * @private
   * @throws Error if the worker has not been started
   * @returns The initialized SharedWorker instance
   */
  private get worker() {
    if (!this._worker) {
      throw new Error("Start the worker first");
    }
    return this._worker;
  }

  /**
   * The database name being used by this adapter
   */
  private dbName!: string;

  /**
   * Starts the database with the provided options.
   *
   * This method initializes the worker if needed, then sends a start message
   * to the worker to initialize the database.
   *
   * @param options Optional configuration for startup including storage type and encryption
   * @returns A Promise that resolves when the database has successfully started
   * @throws Error if dbName is missing
   */
  async start(options?: StartOptions<T>): Promise<void> {
    if (!this._worker) {
      await this.initializeWorker();
    }
    const dbName = options?.dbName || this.options.dbName!;
    if (!dbName) {
      throw new Error("dbName is required");
    }
    this.dbName = dbName;

    return new Promise((resolve, reject) => {
      this._pendingRequests.set(this._sessionId!, { resolve, reject });
      this.worker.port.postMessage({
        action: "start",
        requestId: this._sessionId!,
        data: {
          dbName: this.dbName,
          schemas: this.options.schemas,
          migrations: this.options.migrations || {},
          options,
        },
      });
    }).then(() => {
      this._started = true;
    });
  }

  /**
   * Closes the database connection and releases worker resources.
   *
   * Sends a close message to the worker and closes the worker port.
   *
   * @returns A Promise that resolves when the database has been successfully closed
   */
  async close(): Promise<void> {
    if (!this._worker) return;
    this.worker.port.postMessage({
      action: "close",
      requestId: this._sessionId!,
      data: {
        dbName: this.dbName,
      },
    });
    await this.worker.port.close();
    this._worker = undefined;
    this._started = false;
  }

  /**
   * Gets the collections for this database.
   *
   * Creates proxy objects that delegate collection operations to the worker,
   * allowing transparent access to collections as if they were local.
   *
   * @returns An object containing proxied collections for all schemas
   */
  getCollections(): { [name in keyof T]: Collection<Schema<T[name]>> } {
    // Create proxy collections for worker communication
    const result = {} as { [name in keyof T]: Collection<Schema<T[name]>> };
    const validOperations = ["find", "count", "findById", "update", "create", "delete"];

    for (const key of Object.keys(this.options.schemas) as Array<keyof T>) {
      result[key] = new Proxy({} as any, {
        get: (target, prop, receiver) => {
          if (validOperations.includes(prop.toString())) {
            return async (data: any) => {
              const requestId = uuidv4();
              return new Promise((resolve, reject) => {
                this._pendingRequests.set(requestId, { resolve, reject });
                this.worker.port.postMessage({
                  action: prop.toString(),
                  requestId,
                  data: {
                    dbName: this.dbName,
                    collection: key,
                    body: data,
                  },
                });
              });
            };
          }
          return Reflect.get(target, prop, receiver);
        },
      });
    }
    return result;
  }

  /**
   * Checks if the database has been started.
   *
   * @returns True if the database is started, false otherwise
   */
  isStarted(): boolean {
    return this._started;
  }
}
