import React, { createContext, useMemo, useContext, useEffect, useState } from 'react';
import { DBOptions, RIDB, StartOptions, WasmInternal } from "@trust0/ridb";
import { SchemaTypeRecord } from '@trust0/ridb-core';

type RIDBProps<T extends SchemaTypeRecord> = DBOptions<T> & {startOptions?: StartOptions<T>};


const RIDBContext = createContext<RIDB<any> | null>(null);

export type RIDBComponentProps<T extends SchemaTypeRecord> = RIDBProps<T> & {
  children?: React.ReactNode;
};

// Hook that returns the database and loading state
export function useRIDB<T extends SchemaTypeRecord>() {
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


