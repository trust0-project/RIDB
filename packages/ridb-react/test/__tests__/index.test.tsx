import React from 'react'
import { render, waitFor, screen } from '@testing-library/react'
import { SchemaFieldType, StorageType } from '@trust0/ridb'
import { describe, expect, it, vi } from 'vitest'
import { RIDBDatabase, useRIDB } from '../../src'

const users = {
  version: 0 as const,
  primaryKey: 'id',
  type: SchemaFieldType.object,
  properties: {
      id: {
          type: SchemaFieldType.string,
          maxLength: 60
      },
      name: {
          type: SchemaFieldType.string
      },
      age: {
          type: SchemaFieldType.number
      }
  }
} as const

const schemas = {
  users: users
}
  
type DatabaseSchemas = typeof schemas;

const MyComponent: React.FC = () => {
    const { db, isLoading, error } = useRIDB<DatabaseSchemas>();
    const [isDbReady, setIsDbReady] = React.useState(false);

    React.useEffect(() => {
        const startDb = async () => {
            if (db) {
                await db.start();
                setIsDbReady(true);
            }
        };
        
        if (!isLoading && db) {
            startDb();
        }
    }, [db, isLoading]);

    if (error) {
        return <div data-testid="error">Error: {error.message}</div>;
    }

    if (isLoading) {
        return <div data-testid="loading">Database is initializing...</div>;
    }

    if (!db) {
        return <div data-testid="not-available">Database is not available</div>;
    }

    if (!isDbReady) {
        return <div data-testid="starting">Loading...</div>;
    }

    return (
        <div data-testid="ready-component"> <h1>My Component</h1> </div>
    );
};

// Component that simulates database operations
const MockDataComponent: React.FC = () => {
    const { db, isLoading, error } = useRIDB<DatabaseSchemas>();
    const [status, setStatus] = React.useState<string>('idle');

    React.useEffect(() => {
        const simulateDbOperations = async () => {
            if (db) {
                try {
                    await db.start();
                    setStatus('success');
                } catch (err) {
                    setStatus('error');
                }
            }
        };
        
        if (!isLoading && db) {
            setStatus('processing');
            simulateDbOperations();
        }
    }, [db, isLoading]);

    if (error) {
        return <div data-testid="error">Error: {error.message}</div>;
    }

    if (isLoading) {
        return <div data-testid="loading">Loading database...</div>;
    }

    if (status === 'idle') {
        return <div data-testid="idle">Waiting to start...</div>;
    }

    if (status === 'processing') {
        return <div data-testid="processing">Processing operations...</div>;
    }

    if (status === 'error') {
        return <div data-testid="op-error">Operation failed</div>;
    }

    return (
        <div data-testid="data-ready">
            <h1>Data Operations Complete</h1>
            <div data-testid="user-data">John Doe (Age: 30)</div>
        </div>
    );
};

describe('RIDB React Components', () => {
  // Basic rendering tests
  describe('MyComponent', () => {
    it('renders loading state initially', async () => {
      const { getByTestId, container } = render(
        <RIDBDatabase 
          dbName="testDB" 
          schemas={schemas} 
          startOptions={{
            storageType: StorageType.InMemory, 
            password: "testtrust0"
          }}
        >
          <MyComponent />
        </RIDBDatabase>
      );
      
      expect(getByTestId('loading')).toBeTruthy();
      expect(container).toMatchSnapshot('MyComponent-loading-state');
    });

    it('renders successfully when database is ready', async () => {
      const { container } = render(
        <RIDBDatabase 
          dbName="testDB" 
          schemas={schemas} 
          startOptions={{
            storageType: StorageType.InMemory, 
            password: "testtrust0"
          }}
        >
          <MyComponent />
        </RIDBDatabase>
      );
      
      // Wait for the component to fully load
      await waitFor(() => {
        expect(screen.getByTestId('ready-component')).toBeTruthy();
      }, { timeout: 3000 });
      
      expect(container).toMatchSnapshot('MyComponent-ready-state');
    });
  });
  
  // Data operations tests (using mock)
  describe('Data Operations', () => {
    it('shows correct states during data operations', async () => {
      const { container } = render(
        <RIDBDatabase 
          dbName="testDB" 
          schemas={schemas} 
          startOptions={{
            storageType: StorageType.InMemory, 
            password: "testtrust0"
          }}
        >
          <MockDataComponent />
        </RIDBDatabase>
      );
      
      // First should show loading
      expect(screen.getByTestId('loading')).toBeTruthy();
      expect(container).toMatchSnapshot('MockDataComponent-loading-state');
      
      // Eventually should show data is ready
      await waitFor(() => {
        expect(screen.getByTestId('data-ready')).toBeTruthy();
      }, { timeout: 3000 });
      
      // Should display mocked user data
      expect(screen.getByTestId('user-data').textContent).toContain('John Doe');
      expect(container).toMatchSnapshot('MockDataComponent-data-ready-state');
    });
  });
  
})

