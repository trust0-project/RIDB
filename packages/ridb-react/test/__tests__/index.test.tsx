import React from 'react'
import { render, waitFor } from '@testing-library/react'
import { SchemaFieldType } from '@trust0/ridb'
import { describe, expect, it } from 'vitest'
import { RIDBDatabase, useRIDB } from '../..'

const users = {
  version: 0 as const,
  primaryKey: 'id',
  type: SchemaFieldType.object,
  properties: {
      id: {
          type: SchemaFieldType.string,
          maxLength: 60
      }
  }
} as const

const schemas = {
  users: users
}
  
type DatabaseSchemas = typeof schemas;


const MyComponent: React.FC = () => {
    const db = useRIDB<DatabaseSchemas>();
    const [isDbReady, setIsDbReady] = React.useState(false);

    React.useEffect(() => {
        const startDb = async () => {
            if (db) {
                await db.start();
                setIsDbReady(true);
            }
        };
        startDb();
    }, [db]);

    if (!db) {
        return <div>No database available</div>;
    }

    if (!isDbReady) {
        return <div>Loading...</div>;
    }

    return (
        <div> <h1>My Component</h1> </div>
    );
};

describe('MyComponent', () => {

  it('renders correctly while db is loading', async () => {
    const { asFragment, getByText } = render(
      <RIDBDatabase dbName="testDB" schemas={schemas}>
        <MyComponent />
      </RIDBDatabase>
    );
    expect(asFragment()).toMatchSnapshot();
  });

  it('renders correctly when db is ready', async () => {
    const { asFragment, getByText } = render(
      <RIDBDatabase dbName="testDB" schemas={schemas}>
        <MyComponent />
      </RIDBDatabase>
    );
    await waitFor(() => {
      const element = getByText('My Component');
      expect(element).toBeTruthy();
    });
    expect(asFragment()).toMatchSnapshot();
  }); 

 
})

