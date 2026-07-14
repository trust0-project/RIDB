import type { MongoClient, Db, Collection, ObjectId } from "mongodb";

// In-memory storage for our mock
const mockDatabases = new Map<string, Map<string, any[]>>();

class MockCursor {
  private results: any[];
  private skipCount = 0;
  private limitCount = 0;
  private sortSpec: Record<string, 1 | -1> | null = null;

  constructor(results: any[]) {
    this.results = results;
  }

  sort(spec: Record<string, 1 | -1>) {
    this.sortSpec = spec;
    return this;
  }

  skip(count: number) {
    this.skipCount = count;
    return this;
  }

  limit(count: number) {
    this.limitCount = count;
    return this;
  }

  async toArray() {
    let results = [...this.results];

    // Sorting is applied before pagination, mirroring real MongoDB semantics.
    if (this.sortSpec) {
      const entries = Object.entries(this.sortSpec);
      results.sort((a, b) => {
        for (const [field, dir] of entries) {
          const av = a[field];
          const bv = b[field];
          let cmp = 0;
          if (av === bv) {
            cmp = 0;
          } else if (av === undefined || av === null) {
            cmp = 1; // missing values sort last (ascending)
          } else if (bv === undefined || bv === null) {
            cmp = -1;
          } else if (av < bv) {
            cmp = -1;
          } else if (av > bv) {
            cmp = 1;
          }
          if (cmp !== 0) {
            return dir === -1 ? -cmp : cmp;
          }
        }
        return 0;
      });
    }

    if (this.skipCount > 0) {
      results = results.slice(this.skipCount);
    }
    
    if (this.limitCount > 0) {
      results = results.slice(0, this.limitCount);
    }
    
    return results;
  }
}

class MockCollection {
  private documents: any[];
  private collectionName: string;
  private dbName: string;

  constructor(dbName: string, collectionName: string) {
    this.dbName = dbName;
    this.collectionName = collectionName;
    
    // Get or create the collection in our mock storage
    if (!mockDatabases.has(dbName)) {
      mockDatabases.set(dbName, new Map());
    }
    
    const db = mockDatabases.get(dbName)!;
    if (!db.has(collectionName)) {
      db.set(collectionName, []);
    }
    
    this.documents = db.get(collectionName)!;
  }

  async insertOne(doc: any): Promise<any> {
    // Clone the document to avoid mutations
    const docToInsert = { ...doc };
    
    // MongoDB automatically adds _id if not present, but we'll skip that
    // since your adapter doesn't use it
    
    this.documents.push(docToInsert);
    
    return {
      acknowledged: true,
      insertedId: { toString: () => 'mock-id' } // Mock ObjectId
    };
  }

  async findOne(filter: any) {
    return this.documents.find(doc => this.matchesFilter(doc, filter)) || null;
  }

  find(filter: any): any {
    const results = this.documents.filter(doc => this.matchesFilter(doc, filter));
    return new MockCursor(results);
  }

  async countDocuments(filter: any, options?: any) {
    let results = this.documents.filter(doc => this.matchesFilter(doc, filter));
    
    if (options?.skip) {
      results = results.slice(options.skip);
    }
    
    if (options?.limit) {
      results = results.slice(0, options.limit);
    }
    
    return results.length;
  }

  async updateOne(filter: any, update: any) {
    const index = this.documents.findIndex(doc => this.matchesFilter(doc, filter));
    
    if (index === -1) {
      return { matchedCount: 0, modifiedCount: 0 };
    }
    
    // Handle $set operator
    if (update.$set) {
      this.documents[index] = { ...this.documents[index], ...update.$set };
    }
    
    return { 
      matchedCount: 1, 
      modifiedCount: 1,
      acknowledged: true,
      upsertedCount: 0,
      upsertedId: null 
    };
  }

  async deleteOne(filter: any) {
    const index = this.documents.findIndex(doc => this.matchesFilter(doc, filter));
    
    if (index === -1) {
      return { deletedCount: 0 };
    }
    
    this.documents.splice(index, 1);
    return { deletedCount: 1, acknowledged: true };
  }

  private matchesFilter(doc: any, filter: any): boolean {
    for (const [key, value] of Object.entries(filter)) {
      // Handle special operators
      if (key === '$and') {
        return (value as any[]).every(subFilter => this.matchesFilter(doc, subFilter));
      }
      
      if (key === '$or') {
        return (value as any[]).some(subFilter => this.matchesFilter(doc, subFilter));
      }
      
      // Handle field-level operators
      if (typeof value === 'object' && value !== null && !Array.isArray(value)) {
        const fieldValue = doc[key];
        const ops = value as any;
        
        if ('$exists' in ops) {
          if (ops.$exists === false && (fieldValue !== undefined)) {
            return false;
          }
        }
        
        if ('$in' in ops) {
          if (!ops.$in.includes(fieldValue)) return false;
        }
        
        if ('$nin' in ops) {
          if (ops.$nin.includes(fieldValue)) return false;
        }
        
        if ('$gt' in ops) {
          if (!(fieldValue > ops.$gt)) return false;
        }
        
        if ('$gte' in ops) {
          if (!(fieldValue >= ops.$gte)) return false;
        }
        
        if ('$lt' in ops) {
          if (!(fieldValue < ops.$lt)) return false;
        }
        
        if ('$lte' in ops) {
          if (!(fieldValue <= ops.$lte)) return false;
        }
        
        if ('$ne' in ops) {
          if (fieldValue === ops.$ne) return false;
        }
        
        if ('$eq' in ops) {
          if (fieldValue !== ops.$eq) return false;
        }
        
        if ('$regex' in ops) {
          const regex = new RegExp(ops.$regex);
          if (!regex.test(fieldValue)) return false;
        }
      } else {
        // Simple equality check
        if (doc[key] !== value) return false;
      }
    }
    
    return true;
  }
}

class MockDb implements Partial<Db> {
  private dbName: string;

  constructor(dbName: string) {
    this.dbName = dbName;
  }

  collection(name: string): any {
    return new MockCollection(this.dbName, name);
  }

  async dropDatabase() {
    mockDatabases.delete(this.dbName);
    return true;
  }
}

export class MockMongoClient implements Partial<MongoClient> {
  private databases = new Map<string, MockDb>();
  public isConnected = false;

  async connect() {
    this.isConnected = true;
    return this as any;
  }

  async close() {
    this.isConnected = false;
  }

  db(dbName: string): any {
    if (!this.databases.has(dbName)) {
      this.databases.set(dbName, new MockDb(dbName));
    }
    return this.databases.get(dbName)!;
  }
}

// Reset function for cleaning between tests
export function resetMockDatabases() {
  mockDatabases.clear();
} 