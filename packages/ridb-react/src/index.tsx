import React, { createContext, useMemo, useContext, useState, useCallback } from 'react';
import { DBOptions, RIDB, StartOptions } from "@trust0/ridb";
import { SchemaTypeRecord } from '@trust0/ridb-core';

export type DatabaseState = 'disconnected' | 'loading' | 'loaded' | 'error';
type Context<T extends SchemaTypeRecord> = {
  db: RIDB<T>;
  state: DatabaseState;
  start: () => Promise<void>;
  setStartOptions: (options: StartOptions<T>) => void;
  stop: () => Promise<void>;
} | null


export const RIDBContext = createContext<Context<any>>(null);

export function useRIDB<T extends SchemaTypeRecord>() {
  const context = useContext<Context<T>>(RIDBContext);
  if (!context) {
    throw new Error('useRIDB must be used within a RIDB provider');
  }
  return context
}

export function RIDBDatabase<T extends SchemaTypeRecord>({ children, startOptions: initialStartOptions, ...props }:  {startOptions?: StartOptions<T>}  & {
  children?: React.ReactNode;
} & DBOptions<T>) {

  const dbInit = props as DBOptions<T>;
  const [startOptions, setStartOptions] = useState<StartOptions<T> | undefined>(initialStartOptions);
  const db = useMemo(() => new RIDB<T>(dbInit), []);
  const [state, setState] = useState<DatabaseState>('disconnected');

  const start = useCallback(async () => {
    if (startOptions === undefined) {
      setState('error')
      console.error('No start options provided');
      return;
    }

    setState('loading');
    await db.start(startOptions);
    setStartOptions(undefined);
    setState('loaded');
  }, [db]);

  const stop = useCallback(async () => {
    setState('disconnected');
    await db.close();
  }, [db]);


  const setStartOptionsFn = useCallback((options: StartOptions<T>) => {
    setStartOptions(options);
  }, [setStartOptions]);
  
  return (
    <RIDBContext.Provider value={{db, state, start, stop, setStartOptions: setStartOptionsFn}}>
      {children}
    </RIDBContext.Provider>
  );
}


