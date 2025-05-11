import React, { createContext, useMemo, useContext, useEffect, useState } from 'react';
import { DBOptions, RIDB, StartOptions, WasmInternal } from "@trust0/ridb";
import { SchemaTypeRecord } from '@trust0/ridb-core';

type RIDBProps<T extends SchemaTypeRecord> = DBOptions<T> & {startOptions?: StartOptions<T>};

// Create a context that includes both the database and loading state
type RIDBContextValue<T extends SchemaTypeRecord> = {
  db: RIDB<T> | null;
  isLoading: boolean;
  error: Error | null;
};

const RIDBContext = createContext<RIDBContextValue<any>>({
  db: null,
  isLoading: true,
  error: null
});

export type RIDBComponentProps<T extends SchemaTypeRecord> = RIDBProps<T> & {
  children?: React.ReactNode;
};

// Hook that returns the database and loading state
export function useRIDB<T extends SchemaTypeRecord>() {
  const context = useContext(RIDBContext);
  
  if (!context) {
    throw new Error('useRIDB must be used within a RIDB provider');
  }
  
  return context as RIDBContextValue<T>;
}

// Simplified hook for just getting the database instance when you know it's ready
export function useRIDBInstance<T extends SchemaTypeRecord>() {
  const { db, isLoading, error } = useRIDB<T>();
  
  if (isLoading) {
    throw new Error('Database is still loading');
  }
  
  if (error) {
    throw error;
  }
  
  if (!db) {
    throw new Error('Database is not available');
  }
  
  return db;
}

export function RIDBDatabase<T extends SchemaTypeRecord>({ children, ...props }: RIDBComponentProps<T>) {
  const dbInit = props as RIDBProps<T>;
  const [isLoading, setIsLoading] = useState(true);
  const [error, setError] = useState<Error | null>(null);
  const [db, setDb] = useState<RIDB<T> | null>(null);
  
  useEffect(() => {
    let mounted = true;
    const initializeDb = async () => {
      try {
        // Initialize WASM
        await WasmInternal();
        // Create database instance
        const dbInstance = new RIDB<T>(dbInit);
        await dbInstance.start(dbInit.startOptions);
        if (mounted) {
          setDb(dbInstance);
          setIsLoading(false);
        }
      } catch (err) {
        if (mounted) {
          setError(err instanceof Error ? err : new Error(String(err)));
          setIsLoading(false);
        }
      }
    };
    initializeDb();
    // Cleanup function
    return () => {
      mounted = false;
    };
  }, [JSON.stringify(dbInit)]);
  
  const contextValue = useMemo(() => ({
    db,
    isLoading,
    error
  }), [db, isLoading, error]);
  
  return (
    <RIDBContext.Provider value={contextValue}>
      {children}
    </RIDBContext.Provider>
  );
}


