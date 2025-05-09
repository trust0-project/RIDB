/* tslint:disable */
/* eslint-disable */
/**
*/
declare function main_js(): void;
/**
* @returns {boolean}
*/
declare function is_debug_mode(): boolean;
/**
* Handler for `console.log` invocations.
*
* If a test is currently running it takes the `args` array and stringifies
* it and appends it to the current output of the test. Otherwise it passes
* the arguments to the original `console.log` function, psased as
* `original`.
* @param {Array<any>} args
*/
declare function __wbgtest_console_log(args: Array<any>): void;
/**
* Handler for `console.debug` invocations. See above.
* @param {Array<any>} args
*/
declare function __wbgtest_console_debug(args: Array<any>): void;
/**
* Handler for `console.info` invocations. See above.
* @param {Array<any>} args
*/
declare function __wbgtest_console_info(args: Array<any>): void;
/**
* Handler for `console.warn` invocations. See above.
* @param {Array<any>} args
*/
declare function __wbgtest_console_warn(args: Array<any>): void;
/**
* Handler for `console.error` invocations. See above.
* @param {Array<any>} args
*/
declare function __wbgtest_console_error(args: Array<any>): void;
/**
* Represents the type of operation to be performed on the collection.
*/
declare enum OpType {
/**
* Create operation.
*/
  CREATE = 0,
/**
* Update operation.
*/
  UPDATE = 1,
/**
* Delete operation.
*/
  DELETE = 2,
/**
* Query Operation.
*/
  QUERY = 3,
/**
* Count Operation.
*/
  COUNT = 4,
}
/**
*/
declare enum Errors {
  Error = 0,
  HookError = 1,
  QueryError = 2,
  SerializationError = 3,
  ValidationError = 4,
  AuthenticationError = 5,
}

/**
 * Represents a property within a schema, including various constraints and nested properties.
 */
declare class Property {
    /**
     * The type of the property.
     */
    readonly type: string;

    /**
     * The version of the property, if applicable.
     */
    readonly version?: number;

    /**
     * The primary key of the property, if applicable.
     */
    readonly primaryKey?: string;

    /**
     * An optional array of nested properties for array-type properties.
     */
    readonly items?: Property;

    /**
     * The maximum number of items for array-type properties, if applicable.
     */
    readonly maxItems?: number;

    /**
     * The minimum number of items for array-type properties, if applicable.
     */
    readonly minItems?: number;

    /**
     * The maximum length for string-type properties, if applicable.
     */
    readonly maxLength?: number;

    /**
     * The minimum length for string-type properties, if applicable.
     */
    readonly minLength?: number;

    /**
     * An optional array of required fields for object-type properties.
     */
    readonly required?: boolean;

    /**
     * An optional default value for the property.
     */
    readonly default?: any;

    /**
     * An optional map of nested properties for object-type properties.
     */
    readonly properties?: {
        [name: string]: Property;
    };
}



type Operators<T> = {
    $gte?: number,
    $gt?: number
    $lt?: number,
    $lte?: number,
    $eq?: T,
    $ne?: T
};

type InOperator<T> = {  $in?: T[] };
type NInOperator<T> = {  $nin?: T[] };

type OperatorOrType<T> = T extends number ? 
    T | Operators<T> | InOperator<T> | NInOperator<T> : 
    T | InOperator<T> | NInOperator<T>;

type LogicalOperators<T extends SchemaType> = {
    $and?: Partial<QueryType<T>>[];
    $or?: Partial<QueryType<T>>[];
};

type QueryType<T extends SchemaType> = Partial<{
    [K in keyof T['properties']]: OperatorOrType<
        ExtractType<
            T['properties'][K]['type']
        >
    >
}> & LogicalOperators<T> | LogicalOperators<T>[];

declare class Query<T extends SchemaType> {
    constructor(query: QueryType<T>, schema:Schema<T>);
    readonly query: QueryType<T>;
}



/**
 * Represents an IndexDB storage system extending the base storage functionality.
 *
 * @template T - The schema type.
 */
declare class IndexDB<T extends SchemaTypeRecord> extends BaseStorage<T> {
    /**
     * Frees the resources used by the in-memory storage.
     */
    free(): void;

    static create<SchemasCreate extends SchemaTypeRecord>(
        dbName: string,
        schemas: SchemasCreate,
    ): Promise<
        IndexDB<
            SchemasCreate
        >
    >;
}



type InternalsRecord = {
    [name: string]: BaseStorage<SchemaTypeRecord>
};
/**
 * ExtractType is a utility type that maps a string representing a basic data type to the actual TypeScript type.
 *
 * @template T - A string literal type representing the basic data type ('string', 'number', 'boolean', 'object', 'array').
 *
 * @example
 * type StringType = ExtractType<'string'>; // StringType is string
 * type NumberType = ExtractType<'number'>; // NumberType is number
 * type BooleanType = ExtractType<'boolean'>; // BooleanType is boolean
 * type ObjectType = ExtractType<'object'>; // ObjectType is object
 * type ArrayType = ExtractType<'array'>; // ArrayType is Array<any>
 */
type ExtractType<T extends string> = 
  T extends "string" ? string : 
  T extends "number" ? number : 
  T extends "boolean" ? boolean : 
  T extends "object" ? object : 
  T extends "array" ? any[] : 
  never;

type IsOptional<T> = T extends { required: false } ? true :
  T extends { default: any } ? true : false;

/**
 * Doc is a utility type that transforms a schema type into a document type where each property is mapped to its extracted type.
 *
 * @template T - A schema type with a 'properties' field where each property's type is represented as a string.
 *
 * type Document = Doc<Schema>; // Document is { name: string; age: number; }
 */
type Doc<T extends SchemaType> = {
  [K in keyof T["properties"] as IsOptional<T["properties"][K]> extends true ? K : never]?: 
    ExtractType<T["properties"][K]["type"]>
} & {
  [K in keyof T["properties"] as IsOptional<T["properties"][K]> extends false ? K : never]: 
    ExtractType<T["properties"][K]["type"]>
} & {
  __version?: number;
  createdAt?: number;
  updatedAt?: number;
};

type QueryOptions = {
    limit?: number;
    offset?: number;
}

/**
 * Collection is a class that represents a collection of documents in a database.
 * @template T - A schema type defining the structure of the documents in the collection.
 */
declare class Collection<T extends SchemaType> {
	/**
	 * Finds all documents in the collection.
	 *
	 * @returns A promise that resolves to an array of documents.
	 */
	find(query: QueryType<T>, options?: QueryOptions): Promise<Doc<T>[]>;
	/**
	 * count all documents in the collection.
	 *
	 * @returns A promise that resolves to an array of documents.
	 */
	count(query: QueryType<T>, options?: QueryOptions): Promise<number>;
	/**
	 * Finds a single document in the collection by its ID.
	 *
	 * @param id - The ID of the document to find.
	 * @returns A promise that resolves to the found document.
	 */
	findById(id: string): Promise<Doc<T>>;
	/**
	 * Updates a document in the collection by its ID.
	 *
	 * @param id - The ID of the document to update.
	 * @param document - A partial document containing the fields to update.
	 * @returns A promise that resolves when the update is complete.
	 */
	update(document: Partial<Doc<T>>): Promise<void>;
	/**
	 * Creates a new document in the collection.
	 *
	 * @param document - The document to create.
	 * @returns A promise that resolves to the created document.
	 */
	create(document: Doc<T>): Promise<Doc<T>>;
	/**
	 * Deletes a document in the collection by its ID.
	 *
	 * @param id - The ID of the document to delete.
	 * @returns A promise that resolves when the deletion is complete.
	 */
	delete(id: string): Promise<void>;
}




declare class CoreStorage {
    /**
    * @param {any} document
    * @param {Query} query
    * @returns {boolean}
    */
    matchesQuery(document: any, query: Query<any>): boolean;
    getPrimaryKeyTyped(value: any): string | number;
    getIndexes(schema: Schema<any>, op: Operation): string[];
}



/**
 * Represents an operation to be performed on a collection.
 *
 * @template T - The schema type of the collection.
 */
type Operation<T extends SchemaType = SchemaType> = {
    /**
     * The name of the collection on which the operation will be performed.
     */
    collection: string,

    /**
     * The type of operation to be performed (e.g., CREATE, UPDATE, DELETE).
     */
    opType: OpType,

    /**
     * The data involved in the operation, conforming to the schema type.
     */
    data: Doc<T>,

    primaryKeyField?: string,
    primaryKey?: string
}



/**
 * Represents an in-memory storage system extending the base storage functionality.
 *
 * @template T - The schema type.
 */
declare class InMemory<T extends SchemaTypeRecord> extends BaseStorage<T> {
    /**
     * Frees the resources used by the in-memory storage.
     */
    free(): void;

    static create<SchemasCreate extends SchemaTypeRecord>(
        dbName: string,
        schemas: SchemasCreate,
    ): Promise<
        InMemory<
            SchemasCreate
        >
    >;
}



type BaseStorageOptions =  {
    [name:string]:string | boolean | number
}

declare class BaseStorage<Schemas extends SchemaTypeRecord> extends StorageInternal<Schemas> {
    static create<SchemasCreate extends SchemaTypeRecord>(
        dbName: string,
        schemas: SchemasCreate,
        options?: BaseStorageOptions
    ): Promise<
        BaseStorage<
            SchemasCreate
        >
    >;
    constructor(
        dbName: string, 
        schemas: Schemas, 
        options?: BaseStorageOptions
    );
    readonly dbName: string;
    readonly schemas: Record<keyof Schemas, Schema<Schemas[keyof Schemas]>>;
    readonly options: BaseStorageOptions;
    readonly core: CoreStorage;
    start(): Promise<void>;
    close(): Promise<void>;
    count(colectionName: keyof Schemas, query: QueryType<Schemas[keyof Schemas]>, options?: QueryOptions): Promise<number>;
    findDocumentById(collectionName: keyof Schemas, id: string): Promise<Doc<Schemas[keyof Schemas]> | null>;
    find(collectionName: keyof Schemas, query: QueryType<Schemas[keyof Schemas]>, options?: QueryOptions): Promise<Doc<Schemas[keyof Schemas]>[]>;
    write(op: Operation<Schemas[keyof Schemas]>): Promise<Doc<Schemas[keyof Schemas]>>;
    getOption(name: string): string | boolean | number | undefined;
    getSchema(name: string): Schema<any>;
    //Call addIndexSchemas if you need extra indexing schemas for your database
    addIndexSchemas(): null
}



/**
 * Represents a database containing collections of documents.
 * RIDB extends from this class and is used to expose collections.
 * 
 * So if you specify:
 * ```typescript
 * const db = new RIDB(
 *     {
 *         schemas: {
 *             demo: {
 *                 version: 0,
 *                 primaryKey: 'id',
 *                 type: SchemaFieldType.object,
 *                 properties: {
 *                     id: {
 *                         type: SchemaFieldType.string,
 *                         maxLength: 60
 *                     }
 *                 }
 *             }
 *         } as const
 *     }
 * )
 * ```
 * 
 * The collection will be available as `db.collections.demo` and all the methods for the collection (find, count, findById, update, create, delete) will be available.
 *
 * @template T - A record of schema types.
 */
declare class Database<T extends SchemaTypeRecord> {

    /**
     * Creates a new `Database` instance with the provided schemas and storage module.
     *
     * @template TS - A record of schema types.
     * @param {TS} schemas - The schemas to use for the collections.
     * @param migrations
     * @param plugins
     * @param options
     * @param password
     * @returns {Promise<Database<TS>>} A promise that resolves to the created `Database` instance.
     */
    static create<TS extends SchemaTypeRecord>(
        db_name: string,
        schemas: TS,
        migrations: MigrationPathsForSchemas<TS> | MigrationPathsForSchema<TS[string]>,
        plugins:Array<typeof BasePlugin>,
        options: RIDBModule,
        password?:string,
        storage?: BaseStorage<TS>
    ): Promise<Database<TS>>;

    authenticate(password: string): Promise<boolean>;

    /**
     * The collections in the database.
     *
     * This is a read-only property where the key is the name of the collection and the value is a `Collection` instance.
     */
    readonly collections: {
        [name in keyof T]: Collection<Schema<T[name]>>
    }

    readonly started: boolean;

    /**
     * Starts the database.
     *
     * @returns {Promise<void>} A promise that resolves when the database is started.
     */
    start(): Promise<void>;

    /**
     * Closes the database.
     *
     * @returns {Promise<void>} A promise that resolves when the database is closed.
     */
    close(): Promise<void>;
}

/**
 * Represents a function type for creating storage with the provided schema type records.
 *
 * @template T - The schema type record.
 * @param {T} records - The schema type records.
 * @returns {Promise<InternalsRecord>} A promise that resolves to the created internals record.
 */
type CreateStorage = <T extends SchemaTypeRecord>(
    records: T
) => Promise<BaseStorage<T>>;

/**
 * Represents a storage module with a method for creating storage.
 */
type RIDBModule = {

    /**
     * Plugin constructors array
     */
    apply: (plugins:Array<typeof BasePlugin>) => Array<BasePlugin>;
};



/**
 * Represents a record of schema types, where each key is a string and the value is a `SchemaType`.
 */
type SchemaTypeRecord = {
    [name: string]: SchemaType
};

declare abstract class StorageInternal<Schemas extends SchemaTypeRecord> {
    constructor(
        name: string, 
        schemas: Schemas
    );
    abstract start(): Promise<void>;
    abstract close(): Promise<void>;
    abstract count(
        colectionName: keyof Schemas, 
        query: QueryType<Schemas[keyof Schemas]>,
        options?: QueryOptions
    ): Promise<number>;
    abstract findDocumentById(
        collectionName: keyof Schemas, 
        id: string
    ): Promise<Doc<Schemas[keyof Schemas]> | null>;
    abstract find(
        collectionName: keyof Schemas, 
        query: QueryType<Schemas[keyof Schemas]>,
        options?: QueryOptions
    ): Promise<Doc<Schemas[keyof Schemas]>[]>;
    abstract write(
        op: Operation<Schemas[keyof Schemas]>
    ): Promise<Doc<Schemas[keyof Schemas]>>;
}


/**
 * Represents the type definition for a schema.
 */
type SchemaType = {
    /**
     * The version of the schema.
     */
     version: number;

    /**
     * The primary key of the schema.
     */
     primaryKey: string;

    /**
     * The type of the schema.
     */
     type: string;
     indexes?:  string[];
     encrypted?:  string[];
    /**
     * The properties defined in the schema.
     */
     properties: {
        [name: string]: Property;
    };
};


/**
 * Represents a schema, including its definition and related methods.
 * You may be trying to build a storage, in any other can u won't need access tho this class.
 * Check this example 
 * 
 * ```typescript
 * class MyStorage extends <T extends SchemaTypeRecord> extends BaseStorage<T> {
 *  example() {
 *    const schema: Schema<any> = this.getSchema("mySchema")
 *  }
 * }
 * ```
 * You alwayswill have access to getSchema through the Storage class.
 * 
 * @template T - The schema type.
 */
declare class Schema<T extends SchemaType> {
    /**
     * The schema definition.
     */
    schema: Schema<T>;

    /**
     * Creates a new `Schema` instance from the provided definition.
     *
     * @template TS - The schema type.
     * @param {TS} defi, Debugnition - The schema definition.
     * @returns {Schema<TS>} The created `Schema` instance.
     */
    static create<TS extends SchemaType>(definition: TS): Schema<TS>;

    /**
     * The version of the schema.
     */
    readonly version: number;

    /**
     * The primary key of the schema.
     */
    readonly primaryKey: string;

    /**
     * The type of the schema.
     */
    readonly type: string;

    /**
     * An optional array of indexes.
     */
    /**
     * An optional array of indexes.
     */
    readonly indexes?: (Extract<keyof T, string>)[];

    /**
     * An optional array of encrypted fields.
     */
    readonly encrypted?: (Extract<keyof T, string>)[];

    /**
     * The properties defined in the schema.
     */
    readonly properties: {
        [K in keyof T['properties'] as T['properties'][K]['required'] extends false | (T['properties'][K]['default'] extends undefined ? true: false)  ? K : never]?: T['properties'][K];
    } & {
        [K in keyof T['properties'] as T['properties'][K]['required'] extends false ? never : K]: T['properties'][K];
    };
    /**
     * Converts the schema to a JSON representation.
     *
     * @returns {SchemaType} The JSON representation of the schema.
     */
    toJSON(): SchemaType;

    validate(document: Doc<Schema<T>>): boolean;
}



type EnumerateUpTo<
    N extends number,
    Acc extends number[] = []
> = Acc['length'] extends N ?
    Acc[number]:
    EnumerateUpTo<N, [...Acc, Acc['length']]> ;

type EnumerateFrom1To<
    N extends number
> = Exclude<EnumerateUpTo<N>,0> | (N extends 0 ? never : N);

type IsVersionGreaterThan0<
    V extends number
> = V extends 0 ? false : true;

type AnyVersionGreaterThan1<
    T extends Record<string, SchemaType>
> = true extends {
    [K in keyof T]: IsVersionGreaterThan0<T[K]['version']>;
} [keyof T] ? true : false;

type MigrationFunction<T extends SchemaType> = (doc: Doc <T> ) => Doc <T>

type MigrationPathsForSchema<
    T extends SchemaType
> = T['version'] extends 0 ? {}: // No migrations needed for version 1
    {
        [K in EnumerateFrom1To < T['version'] > ]: MigrationFunction<T> ;
    };

type MigrationPathsForSchemas<
    T extends SchemaTypeRecord
> = {
    [K in keyof T]: MigrationPathsForSchema<T[K]>;
};

type MigrationsParameter<
    T extends SchemaTypeRecord
> = AnyVersionGreaterThan1<T> extends true ?
    {
        migrations: MigrationPathsForSchemas<T>
    }:
    {
        migrations?: never
    };



type Hook = (
    schema: Schema<SchemaType>,
    migration: MigrationPathsForSchema<SchemaType>,
    doc: Doc<SchemaType>
) => Doc<SchemaType>

type BasePluginOptions = {
    docCreateHook?: Hook,
    docRecoverHook?: Hook
}

declare class BasePlugin implements BasePluginOptions {
     docCreateHook?:Hook;
     docRecoverHook?:Hook;
}


/**
*/
declare class RIDBError {
/**
** Return copy of self without private attributes.
*/
  toJSON(): Object;
/**
* Return stringified version of self.
*/
  toString(): string;
  free(): void;
/**
* @param {string} err_type
* @param {string} message
* @param {number} code
*/
  constructor(err_type: string, message: string, code: number);
/**
* @param {any} err
* @returns {RIDBError}
*/
  static from(err: any): RIDBError;
/**
* @param {string} err
* @param {number} code
* @returns {RIDBError}
*/
  static error(err: string, code: number): RIDBError;
/**
* @param {string} err
* @param {number} code
* @returns {RIDBError}
*/
  static query(err: string, code: number): RIDBError;
/**
* @param {string} err
* @param {number} code
* @returns {RIDBError}
*/
  static authentication(err: string, code: number): RIDBError;
/**
* @param {string} err
* @param {number} code
* @returns {RIDBError}
*/
  static serialisation(err: string, code: number): RIDBError;
/**
* @param {string} err
* @param {number} code
* @returns {RIDBError}
*/
  static validation(err: string, code: number): RIDBError;
/**
* @param {string} err
* @param {number} code
* @returns {RIDBError}
*/
  static hook(err: string, code: number): RIDBError;
/**
*/
  readonly code: any;
/**
*/
  readonly message: string;
/**
*/
  readonly type: string;
}
/**
* Runtime test harness support instantiated in JS.
*
* The node.js entry script instantiates a `Context` here which is used to
* drive test execution.
*/
declare class WasmBindgenTestContext {
  free(): void;
/**
* Creates a new context ready to run tests.
*
* A `Context` is the main structure through which test execution is
* coordinated, and this will collect output and results for all executed
* tests.
*/
  constructor();
/**
* Inform this context about runtime arguments passed to the test
* harness.
* @param {any[]} args
*/
  args(args: any[]): void;
/**
* Executes a list of tests, returning a promise representing their
* eventual completion.
*
* This is the main entry point for executing tests. All the tests passed
* in are the JS `Function` object that was plucked off the
* `WebAssembly.Instance` exports list.
*
* The promise returned resolves to either `true` if all tests passed or
* `false` if at least one test failed.
* @param {any[]} tests
* @returns {Promise<any>}
*/
  run(tests: any[]): Promise<any>;
}

type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_property_free: (a: number) => void;
  readonly property_is_valid: (a: number, b: number) => void;
  readonly property_type: (a: number) => number;
  readonly property_items: (a: number, b: number) => void;
  readonly property_maxItems: (a: number, b: number) => void;
  readonly property_minItems: (a: number, b: number) => void;
  readonly property_maxLength: (a: number, b: number) => void;
  readonly property_minLength: (a: number, b: number) => void;
  readonly property_properties: (a: number, b: number) => void;
  readonly __wbgt_test_property_creation_0: (a: number) => void;
  readonly __wbgt_test_property_validation_1: (a: number) => void;
  readonly __wbgt_test_invalid_property_2: (a: number) => void;
  readonly __wbg_query_free: (a: number) => void;
  readonly query_new: (a: number, b: number, c: number) => void;
  readonly query_query: (a: number, b: number) => void;
  readonly query_get_properties: (a: number, b: number) => void;
  readonly query_parse: (a: number, b: number) => void;
  readonly query_process_query: (a: number, b: number, c: number) => void;
  readonly query_get: (a: number, b: number, c: number, d: number) => void;
  readonly __wbgt_test_get_properties_simple_fields_6: (a: number) => void;
  readonly __wbgt_test_get_properties_with_operators_7: (a: number) => void;
  readonly __wbgt_test_get_properties_with_logical_operators_8: (a: number) => void;
  readonly __wbgt_test_get_properties_nested_operators_9: (a: number) => void;
  readonly __wbgt_test_get_properties_array_values_10: (a: number) => void;
  readonly __wbgt_test_get_properties_empty_query_11: (a: number) => void;
  readonly __wbgt_test_get_properties_deeply_nested_12: (a: number) => void;
  readonly __wbgt_test_get_properties_with_multiple_same_props_13: (a: number) => void;
  readonly __wbgt_test_get_properties_with_array_at_top_level_14: (a: number) => void;
  readonly __wbgt_test_query_parse_operator_wrong_type_15: (a: number) => void;
  readonly __wbgt_test_query_parse_in_operator_16: (a: number) => void;
  readonly __wbgt_test_query_parse_in_operator_wrong_type_17: (a: number) => void;
  readonly __wbgt_test_query_get_query_normalization_simple_attributes_18: (a: number) => void;
  readonly __wbgt_test_query_get_query_normalization_with_logical_operator_19: (a: number) => void;
  readonly __wbgt_test_query_get_query_normalization_nested_logical_operators_20: (a: number) => void;
  readonly __wbgt_test_query_get_query_normalization_only_logical_operator_21: (a: number) => void;
  readonly __wbgt_test_query_get_query_normalization_complex_mixed_22: (a: number) => void;
  readonly __wbgt_test_query_parse_empty_query_23: (a: number) => void;
  readonly __wbgt_test_query_parse_age_query_24: (a: number) => void;
  readonly __wbgt_test_query_parse_non_object_query_25: (a: number) => void;
  readonly __wbgt_test_query_parse_multiple_operators_26: (a: number) => void;
  readonly __wbgt_test_query_parse_invalid_in_operator_27: (a: number) => void;
  readonly __wbgt_test_query_parse_empty_logical_operators_28: (a: number) => void;
  readonly __wbgt_test_query_parse_nin_operator_29: (a: number) => void;
  readonly __wbgt_test_query_parse_nin_operator_wrong_type_30: (a: number) => void;
  readonly __wbgt_test_query_parse_eq_operator_31: (a: number) => void;
  readonly __wbgt_test_query_parse_eq_operator_wrong_type_32: (a: number) => void;
  readonly __wbgt_test_query_parse_ne_operator_33: (a: number) => void;
  readonly __wbgt_test_query_parse_ne_operator_wrong_type_34: (a: number) => void;
  readonly __wbg_ridberror_free: (a: number) => void;
  readonly ridberror_new: (a: number, b: number, c: number, d: number, e: number) => number;
  readonly ridberror_type: (a: number, b: number) => void;
  readonly ridberror_code: (a: number) => number;
  readonly ridberror_message: (a: number, b: number) => void;
  readonly ridberror_from: (a: number) => number;
  readonly ridberror_error: (a: number, b: number, c: number) => number;
  readonly ridberror_query: (a: number, b: number, c: number) => number;
  readonly ridberror_authentication: (a: number, b: number, c: number) => number;
  readonly ridberror_serialisation: (a: number, b: number, c: number) => number;
  readonly ridberror_validation: (a: number, b: number, c: number) => number;
  readonly ridberror_hook: (a: number, b: number, c: number) => number;
  readonly __wbg_indexdb_free: (a: number) => void;
  readonly indexdb_get_stores: (a: number, b: number) => void;
  readonly indexdb_get_store: (a: number, b: number, c: number, d: number) => void;
  readonly indexdb_create: (a: number, b: number, c: number) => number;
  readonly indexdb_write: (a: number, b: number) => number;
  readonly indexdb_find: (a: number, b: number, c: number, d: number, e: number) => number;
  readonly indexdb_findDocumentById: (a: number, b: number, c: number, d: number) => number;
  readonly indexdb_count: (a: number, b: number, c: number, d: number, e: number) => number;
  readonly indexdb_close: (a: number) => number;
  readonly indexdb_start: (a: number) => number;
  readonly main_js: () => void;
  readonly is_debug_mode: () => number;
  readonly __wbg_collection_free: (a: number) => void;
  readonly collection_name: (a: number, b: number) => void;
  readonly collection_schema: (a: number, b: number) => void;
  readonly collection_find: (a: number, b: number, c: number) => number;
  readonly collection_parse_query_options: (a: number, b: number, c: number) => void;
  readonly collection_count: (a: number, b: number, c: number) => number;
  readonly collection_findById: (a: number, b: number) => number;
  readonly collection_update: (a: number, b: number) => number;
  readonly collection_create: (a: number, b: number) => number;
  readonly collection_delete: (a: number, b: number) => number;
  readonly corestorage_new: () => number;
  readonly corestorage_getPrimaryKeyTyped: (a: number, b: number, c: number) => void;
  readonly corestorage_getIndexes: (a: number, b: number, c: number, d: number) => void;
  readonly corestorage_matchesQuery: (a: number, b: number, c: number, d: number) => void;
  readonly __wbg_operation_free: (a: number) => void;
  readonly operation_collection: (a: number, b: number) => void;
  readonly operation_opType: (a: number) => number;
  readonly operation_data: (a: number) => number;
  readonly operation_primaryKeyField: (a: number) => number;
  readonly operation_primaryKey: (a: number) => number;
  readonly operation_primaryKeyIndex: (a: number, b: number) => void;
  readonly __wbg_corestorage_free: (a: number) => void;
  readonly __wbg_inmemory_free: (a: number) => void;
  readonly inmemory_create: (a: number, b: number, c: number) => number;
  readonly inmemory_write: (a: number, b: number) => number;
  readonly inmemory_find: (a: number, b: number, c: number, d: number, e: number) => number;
  readonly inmemory_findDocumentById: (a: number, b: number, c: number, d: number) => number;
  readonly inmemory_count: (a: number, b: number, c: number, d: number, e: number) => number;
  readonly inmemory_close: (a: number) => number;
  readonly inmemory_start: (a: number) => number;
  readonly __wbg_basestorage_free: (a: number) => void;
  readonly basestorage_new: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly basestorage_addIndexSchemas: (a: number, b: number) => void;
  readonly basestorage_getOption: (a: number, b: number, c: number, d: number) => void;
  readonly basestorage_getSchema: (a: number, b: number, c: number, d: number) => void;
  readonly basestorage_core: (a: number, b: number) => void;
  readonly __wbg_database_free: (a: number) => void;
  readonly database_start: (a: number) => number;
  readonly database_close: (a: number) => number;
  readonly database_started: (a: number) => number;
  readonly database_authenticate: (a: number, b: number, c: number) => number;
  readonly database_collections: (a: number, b: number) => void;
  readonly database_create: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number) => number;
  readonly __wbg_queryoptions_free: (a: number) => void;
  readonly queryoptions_limit: (a: number, b: number) => void;
  readonly queryoptions_offset: (a: number, b: number) => void;
  readonly __wbg_schema_free: (a: number) => void;
  readonly schema_validate: (a: number, b: number, c: number) => void;
  readonly schema_is_valid: (a: number, b: number) => void;
  readonly schema_create: (a: number, b: number) => void;
  readonly schema_version: (a: number) => number;
  readonly schema_primaryKey: (a: number, b: number) => void;
  readonly schema_type: (a: number, b: number) => void;
  readonly schema_indexes: (a: number, b: number) => void;
  readonly schema_encrypted: (a: number, b: number) => void;
  readonly schema_properties: (a: number, b: number) => void;
  readonly __wbgt_test_schema_creation_3: (a: number) => void;
  readonly __wbgt_test_schema_validation_4: (a: number) => void;
  readonly __wbgt_test_invalid_schema_5: (a: number) => void;
  readonly __wbg_baseplugin_free: (a: number) => void;
  readonly baseplugin_new: (a: number, b: number, c: number) => void;
  readonly baseplugin_name: (a: number) => number;
  readonly baseplugin_get_doc_create_hook: (a: number) => number;
  readonly baseplugin_get_doc_recover_hook: (a: number) => number;
  readonly baseplugin_set_doc_create_hook: (a: number, b: number) => void;
  readonly baseplugin_set_doc_recover_hook: (a: number, b: number) => void;
  readonly __wbg_wasmbindgentestcontext_free: (a: number) => void;
  readonly wasmbindgentestcontext_new: () => number;
  readonly wasmbindgentestcontext_args: (a: number, b: number, c: number) => void;
  readonly wasmbindgentestcontext_run: (a: number, b: number, c: number) => number;
  readonly __wbgtest_console_log: (a: number) => void;
  readonly __wbgtest_console_debug: (a: number) => void;
  readonly __wbgtest_console_info: (a: number) => void;
  readonly __wbgtest_console_warn: (a: number) => void;
  readonly __wbgtest_console_error: (a: number) => void;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __wbindgen_export_2: WebAssembly.Table;
  readonly _dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h2ac93f1c2af0bde9: (a: number, b: number, c: number) => void;
  readonly _dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h9b3a888f37401eda: (a: number, b: number, c: number) => number;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
  readonly _dyn_core__ops__function__Fn__A_B_C___Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h4814c8631e98bfb6: (a: number, b: number, c: number, d: number, e: number, f: number) => void;
  readonly _dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h27f03e771f4393f9: (a: number, b: number, c: number) => void;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
  readonly __wbindgen_exn_store: (a: number) => void;
  readonly wasm_bindgen__convert__closures__invoke0_mut__h0f5b26648d09e4b0: (a: number, b: number) => void;
  readonly wasm_bindgen__convert__closures__invoke3_mut__h447a9f4e2970c0cf: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly wasm_bindgen__convert__closures__invoke2_mut__h36f949ecffe8079d: (a: number, b: number, c: number, d: number) => void;
  readonly __wbindgen_start: () => void;
}

type SyncInitInput = BufferSource | WebAssembly.Module;
/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {SyncInitInput} module
*
* @returns {InitOutput}
*/
declare function initSync(module: SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
declare function __wbg_init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;

export { type AnyVersionGreaterThan1, BasePlugin, BaseStorage, type BaseStorageOptions, Collection, CoreStorage, type CreateStorage, Database, type Doc, type EnumerateFrom1To, type EnumerateUpTo, Errors, type ExtractType, InMemory, type InOperator, IndexDB, type InitInput, type InitOutput, type InternalsRecord, type IsOptional, type IsVersionGreaterThan0, type LogicalOperators, type MigrationFunction, type MigrationPathsForSchema, type MigrationPathsForSchemas, type MigrationsParameter, type NInOperator, OpType, type Operation, type OperatorOrType, type Operators, Property, Query, type QueryOptions, type QueryType, RIDBError, type RIDBModule, Schema, type SchemaType, type SchemaTypeRecord, StorageInternal, type SyncInitInput, WasmBindgenTestContext, __wbgtest_console_debug, __wbgtest_console_error, __wbgtest_console_info, __wbgtest_console_log, __wbgtest_console_warn, __wbg_init as default, initSync, is_debug_mode, main_js };
