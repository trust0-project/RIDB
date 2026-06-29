/**
 * @packageDocumentation
 *
 * <p align="center">
 *      <img src="../../../resources/ridb-dark.svg" alt="JavaScript Database" />
 *  <br />
 *  <br />
 *  <h3 align="center">A secure light-weight and dependency free database wrapper for the web.</h3>
 * </p>
 * <p align="center">
 *   <a href="https://github.com/trust0-project/RIDB/releases"><img src="https://img.shields.io/github/v/release/trust0-project/ridb?color=%23ff00a0&include_prereleases&label=version&sort=semver&style=flat-square"></a>
 *   <a href="#"><img src="https://img.shields.io/npm/types/rxdb?style=flat-square"></a>
 *   <a href="https://raw.githubusercontent.com/trust0-project/RIDB/refs/heads/main/LICENSE"><img src="https://img.shields.io/github/license/trust0-project/ridb?style=flat-square"></a>
 *   <a href="https://www.npmjs.com/package/@trust0/ridb"><img src="https://img.shields.io/npm/dm/@trust0/ridb?color=c63a3b&style=flat-square"></a>
 * </p>
 *
 * # Features
 * Provides MongoDB storage backend for RIDB with full CRUD operations and query support.
 * Supports all MongoDB connection options and authentication methods.
 *
 * # SDK Rerefence
 */
/** biome-ignore-all lint/suspicious/noExplicitAny: Not needed here */

import { WasmInternal } from "@trust0/ridb";
import {
  type BaseStorageOptions,
  type BaseStorage as BaseStorageType,
  type Doc,
  type Operation,
  OpType,
  type QueryOptions,
  type QueryType,
  type SchemaTypeRecord,
} from "@trust0/ridb-core";

import type { Collection, Db, MongoClient } from "mongodb";

export interface MongoDBConfig {
  /** MongoDB connection URL */
  url?: string;
  /** Database name in MongoDB */
  dbName?: string;
  /** MongoDB client options */
  mongoOptions?: Record<string, any>;
}

export type MongoDBStorageOptions = BaseStorageOptions & MongoDBConfig;

/**
 * Create a MongoDB storage instance
 * @public
 * @returns A factory function that creates MongoDB storage instances
 */
export async function createMongoDB(): Promise<typeof BaseStorageType> {
  const { BaseStorage } = await WasmInternal();

  // We need to extend the actual BaseStorage from WasmInternal
  /**
   * MongoDB storage implementation class
   * @public
   */
  class MongoDBStorage<T extends SchemaTypeRecord> extends BaseStorage<T> {
    private client?: MongoClient;
    private db?: Db;
    private mongoConfig: MongoDBConfig;

    constructor(name: string, schemas: T, options: MongoDBStorageOptions = {} as MongoDBStorageOptions) {
      super(name, schemas, options);
      this.mongoConfig = options;
    }

    /**
     * Utility method to recursively convert ObjectId values to strings throughout an object
     * This ensures that MongoDB's ObjectId instances are never returned to the application layer,
     * maintaining consistency with RIDB's string-based approach and preventing serialization issues.
     * Also removes MongoDB's automatic _id field entirely from results.
     * @private
     */
    private convertObjectIdsToStrings(obj: any): any {
      if (!obj || typeof obj !== "object") {
        return obj;
      }

      // Handle ObjectId
      if (obj.constructor?.name === "ObjectId" || (obj._bsontype && obj._bsontype === "ObjectId")) {
        const stringValue = obj.toString();
        return stringValue;
      }

      // Handle arrays
      if (Array.isArray(obj)) {
        return obj.map((item) => this.convertObjectIdsToStrings(item));
      }

      // Handle plain objects
      const converted: any = {};
      for (const [key, value] of Object.entries(obj)) {
        // Skip MongoDB's _id field entirely
        if (key === "_id") {
          continue;
        }
        converted[key] = this.convertObjectIdsToStrings(value);
      }

      return converted;
    }

    /**
     * Normalize a document into the exact JSON shape that RIDB hashed at write
     * time, so the integrity plugin's recover-time check matches.
     *
     * RIDB's integrity plugin computes its hash with `JSON.stringify`, which
     * serializes values through their `toJSON()` (e.g. `Date` -> ISO string,
     * `ObjectId` -> hex string). A document read back from MongoDB, however,
     * contains live BSON wrapper instances (`Date`, `ObjectId`, `Long`,
     * `Decimal128`, `Binary`, ...). The previous recursion only special-cased
     * `ObjectId` and treated every other wrapper as a plain object, mangling it
     * (a `Date` collapsed to `{}`), which diverged from the hashed value and
     * broke the integrity check.
     *
     * Round-tripping through `JSON.stringify`/`JSON.parse` reproduces the
     * integrity plugin's serialization exactly, and dropping MongoDB's injected
     * top-level `_id` mirrors what was originally written (RIDB never stores an
     * `_id`).
     * @private
     */
    private normalizeDocument(doc: any): any {
      if (!doc || typeof doc !== "object") {
        return doc;
      }
      const { _id,  ...rest } = doc as Record<string, unknown>;
      return this.stripNullFields(JSON.parse(JSON.stringify(rest)));
    }

    /**
     * Recursively remove every `null` field from a document before it is
     * returned to the application layer. MongoDB can persist explicit `null`
     * values (e.g. from sparse writes), but RIDB treats an absent field and a
     * `null` field differently, so we drop them entirely to keep the returned
     * shape consistent with what was originally written.
     * @private
     */
    private stripNullFields(value: any): any {
      if (Array.isArray(value)) {
        return value.map((item) => this.stripNullFields(item));
      }

      if (value && typeof value === "object") {
        const result: any = {};
        for (const [key, val] of Object.entries(value)) {
          if (val === null) {
            continue;
          }
          result[key] = this.stripNullFields(val);
        }
        return result;
      }

      return value;
    }

    /**
     * Create a new MongoDB storage instance
     * @param name - Database name
     * @param schemas - Collection schemas
     * @param options - Storage options including MongoDB connection details
     * @returns A new Instance of MongoDB storage
     * @public
     */
    static async create<SchemasCreate extends SchemaTypeRecord>(name: string, schemas: SchemasCreate, options: MongoDBStorageOptions = {}) {
      const db = new MongoDBStorage<SchemasCreate>(name, schemas, options);
      return db;
    }

    /** Start the database connection */
    async start(): Promise<void> {
      const { MongoClient } = await import("mongodb");
      const url = this.mongoConfig.url || process.env.MONGODB_URL;
      if (!url) {
        throw new Error("MONGODB_URL is required");
      }
      const dbName = this.mongoConfig.dbName || (this as any).name;
      this.client = new MongoClient(url, {
        connectTimeoutMS: 5000,
        serverSelectionTimeoutMS: 5000,
        socketTimeoutMS: 5000,
        maxPoolSize: 10,
        minPoolSize: 5,
        maxIdleTimeMS: 5000,
        maxConnecting: 5,
      });
      await this.client.connect();
      this.db = this.client.db(dbName);
    }

    /** Close the database connection */
    async close(): Promise<void> {
      if (this.client) {
        await this.client.close();
        this.client = undefined;
        this.db = undefined;
      }
    }

    /** Get MongoDB collection for a given collection name */
    private getCollection(collectionName: string): Collection {
      if (!this.db) {
        throw new Error("Database not started. Call start() first.");
      }
      const collection = this.db.collection(collectionName);
      return collection;
    }

    /** Find a document by its ID */
    async findDocumentById(collectionName: keyof T, id: string): Promise<Doc<T[keyof T]> | null> {
      const collection = this.getCollection(String(collectionName));
      const schema = this.getSchema(String(collectionName));
      const primaryKey = schema.primaryKey;

      const query = { [primaryKey]: id };

      const doc = await collection.findOne(query, { projection: { _id: 0 } });

      if (!doc) {
        return null;
      }

      return this.normalizeDocument(doc) as Doc<T[keyof T]>;
    }

    /** Write an operation (insert, update, delete) */
    async write(op: Operation<T[keyof T]>): Promise<Doc<T[keyof T]>> {
      const collectionName = String(op.collection);
      const collection = this.getCollection(collectionName);
      const { primaryKey } = this.getSchema(collectionName);

      switch (op.opType) {
        case OpType.CREATE: {
          const id = primaryKey in op.data ? (op.data as any)[primaryKey] : null;

          if (!id) {
            throw new Error("Document ID is required");
          }

          // Check if document already exists
          const existQuery = { [primaryKey]: id };
          const existing = await collection.findOne(existQuery, { projection: { _id: 0, tenantId: 0 } });

          if (existing) {
            throw new Error(`Document with ${primaryKey} '${id}' already exists`);
          }

          // Prevent MongoDB from auto-generating _id by explicitly setting it to null
          const docToInsert = { ...(op.data as any) };

          await collection.insertOne(docToInsert);

          const result = this.normalizeDocument(docToInsert);
          return result;
        }
        case OpType.UPDATE: {
          const id = primaryKey in op.data ? (op.data as any)[primaryKey] : null;

          if (!id) {
            throw new Error("Document ID is required");
          }

          const updateQuery = { [primaryKey]: id };
          const updateData = { $set: op.data };

          const result = await collection.updateOne(updateQuery, updateData);

          if (result.matchedCount === 0) {
            throw new Error("Document ID not found");
          }

          const convertedResult = this.normalizeDocument(op.data);
          return convertedResult;
        }
        case OpType.DELETE: {
          const id = op.data;

          const deleteQuery = { [primaryKey]: id };

          const result = await collection.deleteOne(deleteQuery);

          if (result.deletedCount === 0) {
            throw new Error("Document ID not found");
          }

          const convertedResult = this.convertObjectIdsToStrings(op.data);
          return convertedResult;
        }
        default:
          throw new Error(`Unknown operation type: ${op.opType}`);
      }
    }

    /** Convert RIDB query to MongoDB filter */
    private convertQueryToMongoFilter(query: QueryType<T[keyof T]>, _schema: any): any {
      if (!query || typeof query !== "object") {
        return {};
      }

      const filter: any = {};

      for (const [key, value] of Object.entries(query)) {
        if (value === null || value === undefined) {
          filter[key] = { $exists: false };
        } else if (typeof value === "object" && value !== null) {
          // Convert any ObjectId values to strings in query operators
          if ("$in" in value) {
            filter[key] = { $in: this.convertObjectIdsToStrings(value.$in) };
          } else if ("$nin" in value) {
            filter[key] = { $nin: this.convertObjectIdsToStrings(value.$nin) };
          } else if ("$gt" in value) {
            filter[key] = { $gt: this.convertObjectIdsToStrings(value.$gt) };
          } else if ("$gte" in value) {
            filter[key] = { $gte: this.convertObjectIdsToStrings(value.$gte) };
          } else if ("$lt" in value) {
            filter[key] = { $lt: this.convertObjectIdsToStrings(value.$lt) };
          } else if ("$lte" in value) {
            filter[key] = { $lte: this.convertObjectIdsToStrings(value.$lte) };
          } else if ("$ne" in value) {
            filter[key] = { $ne: this.convertObjectIdsToStrings(value.$ne) };
          } else if ("$regex" in value) {
            filter[key] = { $regex: value.$regex };
          } else {
            // For complex objects, convert ObjectIds and use exact match
            filter[key] = this.convertObjectIdsToStrings(value);
          }
        } else {
          filter[key] = this.convertObjectIdsToStrings(value);
        }
      }

      return filter;
    }

    /** Count documents matching a query (supports offset & limit) */
    async count(collectionName: keyof T, query: QueryType<T[keyof T]>, options?: QueryOptions): Promise<number> {
      const collection = this.getCollection(String(collectionName));
      const schema = this.getSchema(String(collectionName));

      // For MongoDB, we use the native query system instead of the RIDB Query class
      // to get better performance, but we could also use Query class for consistency
      const filter = this.convertQueryToMongoFilter(query, schema);

      const countOptions: any = {};
      if (options?.offset) {
        countOptions.skip = options.offset;
      }
      if (options?.limit) {
        countOptions.limit = options.limit;
      }

      const count = await collection.countDocuments(filter, Object.keys(countOptions).length > 0 ? countOptions : undefined);

      return count;
    }

    /** Find documents matching a query with pagination */
    async find(collectionName: keyof T, query: QueryType<T[keyof T]>, options?: QueryOptions): Promise<Doc<T[keyof T]>[]> {
      const collection = this.getCollection(String(collectionName));
      const schema = this.getSchema(String(collectionName));

      // For MongoDB, we use the native query system for better performance
      const filter = this.convertQueryToMongoFilter(query, schema);

      let findQuery = collection.find(filter, { projection: { _id: 0 } });

      // Apply offset and limit if provided
      if (options?.offset) {
        findQuery = findQuery.skip(options.offset);
      }

      if (options?.limit) {
        findQuery = findQuery.limit(options.limit);
      }

      const docs = await findQuery.toArray();

      // Normalize each document back to the exact JSON shape RIDB hashed at
      // write time so the integrity check matches on recovery.
      const convertedDocs = docs.map((doc) => this.normalizeDocument(doc)) as Doc<T[keyof T]>[];

      return convertedDocs;
    }
  }

  // `MongoDBStorage` is a concrete subclass of the WASM `BaseStorage`, so its
  // constructor type is assignable to the generic `typeof BaseStorage` that
  // consumers (e.g. RIDB's `storageType` / the Identus SDK) expect — no cast.
  return MongoDBStorage;
}
