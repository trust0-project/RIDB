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
import {
    OpType, 
    QueryOptions, 
    SchemaTypeRecord, 
    Doc, 
    Operation,
    QueryType,  
    BaseStorage,
    BaseStorageOptions
 } from "@trust0/ridb-core"
import {
    WasmInternal
 } from "@trust0/ridb";

import type { MongoClient, Db, Collection, ObjectId } from "mongodb";

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
 * MongoDB storage implementation class
 * @public
 */
export class MongoDBStorage<T extends SchemaTypeRecord> extends BaseStorage<T> {
    private client?: MongoClient;
    private db?: Db;
    private mongoConfig: MongoDBConfig;

    constructor(name: string, schemas: T, options: MongoDBStorageOptions = {} as MongoDBStorageOptions) {
        super(name, schemas, options);
        this.mongoConfig = options;
        console.log(`[MongoDBStorage] Constructor called with name: ${name}, schemas:`, Object.keys(schemas), 'options:', options);
    }
    
    /**
     * Utility method to recursively convert ObjectId values to strings throughout an object
     * This ensures that MongoDB's ObjectId instances are never returned to the application layer,
     * maintaining consistency with RIDB's string-based approach and preventing serialization issues.
     * Also removes MongoDB's automatic _id field entirely from results.
     * @private
     */
    private convertObjectIdsToStrings(obj: any): any {
        if (!obj || typeof obj !== 'object') {
            return obj;
        }
        
        // Handle ObjectId
        if (obj.constructor?.name === 'ObjectId' || (obj._bsontype && obj._bsontype === 'ObjectId')) {
            const stringValue = obj.toString();
            console.log(`[MongoDBStorage] Converting ObjectId ${obj} to string: ${stringValue}`);
            return stringValue;
        }
        
        // Handle arrays
        if (Array.isArray(obj)) {
            console.log(`[MongoDBStorage] Converting ObjectIds in array of length: ${obj.length}`);
            return obj.map(item => this.convertObjectIdsToStrings(item));
        }
        
        // Handle plain objects
        const converted: any = {};
        for (const [key, value] of Object.entries(obj)) {
            // Skip MongoDB's _id field entirely
            if (key === '_id') {
                console.log(`[MongoDBStorage] Skipping MongoDB _id field with value:`, value);
                continue;
            }
            converted[key] = this.convertObjectIdsToStrings(value);
        }
        
        return converted;
    }
    
    /**
     * Create a new MongoDB storage instance
     * @param name - Database name
     * @param schemas - Collection schemas
     * @param options - Storage options including MongoDB connection details
     * @returns A new Instance of MongoDB storage
     * @public
     */ 
    static async create<SchemasCreate extends SchemaTypeRecord>(
        name: string,
        schemas: SchemasCreate,
        options: MongoDBStorageOptions = {}
    ) {
        console.log(`[MongoDBStorage] Creating new instance with name: ${name}, schemas:`, Object.keys(schemas), 'options:', options);
        const db = new MongoDBStorage<SchemasCreate>(name, schemas, options);
        console.log(`[MongoDBStorage] Instance created successfully`);
        return db;
    }
    
    /** Start the database connection */
    async start(): Promise<void> {
        console.log(`[MongoDBStorage] Starting database connection...`);
        const { MongoClient } = await import("mongodb");
        
        const url = this.mongoConfig.url || process.env.MONGODB_URL || "mongodb://localhost:27017";
        const dbName = this.mongoConfig.dbName || (this as any).name;
        
        console.log(`[MongoDBStorage] Connecting to MongoDB at: ${url}, database: ${dbName}`);
        console.log(`[MongoDBStorage] Using mongo options:`, this.mongoConfig.mongoOptions);
        
        try {
            this.client = new MongoClient(url, this.mongoConfig.mongoOptions || {});
            await this.client.connect();
            this.db = this.client.db(dbName);
            console.log(`[MongoDBStorage] Successfully connected to MongoDB database: ${dbName}`);
        } catch (error) {
            console.error(`[MongoDBStorage] Failed to connect to MongoDB:`, error);
            throw error;
        }
    }
    
    /** Close the database connection */
    async close(): Promise<void> {
        console.log(`[MongoDBStorage] Closing database connection...`);
        if (this.client) {
            try {
                await this.client.close();
                this.client = undefined;
                this.db = undefined;
                console.log(`[MongoDBStorage] Database connection closed successfully`);
            } catch (error) {
                console.error(`[MongoDBStorage] Error closing database connection:`, error);
                throw error;
            }
        } else {
            console.log(`[MongoDBStorage] No active connection to close`);
        }
    }
    
    /** Get MongoDB collection for a given collection name */
    private getCollection(collectionName: string): Collection {
        if (!this.db) {
            console.error(`[MongoDBStorage] Database not started when trying to get collection: ${collectionName}`);
            throw new Error("Database not started. Call start() first.");
        }
        console.log(`[MongoDBStorage] Getting collection: ${collectionName}`);
        const collection = this.db.collection(collectionName);
        console.log(`[MongoDBStorage] Collection obtained: ${collectionName}`);
        return collection;
    }
    
    /** Find a document by its ID */
    async findDocumentById(
        collectionName: keyof T,
        id: string
    ): Promise<Doc<T[keyof T]> | null> {
        console.log(`[MongoDBStorage] Finding document by ID in collection ${String(collectionName)}, ID: ${id}`);
        
        try {
            const collection = this.getCollection(String(collectionName));
            const schema = this.getSchema(String(collectionName));
            const primaryKey = schema.primaryKey;
            
            console.log(`[MongoDBStorage] Using primary key: ${primaryKey} for collection: ${String(collectionName)}`);
            
            const query = { [primaryKey]: id };
            console.log(`[MongoDBStorage] MongoDB query:`, query);
            
            const doc = await collection.findOne(query);
            console.log(`[MongoDBStorage] Raw document found:`, doc);
            
            if (!doc) {
                console.log(`[MongoDBStorage] No document found with ID: ${id}`);
                return null;
            }
            
            const converted = this.convertObjectIdsToStrings(doc) as Doc<T[keyof T]>;
            console.log(`[MongoDBStorage] Converted document:`, converted);
            return converted;
        } catch (error) {
            console.error(`[MongoDBStorage] Error finding document by ID:`, error);
            throw error;
        }
    }
    
    /** Write an operation (insert, update, delete) */
    async write(op: Operation<T[keyof T]>): Promise<Doc<T[keyof T]>> {
        console.log(`[MongoDBStorage] Writing operation:`, {
            opType: op.opType,
            collection: String(op.collection),
            data: op.data
        });
        
        try {
            const collectionName = String(op.collection);
            const collection = this.getCollection(collectionName);
            const {primaryKey} = this.getSchema(collectionName);
            
            console.log(`[MongoDBStorage] Using primary key: ${primaryKey} for operation`);

            switch (op.opType) {
                case OpType.CREATE: {
                    const id = primaryKey in op.data ? (op.data as any)[primaryKey] : null;
                    console.log(`[MongoDBStorage] CREATE operation - ID: ${id}`);
                    
                    if (!id) {
                        console.error(`[MongoDBStorage] Document ID is required for CREATE operation`);
                        throw new Error("Document ID is required");
                    }
                    
                    // Check if document already exists
                    const existQuery = { [primaryKey]: id };
                    console.log(`[MongoDBStorage] Checking if document exists with query:`, existQuery);
                    const existing = await collection.findOne(existQuery);
                    console.log(`[MongoDBStorage] Existing document check result:`, existing);
                    
                    if (existing) {
                        console.error(`[MongoDBStorage] Document already exists with ${primaryKey}: ${id}`);
                        throw new Error(`Document with ${primaryKey} '${id}' already exists`);
                    }
                    
                    // Prevent MongoDB from auto-generating _id by explicitly setting it to null
                    const docToInsert = { ...op.data as any };
                    console.log(`[MongoDBStorage] Inserting document:`, docToInsert);
                    
                    const insertResult = await collection.insertOne(docToInsert);
                    console.log(`[MongoDBStorage] Insert result:`, insertResult);
                    
                    const result = this.convertObjectIdsToStrings(op.data);
                    console.log(`[MongoDBStorage] CREATE operation completed, returning:`, result);
                    return result;
                }
                case OpType.UPDATE: {
                    const id = primaryKey in op.data ? (op.data as any)[primaryKey] : null;
                    console.log(`[MongoDBStorage] UPDATE operation - ID: ${id}`);
                    
                    if (!id) {
                        console.error(`[MongoDBStorage] Document ID is required for UPDATE operation`);
                        throw new Error("Document ID is required");
                    }
                    
                    const updateQuery = { [primaryKey]: id };
                    const updateData = { $set: op.data };
                    console.log(`[MongoDBStorage] Update query:`, updateQuery);
                    console.log(`[MongoDBStorage] Update data:`, updateData);
                    
                    const result = await collection.updateOne(updateQuery, updateData);
                    console.log(`[MongoDBStorage] Update result:`, result);
                    
                    if (result.matchedCount === 0) {
                        console.error(`[MongoDBStorage] No document found to update with ${primaryKey}: ${id}`);
                        throw new Error("Document ID not found");
                    }
                    
                    const convertedResult = this.convertObjectIdsToStrings(op.data);
                    console.log(`[MongoDBStorage] UPDATE operation completed, returning:`, convertedResult);
                    return convertedResult;
                }
                case OpType.DELETE: {
                    const id = op.data;
                    console.log(`[MongoDBStorage] DELETE operation - ID: ${id}`);
                    
                    const deleteQuery = { [primaryKey]: id };
                    console.log(`[MongoDBStorage] Delete query:`, deleteQuery);
                    
                    const result = await collection.deleteOne(deleteQuery);
                    console.log(`[MongoDBStorage] Delete result:`, result);
                    
                    if (result.deletedCount === 0) {
                        console.error(`[MongoDBStorage] No document found to delete with ${primaryKey}: ${id}`);
                        throw new Error("Document ID not found");
                    }
                    
                    const convertedResult = this.convertObjectIdsToStrings(op.data);
                    console.log(`[MongoDBStorage] DELETE operation completed, returning:`, convertedResult);
                    return convertedResult;
                }
                default:
                    console.error(`[MongoDBStorage] Unknown operation type: ${op.opType}`);
                    throw new Error(`Unknown operation type: ${op.opType}`);
            }
        } catch (error) {
            console.error(`[MongoDBStorage] Error in write operation:`, error);
            throw error;
        }
    }
    
    /** Convert RIDB query to MongoDB filter */
    private convertQueryToMongoFilter(query: QueryType<T[keyof T]>, schema: any): any {
        console.log(`[MongoDBStorage] Converting query to MongoDB filter:`, query);
        
        if (!query || typeof query !== 'object') {
            console.log(`[MongoDBStorage] Empty or invalid query, returning empty filter`);
            return {};
        }
        
        const filter: any = {};
        
        for (const [key, value] of Object.entries(query)) {
            console.log(`[MongoDBStorage] Processing query field: ${key} =`, value);
            
            if (value === null || value === undefined) {
                filter[key] = { $exists: false };
                console.log(`[MongoDBStorage] Converted null/undefined to $exists: false for ${key}`);
            } else if (typeof value === 'object' && value !== null) {
                // Convert any ObjectId values to strings in query operators
                if ('$in' in value) {
                    filter[key] = { $in: this.convertObjectIdsToStrings(value.$in) };
                    console.log(`[MongoDBStorage] Converted $in operator for ${key}:`, filter[key]);
                } else if ('$nin' in value) {
                    filter[key] = { $nin: this.convertObjectIdsToStrings(value.$nin) };
                    console.log(`[MongoDBStorage] Converted $nin operator for ${key}:`, filter[key]);
                } else if ('$gt' in value) {
                    filter[key] = { $gt: this.convertObjectIdsToStrings(value.$gt) };
                    console.log(`[MongoDBStorage] Converted $gt operator for ${key}:`, filter[key]);
                } else if ('$gte' in value) {
                    filter[key] = { $gte: this.convertObjectIdsToStrings(value.$gte) };
                    console.log(`[MongoDBStorage] Converted $gte operator for ${key}:`, filter[key]);
                } else if ('$lt' in value) {
                    filter[key] = { $lt: this.convertObjectIdsToStrings(value.$lt) };
                    console.log(`[MongoDBStorage] Converted $lt operator for ${key}:`, filter[key]);
                } else if ('$lte' in value) {
                    filter[key] = { $lte: this.convertObjectIdsToStrings(value.$lte) };
                    console.log(`[MongoDBStorage] Converted $lte operator for ${key}:`, filter[key]);
                } else if ('$ne' in value) {
                    filter[key] = { $ne: this.convertObjectIdsToStrings(value.$ne) };
                    console.log(`[MongoDBStorage] Converted $ne operator for ${key}:`, filter[key]);
                } else if ('$regex' in value) {
                    filter[key] = { $regex: value.$regex };
                    console.log(`[MongoDBStorage] Converted $regex operator for ${key}:`, filter[key]);
                } else {
                    // For complex objects, convert ObjectIds and use exact match
                    filter[key] = this.convertObjectIdsToStrings(value);
                    console.log(`[MongoDBStorage] Converted complex object for ${key}:`, filter[key]);
                }
            } else {
                filter[key] = this.convertObjectIdsToStrings(value);
                console.log(`[MongoDBStorage] Converted simple value for ${key}:`, filter[key]);
            }
        }
        
        console.log(`[MongoDBStorage] Final MongoDB filter:`, filter);
        return filter;
    }
    
    /** Count documents matching a query (supports offset & limit) */
    async count(
        collectionName: keyof T,
        query: QueryType<T[keyof T]>,
        options?: QueryOptions
    ): Promise<number> {
        console.log(`[MongoDBStorage] Counting documents in collection ${String(collectionName)} with query:`, query, 'options:', options);
        
        try {
            const collection = this.getCollection(String(collectionName));
            const schema = this.getSchema(String(collectionName));
            
            // For MongoDB, we use the native query system instead of the RIDB Query class
            // to get better performance, but we could also use Query class for consistency
            const filter = this.convertQueryToMongoFilter(query, schema);
            
            let countOptions: any = {};
            if (options?.offset) {
                countOptions.skip = options.offset;
                console.log(`[MongoDBStorage] Adding skip to count query: ${options.offset}`);
            }
            if (options?.limit) {
                countOptions.limit = options.limit;
                console.log(`[MongoDBStorage] Adding limit to count query: ${options.limit}`);
            }
            
            console.log(`[MongoDBStorage] Executing count with filter:`, filter, 'options:', countOptions);
            const count = await collection.countDocuments(filter, Object.keys(countOptions).length > 0 ? countOptions : undefined);
            console.log(`[MongoDBStorage] Count result: ${count}`);
            
            return count;
        } catch (error) {
            console.error(`[MongoDBStorage] Error in count operation:`, error);
            throw error;
        }
    }
    
    /** Find documents matching a query with pagination */
    async find(
        collectionName: keyof T,
        query: QueryType<T[keyof T]>,
        options?: QueryOptions
    ): Promise<Doc<T[keyof T]>[]> {
        console.log(`[MongoDBStorage] Finding documents in collection ${String(collectionName)} with query:`, query, 'options:', options);
        
        try {
            const collection = this.getCollection(String(collectionName));
            const schema = this.getSchema(String(collectionName));
            
            // For MongoDB, we use the native query system for better performance
            const filter = this.convertQueryToMongoFilter(query, schema);
            
            let findQuery = collection.find(filter);
            console.log(`[MongoDBStorage] Initial find query created with filter:`, filter);
            
            // Apply offset and limit if provided
            if (options?.offset) {
                findQuery = findQuery.skip(options.offset);
                console.log(`[MongoDBStorage] Applied skip: ${options.offset}`);
            }
            
            if (options?.limit) {
                findQuery = findQuery.limit(options.limit);
                console.log(`[MongoDBStorage] Applied limit: ${options.limit}`);
            }
            
            console.log(`[MongoDBStorage] Executing find query...`);
            const docs = await findQuery.toArray();
            console.log(`[MongoDBStorage] Found ${docs.length} raw documents:`, docs);
            
            // Convert ObjectIds to strings and remove _id fields from all documents
            const convertedDocs = docs.map(doc => this.convertObjectIdsToStrings(doc)) as Doc<T[keyof T]>[];
            console.log(`[MongoDBStorage] Converted ${convertedDocs.length} documents:`, convertedDocs);
            
            return convertedDocs;
        } catch (error) {
            console.error(`[MongoDBStorage] Error in find operation:`, error);
            throw error;
        }
    }
}

/**
 * Create a MongoDB storage instance
 * @public
 * @returns A factory function that creates MongoDB storage instances
 */
export default async function createMongoDB<T extends SchemaTypeRecord>(): Promise<typeof BaseStorage<T>> {
    console.log(`[MongoDBStorage] Creating MongoDB factory function...`);
    
    try {
        const {BaseStorage: base} = await WasmInternal();
        console.log(`[MongoDBStorage] WasmInternal loaded successfully`);
        
        // We need to extend the actual BaseStorage from WasmInternal
        Object.setPrototypeOf(MongoDBStorage.prototype, base.prototype);
        console.log(`[MongoDBStorage] Prototype chain set up successfully`);
        
        // Return the static create method as the constructor
        console.log(`[MongoDBStorage] Factory function created successfully`);
        return MongoDBStorage as unknown as typeof BaseStorage<T>
    } catch (error) {
        console.error(`[MongoDBStorage] Error creating MongoDB factory:`, error);
        throw error;
    }
}
