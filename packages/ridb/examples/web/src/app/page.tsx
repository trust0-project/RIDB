'use client';

import Image from "next/image";
import React, {  useState, useEffect, useMemo } from 'react';

import {  SchemaFieldType, StorageType, Doc, RIDB } from "@trust0/ridb";


const demoSchema = {
  version: 0 as const,
  primaryKey: 'id',
  type: SchemaFieldType.object,
  encrypted: ["age"],
  properties: {
    id: {
      type: SchemaFieldType.string,
      maxLength: 60
    },
    age: {
      type: SchemaFieldType.number,
      default: 18
    }
  }
};

const schemas = {
  demo: demoSchema
};

export default function Home() {
  const [dbName, setDbName] = useState('test-database');
  const [dbPassword, setDbPassword] = useState('demo');
  const [storageType, setStorageType] = useState<StorageType>(StorageType.IndexDB);

  const db = useMemo<RIDB<typeof schemas>>(
    () => new RIDB({
      dbName,
      schemas,
      worker: true
    }), []
  );

  const [isStarted, setIsStarted] = useState(false);
  const [demos, setDemos] = useState<Doc<typeof schemas.demo>[]>([]);
  const [newDemoId, setNewDemoId] = useState('');
  const [numRecords, setNumRecords] = useState<number>(1);

  useEffect(() => {
    return () => {
      console.log('[Home] Cleaning up RIDBWorker');
      if (db) {
        db.close();
      }
    };
  }, [db]);

  /**
   * Merged connection + start in a single step
   */
  const handleConnectAndStart = async () => {
    if (!db) return;

    // (1) Close any existing connection:
    await db.close();
    console.log(`[Home] Creating a new database connection: ${dbName}`);
    setIsStarted(false);
    setDemos([]);

    // (2) Start the new DB:
    console.log(`[Home] Starting the database ${storageType} with name "${dbName}"`);
    await db.start({ storageType, password: dbPassword });
    console.log(`[Home] Database "${dbName}" started - storageType: ${storageType}`);
    setIsStarted(true);

    // (3) Fetch data
    fetchDemos();
  };

  const handleClose = async () => {
    if (!db) return;
    console.log('[Home] Closing the database');
    await db.close();
    setIsStarted(false);
    console.log('[Home] Database closed');
  };

  const fetchDemos = async () => {
    if (!db) return;
    console.log('[Home] Fetching demos');
    const demoCollection = db.collections.demo;
    const allDemos = await demoCollection.find({});
    setDemos(allDemos);
    console.log('[Home] Demos fetched:', allDemos);
  };

  const handleAddDemo = async () => {
    if (isStarted && newDemoId && db) {
      console.log('[Home] Adding a new demo:', newDemoId);
      const demoCollection = db.collections.demo;
      await demoCollection.create({ id: newDemoId });
      setNewDemoId('');
      fetchDemos();
      console.log('[Home] New demo added:', newDemoId);
    }
  };

  const generateRandomData = async () => {
    if (isStarted && db) {
      console.log(`[Home] Generating random data for ${numRecords} record(s)`);
      const demoCollection = db.collections.demo;
      for (let i = 0; i < numRecords; i++) {
        const randomId = `demo-${Math.random().toString(36).substr(2, 9)}`;
        const randomAge = Math.floor(Math.random() * 100);
        await demoCollection.create({ id: randomId, age: randomAge });
      }
      fetchDemos();
      console.log('[Home] Finished generating random data');
    }
  };

  return (
    <div className="w-screen h-screen flex flex-col bg-gray-100 dark:bg-gray-900">
      {/* Toolbar */}
      <header className="w-full p-4 bg-white dark:bg-gray-800 shadow flex items-center gap-4">
        <div className="flex flex-col sm:flex-row gap-2 sm:items-center">
          <label className="text-gray-700 dark:text-gray-200 font-medium">
            DB Name:
          </label>
          <input
            type="text"
            value={dbName}
            onChange={(e) => setDbName(e.target.value)}
            className="p-2 border rounded-md shadow-sm focus:outline-none focus:ring-2 focus:ring-blue-500 text-gray-900 dark:text-gray-800"
          />
        </div>

        <div className="flex flex-col sm:flex-row gap-2 sm:items-center">
          <label className="text-gray-700 dark:text-gray-200 font-medium">
            Password:
          </label>
          <input
            type="password"
            value={dbPassword}
            onChange={(e) => setDbPassword(e.target.value)}
            className="p-2 border rounded-md shadow-sm focus:outline-none focus:ring-2 focus:ring-blue-500 text-gray-900 dark:text-gray-800"
          />
        </div>

        <div className="flex flex-col sm:flex-row gap-2 sm:items-center">
          <label className="text-gray-700 dark:text-gray-200 font-medium">
            Storage Type:
          </label>
          <select
            onChange={(e) => setStorageType(e.target.value as StorageType)}
            value={storageType}
            className="p-2 rounded-md shadow-sm bg-white dark:bg-gray-800 text-gray-700 dark:text-gray-200 focus:outline-none focus:ring-2 focus:ring-blue-500 transition-colors"
          >
            <option value={StorageType.IndexDB}>IndexDB</option>
            <option value={StorageType.InMemory}>InMemory</option>
          </select>
        </div>
        
        <div className="ml-auto">
          {!isStarted ? (
            <button
              onClick={handleConnectAndStart}
              className="p-2 rounded-md shadow-md transition-colors bg-blue-500 hover:bg-blue-600 text-white"
              aria-pressed={isStarted}
            >
              Connect & Start DB
            </button>
          ) : (
            <button
              onClick={handleClose}
              className="p-2 rounded-md shadow-md transition-colors bg-red-500 hover:bg-red-600 text-white"
              aria-pressed={!isStarted}
            >
              Close DB
            </button>
          )}
        </div>
      </header>

      {/* Main Content */}
      <main className="flex-1 overflow-auto p-4 flex flex-col items-center sm:items-start max-w-4xl mx-auto w-full">
        <div className="flex gap-4 items-center w-full mb-8">
          <p className="text-lg font-medium">
            Status: {isStarted ? 'Started' : 'Stopped'}
          </p>
        </div>

        {isStarted && (
          <div className="w-full flex flex-col gap-4">
            <div>
              <h2 className="text-xl font-semibold mb-2">Demos</h2>
              <ul className="list-disc pl-5 mb-4">
                {demos.map(demo => (
                  <li key={demo.id} className="text-lg">
                    {demo.id} - {demo.age}
                  </li>
                ))}
              </ul>
            </div>

            <div>
              <input
                type="text"
                value={newDemoId}
                onChange={(e) => setNewDemoId(e.target.value)}
                placeholder="New Demo ID"
                className="p-2 border rounded-md shadow-sm w-full mb-2 focus:outline-none focus:ring-2 focus:ring-blue-500 text-gray-900 dark:text-gray-800"
                aria-label="New Demo ID"
              />
              <button
                onClick={handleAddDemo}
                disabled={!newDemoId || !isStarted || !db}
                className={`p-2 w-full rounded-md shadow-md transition-colors ${
                  !newDemoId || !isStarted || !db
                    ? 'bg-gray-300 cursor-not-allowed'
                    : 'bg-green-500 hover:bg-green-600 text-white'
                }`}
                aria-disabled={!newDemoId || !isStarted || !db}
              >
                Add Demo
              </button>
            </div>

            <div>
              <input
                type="number"
                value={numRecords}
                onChange={(e) => setNumRecords(Number(e.target.value))}
                placeholder="Number of Records"
                className="p-2 border rounded-md shadow-sm w-full mb-2 focus:outline-none focus:ring-2 focus:ring-blue-500 text-gray-900 dark:text-gray-800"
                aria-label="Number of Records"
              />
              <button
                onClick={generateRandomData}
                disabled={!isStarted || !db}
                className={`p-2 w-full rounded-md shadow-md transition-colors ${
                  !isStarted || !db
                    ? 'bg-gray-300 cursor-not-allowed'
                    : 'bg-purple-500 hover:bg-purple-600 text-white'
                }`}
                aria-disabled={!isStarted || !db}
              >
                Generate Random Data
              </button>
            </div>
          </div>
        )}
      </main>
    </div>
  );
}
