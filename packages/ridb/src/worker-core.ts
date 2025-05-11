import { StorageType } from './types';
import { WasmInternal } from './wasm';

/**
 * Core implementation for the worker context
 * This avoids circular dependencies between worker.ts and RIDBCore
 */
export class WorkerDB {
  private db: any;
  private started = false;

  constructor(
    private dbName: string,
    private schemas: any,
    private migrations: any = {}
  ) {}

  async start(options: any = {}) {
    if (this.started) return;

    const { storageType, password } = options;
    
    // Initialize WASM and get Database constructor
    const { Database, InMemory, IndexDB } = await WasmInternal();
    
    // Determine storage class based on options
    let StorageClass;
    if (typeof storageType === 'string') {
      StorageClass = storageType === StorageType.InMemory ? InMemory : IndexDB;
    } else {
      StorageClass = storageType;
    }

    // Create storage if a storage class is specified
    let storage;
    if (StorageClass) {
      if (!StorageClass.create) {
        throw new Error("Your storage does not have an async create function");
      }
      storage = await StorageClass.create(this.dbName, this.schemas, options);
    }

    // Create the database
    this.db = await Database.create(
      this.dbName,
      this.schemas,
      this.migrations,
      [], // No plugins in worker context
      {
        apply: (plugins: any = []) => plugins.map((Plugin: any) => new Plugin()),
      },
      password,
      storage
    );

    await this.db.start();
    await this.db.authenticate(password || "");
    this.started = true;
  }

  async close() {
    if (!this.started) return;
    
    await this.db.close();
    this.db = null;
    this.started = false;
  }

  authenticate(password: string) {
    return this.db?.authenticate(password) ?? false;
  }

  get collections() {
    if (!this.db) {
      throw new Error("Start the database first");
    }
    return this.db.collections;
  }

  // Database operations
  async find(collection: string, query: any) {
    return this.collections[collection].find(query);
  }

  async count(collection: string, query: any) {
    return this.collections[collection].count(query);
  }

  async create(collection: string, data: any) {
    return this.collections[collection].create(data);
  }

  async update(collection: string, data: any) {
    return this.collections[collection].update(data);
  }

  async findById(collection: string, id: string) {
    return this.collections[collection].findById(id);
  }

  async delete(collection: string, id: string) {
    return this.collections[collection].delete(id);
  }
} 