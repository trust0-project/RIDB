/**
 * @packageDocumentation
 *
 * <p align="center">
 *   <img src="https://cdn.jsdelivr.net/gh/trust0-project/ridblatest/docs/logo.svg" alt="JavaScript Database" />
 *   <br />
 *   <br />
 *   <h3 align="center">A secure light-weight and dependency free database wrapper for the web.</h3>
 * </p>
 *
 *
 * <p align="center">
 *     <a href="https://github.com/trust0-project/RIDB/releases"><img src="https://img.shields.io/github/v/release/trust0-project/ridb?color=%23ff00a0&include_prereleases&label=version&sort=semver&style=flat-square"></a>
 *     &nbsp;
 *     <a href="#"><img src="https://img.shields.io/npm/types/@trust0/ridb?style=flat-square"></a>
 *     &nbsp;
 *     <a href="https://raw.githubusercontent.com/trust0-project/RIDB/refs/heads/main/LICENSE"><img src="https://img.shields.io/github/license/trust0-project/ridb?style=flat-square"></a>
 *     &nbsp;
 *     <a href="https://www.npmjs.com/package/@trust0/ridb"><img src="https://img.shields.io/npm/dm/@trust0/ridb?color=c63a3b&style=flat-square"></a>
 * </p>
 * 
 * # SDK Rerefence
 */
import type { 
    SchemaTypeRecord, 
    Doc, 
    Operation,
    QueryType,
 } from "@trust0/ridb-wasm";
import { BaseStorage, OpType, Query} from "@trust0/ridb-wasm";
import type { ClassicLevel } from "classic-level";
type Level = ClassicLevel<string, string>
export class LevelDB<T extends SchemaTypeRecord> extends BaseStorage<T> {
    private db: Level;
    static async create<SchemasCreate extends SchemaTypeRecord>(
        name: string,
        schemas: SchemasCreate,
        options: any
    ): Promise<BaseStorage<SchemasCreate>> {
        const levelImport = await import("classic-level");
        const level = levelImport.ClassicLevel;
        return new LevelDB(level, name, schemas, options);
    }
    constructor(Level: typeof ClassicLevel, name: string, schemas: T, options: any) {
        super(name, schemas, options);
        this.db = new Level(name);
    }
    /** Start the database */
    async start(): Promise<void> {
        await this.db
    }
    /** Close the database */
    async close(): Promise<void> {
        await this.db.close(); // Close the database
    }
    /** Find a document by its ID */
    async findDocumentById(
        collectionName: keyof T,
        id: string
    ): Promise<Doc<T[keyof T]> | null> {
        const key = `${String(collectionName)}:${id}`;
        try {
            const value = await this.db.get(key);
            const doc = JSON.parse(value);
            return doc;
        } catch (err: any) {
            if (err.notFound) {
                return null;
            } else {
                throw err;
            }
        }
    }
    /** Write an operation (insert, update, delete) */
    async write(op: Operation<T[keyof T]>): Promise<Doc<T[keyof T]>> {
        const collectionName = String(op.collection);
        const id = "id"  in op.data ? op.data.id : null;
        if (!id) {
            throw new Error("Document ID is required");
        }
        const key = `${collectionName}:${id}`;
        switch (op.opType) {
            case OpType.CREATE:
            case OpType.UPDATE: {
                const value = JSON.stringify(op.data);
                await this.db.put(key, value);
                break;
            }
            case OpType.DELETE: {
                await this.db.del(key);
                break;
            }
            default:
                throw new Error(`Unknown operation type: ${op.opType}`);
        }
        return op.data;
    }
    /** Count documents matching a query */
    async count(
        collectionName: keyof T,
        query: QueryType<T[keyof T]>
    ): Promise<number> {
        const collectionPrefix = `${String(collectionName)}:`;
        const schema = this.getSchema(String(collectionName));
        const queryInstance = new Query(query, schema);
        let count = 0;
        for await (const [key, value] of this.db.iterator({
            gte: collectionPrefix,
            lt: `${collectionPrefix}\xFF`,
        })) {
            const doc = JSON.parse(value);
            if (this.matchesQuery(doc, queryInstance)) {
                count++;
            }
        }
        return count;
    }
    /** Find documents matching a query */
    async find(
        collectionName: keyof T,
        query: QueryType<T[keyof T]>
    ): Promise<Doc<T[keyof T]>[]> {
        const collectionPrefix = `${String(collectionName)}:`;
        const schema = this.getSchema(String(collectionName));
        const queryInstance = new Query(query, schema);
        const docs: Doc<T[keyof T]>[] = [];
        for await (const [key, value] of this.db.iterator({
            gte: collectionPrefix,
            lt: `${collectionPrefix}\xFF`,
        })) {
            const doc = JSON.parse(value);
            if (this.matchesQuery(doc, queryInstance)) {
                docs.push(doc);
            }
        }
        return docs;
    }
    matchesQuery(doc: Doc<T[keyof T]>, query: Query<any>): boolean {
        return this.core.matchesQuery(doc, query);
    }
}