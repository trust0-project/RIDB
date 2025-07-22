import { describe, beforeAll, afterEach, vi } from 'vitest';
import { runTestsNodeOnly } from '@trust0/ridb/testing';
import {createMongoDB} from '../src';
import { MockMongoClient, resetMockDatabases } from './mock-mongodb';


// Mock the mongodb module before importing the storage adapter
vi.mock('mongodb', () => ({
    MongoClient: MockMongoClient
}));

const MongoDB = await createMongoDB();

describe('MongoDB', async () => {
    beforeAll(async () => {
        // Mock connection URL for the tests
        process.env.MONGODB_URL = 'mongodb://mock-server:27017';
    });

    afterEach(async () => {
        // Reset the mock database between tests
        resetMockDatabases();
    });

    runTestsNodeOnly(
        [{name: "MongoDB", storage: MongoDB}]
    );
});
