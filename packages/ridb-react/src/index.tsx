import React, { createContext, useMemo, useContext, useEffect } from 'react';
import { RIDB,  WasmInternal } from "@trust0/ridb";
import { BasePlugin, MigrationsParameter } from '@trust0/ridb-core';
import { SchemaTypeRecord } from '@trust0/ridb-core';

await WasmInternal();

type RIDBProps<T extends SchemaTypeRecord> = {
  dbName: string;
  schemas: T;
  plugins?: Array<typeof BasePlugin>;
} & MigrationsParameter<T>;

const RIDBContext = createContext<RIDB<any> | null>(null);

export type RIDBComponentProps<T extends SchemaTypeRecord> = RIDBProps<T> & {
  children?: React.ReactNode;
};

export function useRIDB<T extends SchemaTypeRecord>(): RIDB<T> {
  const context = useContext(RIDBContext);
  if (!context) {
    throw new Error('useRIDB must be used within a RIDB provider');
  }
  return context as RIDB<T>;
}

export function RIDBDatabase<T extends SchemaTypeRecord>({ children, ...props }: RIDBComponentProps<T>) {
  const dbInit = props as RIDBProps<T>;
  const db = useMemo(() => new RIDB<T>(dbInit), [props]);
  return (
    <RIDBContext.Provider value={db}>
      {children}
    </RIDBContext.Provider>
  );
}


