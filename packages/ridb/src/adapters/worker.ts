import { v4 as uuidv4 } from 'uuid';
import { SchemaTypeRecord, Schema, Collection } from "@trust0/ridb-core";
import { StartOptions, PendingRequests, RIDBAbstract, WorkerInstance, DBOptions } from "../types";
import { WasmInternal } from "../wasm";

/**
 * An adapter for database operations that runs in a SharedWorker
 */
export class WorkerDBAdapter<T extends SchemaTypeRecord> implements RIDBAbstract<T> {
  private _worker: SharedWorker | undefined;
  private _sessionId: string | undefined;
  private _started: boolean = false;
  private _pendingRequests: PendingRequests = new Map();
/**
 * Creates a shared worker instance for RIDB
 */
 createWorkerInstance(): WorkerInstance {
  const sessionId = uuidv4();
  let worker: SharedWorker;
  
  try {
    worker = new SharedWorker(new URL('@trust0/ridb/worker', import.meta.url), { type: 'module' });
  } catch (err) {
    const workerPath = require.resolve('@trust0/ridb/worker');
    worker = new SharedWorker(workerPath, { type: 'module' });
  }
  
  return { worker, sessionId };
}

/**
 * Sets up worker message handling
 */
 setupWorkerMessageHandler(
  worker: SharedWorker, 
  pendingRequests: PendingRequests, 
  onMessage?: (event: MessageEvent) => void
) {
  worker.port.onmessage = (event: MessageEvent) => {
    const { requestId, status, data } = event.data || {};
    
    if (requestId && pendingRequests.has(requestId)) {
      const pendingRequest = pendingRequests.get(requestId)!;
      if (status === 'success') {
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

constructor(protected options: DBOptions<T>) { }

  private async initializeWorker() {
    const { worker, sessionId } = this.createWorkerInstance();
    this._worker = worker;
    this._sessionId = sessionId;
    this.setupWorkerMessageHandler(worker, this._pendingRequests, async (event) => {
      const { RIDBError } = await WasmInternal();
      const { requestId, status, data } = event.data || {};
      if (status === 'error' && requestId && this._pendingRequests.has(requestId)) {
        const pendingRequest = this._pendingRequests.get(requestId)!;
        const error = RIDBError.from(data);
        pendingRequest.reject(error);
        this._pendingRequests.delete(requestId);
      }
    });
  }

  private get worker() {
    if (!this._worker) {
      throw new Error("Start the worker first");
    }
    return this._worker;
  }
  
  private dbName!: string;

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
        action: 'start',
        requestId: this._sessionId!,
        data: {
          dbName: this.dbName,
          schemas: this.options.schemas,
          migrations: this.options.migrations || {},
          options
        }
      });
    }).then(() => {
      this._started = true;
    });
  }

  async close(): Promise<void> {
    if (!this._worker) return;
    this.worker.port.postMessage({
      action: 'close',
      requestId: this._sessionId!,
      data: {
        dbName: this.dbName,
      }
    });
    await this.worker.port.close();
    this._worker = undefined;
    this._started = false;
  }

  getCollections(): { [name in keyof T]: Collection<Schema<T[name]>> } {
    // Create proxy collections for worker communication
    const result = {} as { [name in keyof T]: Collection<Schema<T[name]>>; };
    const validOperations = ['find', 'count', 'findById', 'update', 'create', 'delete'];
    
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

  isStarted(): boolean {
    return this._started;
  }
} 