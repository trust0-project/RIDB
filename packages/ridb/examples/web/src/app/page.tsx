'use client';

import Image from "next/image";
import React, {  useState, useEffect } from 'react';

import {  SchemaFieldType, StorageType, Doc, Worker } from "@trust0/ridb";


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
  const [db, setWorker] = useState<Worker<typeof schemas> | null>(null);

  useEffect(() => {
    if (typeof window !== 'undefined' && 'SharedWorker' in window) {
      try {
        console.log('[Home] Initializing RIDBWorker');
        const newWorker = new Worker({
          dbName: "test-database",
          schemas
        });
        setWorker(newWorker);
        return () => {
          console.log('[Home] Cleaning up RIDBWorker');
          newWorker.close();
        };
      } catch (error) {
        console.error('[Home] Error initializing SharedWorker:', error);
      }
    } else {
      console.warn('[Home] SharedWorker is not supported in this environment');
    }
  }, []);

  const [isStarted, setIsStarted] = useState(false);
  const [demos, setDemos] = useState<Doc<typeof schemas.demo>[]>([]);
  const [newDemoId, setNewDemoId] = useState('');
  const [storageType, setStorageType] = useState<StorageType>(StorageType.IndexDB);
  const [operationTime, setOperationTime] = useState<string | null>(null);
  const [operationHistory, setOperationHistory] = useState<{ name: string; time: string }[]>([]);
  const [numRecords, setNumRecords] = useState<number>(1);

  const logOperation = (name: string, startTime: number, endTime: number) => {
    const timeTaken = ((endTime - startTime) / 1000).toFixed(2);
    setOperationHistory((prev) => [...prev, { name, time: `${timeTaken} seconds` }]);
  };

  const handleStart = async () => {
    if (db) {
      console.log('[Home] Starting the database');
      const startTime = performance.now();
      await db.start({ storageType, password: "demo" });
      const endTime = performance.now();
      logOperation('Start DB', startTime, endTime);
      setIsStarted(true);
      console.log('[Home] Database started successfully');
      fetchDemos();
    }
  };

  const handleClose = async () => {
    if (db) {
      console.log('[Home] Closing the database');
      const startTime = performance.now();
      await db.close();
      const endTime = performance.now();
      logOperation('Close DB', startTime, endTime);
      setIsStarted(false);
      console.log('[Home] Database closed');
    }
  };

  const fetchDemos = async () => {
    if (db) {
      console.log('[Home] Fetching demos');
      const startTime = performance.now();
      const demoCollection = db.collections.demo;
      const allDemos = await demoCollection.find({});
      const endTime = performance.now();
      logOperation('Fetch Demos', startTime, endTime);
      setDemos(allDemos);
      console.log('[Home] Demos fetched:', allDemos);
    }
  };

  const handleAddDemo = async () => {
    if (db && isStarted && newDemoId) {
      console.log('[Home] Adding a new demo:', newDemoId);
      const startTime = performance.now();
      const demoCollection = db.collections.demo;
      await demoCollection.create({ id: newDemoId });
      const endTime = performance.now();
      logOperation('Add Demo', startTime, endTime);
      setNewDemoId('');
      fetchDemos();
      console.log('[Home] New demo added:', newDemoId);
    }
  };

  const generateRandomData = async () => {
    if (db && isStarted) {
      console.log(`[Home] Generating random data for ${numRecords} record(s)`);
      const startTime = performance.now();
      const demoCollection = db.collections.demo;
      for (let i = 0; i < numRecords; i++) {
        const randomId = `demo-${Math.random().toString(36).substr(2, 9)}`;
        const randomAge = Math.floor(Math.random() * 100);
        await demoCollection.create({ id: randomId, age: randomAge });
      }
      const endTime = performance.now();
      logOperation('Generate Random Data', startTime, endTime);
      fetchDemos();
      console.log('[Home] Finished generating random data');
    }
  };

  return (
    <div className="grid grid-rows-[20px_1fr_20px] items-center justify-items-center min-h-screen p-8 pb-20 gap-16 sm:p-20 font-[family-name:var(--font-geist-sans)] bg-gray-100 dark:bg-gray-900">
      <main className="flex flex-col gap-8 row-start-2 items-center sm:items-start w-full max-w-2xl">
        <Image
          className="dark:invert"
          src="/next.svg"
          alt="Next.js logo"
          width={180}
          height={38}
          priority
        />
        <div className="flex gap-4 items-center flex-col sm:flex-row w-full">
          <h1 className="text-2xl font-bold">Database</h1>
          <select
            onChange={(e) => setStorageType(e.target.value as StorageType)}
            value={storageType}
            className="p-3  rounded-md shadow-sm bg-white dark:bg-gray-800 text-gray-700 dark:text-gray-200 focus:outline-none focus:ring-2 focus:ring-blue-500  transition-colors"
            aria-label="Select storage type"
          >
            <option value={StorageType.IndexDB}>IndexDB</option>
            <option value={StorageType.InMemory}>InMemory</option>
          </select>
          {!isStarted && (
            <button
              onClick={handleStart}
              className="p-2 rounded-md shadow-md transition-colors bg-blue-500 hover:bg-blue-600 text-white"
              aria-pressed={isStarted}
            >
              Start DB
            </button>
          )}
          {isStarted && (
            <button
              onClick={handleClose}
              className="p-2 rounded-md shadow-md transition-colors bg-red-500 hover:bg-red-600 text-white"
              aria-pressed={!isStarted}
            >
              Close DB
            </button>
          )}
          <p className="text-lg font-medium">Status: {isStarted ? 'Started' : 'Stopped'}</p>
          <p className="text-lg font-medium">Operation Time: {operationTime}</p>
        </div>
        {isStarted && (
          <div className="w-full">
            <h2 className="text-xl font-semibold mb-2">Demos</h2>
            <ul className="list-disc pl-5 mb-4">
              {demos.map(demo => (
                <li key={demo.id} className="text-lg">{demo.id} - {demo.age}</li>
              ))}
            </ul>
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
              className={`p-2 w-full rounded-md shadow-md transition-colors ${!newDemoId || !isStarted || !db ? 'bg-gray-300 cursor-not-allowed' : 'bg-green-500 hover:bg-green-600 text-white'
                }`}
              aria-disabled={!newDemoId || !isStarted || !db}
            >
              Add Demo
            </button>
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
              className={`p-2 w-full rounded-md shadow-md transition-colors ${!isStarted || !db ? 'bg-gray-300 cursor-not-allowed' : 'bg-purple-500 hover:bg-purple-600 text-white'
                }`}
              aria-disabled={!isStarted || !db}
            >
              Generate Random Data
            </button>
          </div>
        )}
        <div className="w-full">
          <h2 className="text-xl font-semibold mb-2">Operation History</h2>
          <ul className="list-disc pl-5 mb-4">
            {operationHistory.map((operation, index) => (
              <li key={index} className="text-lg">
                {operation.name}: {operation.time}
              </li>
            ))}
          </ul>
        </div>
      </main>
    </div>
  );
}
