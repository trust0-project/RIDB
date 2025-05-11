import { SchemaTypeRecord, BaseStorage, BasePlugin, MigrationsParameter } from "@trust0/ridb-core";

export type StorageClass<T extends SchemaTypeRecord> = {
    create: (
      name: string,
      schemas: T,
      options: any
    ) => Promise<BaseStorage<T>>;
  }
  
  export enum StorageType {
    InMemory = "InMemory",
    IndexDB = "IndexDB"
  }
  
  export type StartOptions<T extends SchemaTypeRecord> = {
    storageType?: StorageClass<T> | StorageType;
    password?: string;
    dbName?: string;
    [name: string]: any
  }
  
  /**
   * Options for the RIDB constructor.
   *
   * @typedef {DBOptions}
   * @template {SchemaTypeRecord} [T=SchemaTypeRecord] 
   */
  export type DBOptions<T extends SchemaTypeRecord = SchemaTypeRecord> = {
    /**
     * @deprecated Use the dbName option in the start method instead.
     */
    dbName?: string,
    schemas: T,
    plugins?: Array<typeof BasePlugin>,
    worker?: boolean
  } & MigrationsParameter<T>
  
  
  export type PendingRequests = Map<
    string,
    { resolve: (resp: any) => void; reject: (err: any) => void }
  >;
  