import React, { createContext, useMemo, useContext, useEffect } from 'react';
import { RIDB, SchemaTypeRecord, BasePlugin, MigrationsParameter, WasmInternal } from "@trust0/ridb";
await     WasmInternal();


type DatabaseProps<T extends SchemaTypeRecord> = {
  dbName: string;
  schemas: T;
  plugins?: Array<typeof BasePlugin>;
} & MigrationsParameter<T>;

const DatabaseContext = createContext<RIDB<any> | null>(null);

export type DatabaseComponentProps<T extends SchemaTypeRecord> = DatabaseProps<T> & {
  children?: React.ReactNode;
};

export function useDatabase<T extends SchemaTypeRecord>(): RIDB<T> {
  const context = useContext(DatabaseContext);
  if (!context) {
    throw new Error('useDatabase must be used within a Database provider');
  }
  useEffect(() => {
    WasmInternal()
  }, [WasmInternal]);
  return context as RIDB<T>;
}

export function Database<T extends SchemaTypeRecord>({ children, ...props }: DatabaseComponentProps<T>) {
  const dbInit = props as DatabaseProps<T>;
  const db = useMemo(() => new RIDB<T>(dbInit), [props]);
  return (
    <DatabaseContext.Provider value={db}>
      {children}
    </DatabaseContext.Provider>
  );
}


