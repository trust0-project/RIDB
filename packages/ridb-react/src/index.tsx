import React, { createContext, useMemo, useContext, useState, useCallback } from 'react';
import { DBOptions, RIDB, StartOptions } from "@trust0/ridb";
import { SchemaTypeRecord } from '@trust0/ridb-core';

export type DatabaseState = 'disconnected' | 'loading' | 'loaded' | 'error';
type Context<T extends SchemaTypeRecord> = {
  db: RIDB<T>;
  state: DatabaseState;
  start: (options: StartOptions<T>) => Promise<void>;
  stop: () => Promise<void>;
} | null


const RIDBContext = createContext<Context<any>>(null);

export function useRIDB<T extends SchemaTypeRecord>() {
  const context = useContext<Context<T>>(RIDBContext);
  if (!context) {
    throw new Error('useRIDB must be used within a RIDB provider');
  }
  return context
}

export function RIDBDatabase<T extends SchemaTypeRecord>({ children, ...props }:  DBOptions<T> & {startOptions?: StartOptions<T>}  & {
  children?: React.ReactNode;
}) {
  const dbInit = props as DBOptions<T>;
  const db = useMemo(() => new RIDB<T>(dbInit), []);
  const start = useCallback(async (options: StartOptions<T>) => {
      setState('loading');
      await db.start(options);
      setState('loaded');
  }, [db]);
  const stop = useCallback(async () => {
    setState('disconnected');
    await db.close();
  }, [db]);
  const [state, setState] = useState<DatabaseState>('disconnected');
  return (
    <RIDBContext.Provider value={{db, state, start, stop}}>
      {children}
    </RIDBContext.Provider>
  );
}


