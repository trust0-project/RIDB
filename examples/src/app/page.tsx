'use client';

import Image from "next/image";
import React, { useMemo, useState } from 'react';

import { RIDB, SchemaFieldType, StorageType, Doc } from "@trust0/ridb";

const schemas =  {
  demo: {
      version: 1,
      primaryKey: 'id',
      type: SchemaFieldType.object,
      properties: {
          id: {
              type: SchemaFieldType.string,
              maxLength: 60
          },
          age: {
            type: SchemaFieldType.number,
            default: 18,
            required: false
        }
      }
  }
} as const;

type DBType = RIDB<typeof schemas>;

export default function Home() {
  const db: DBType = useMemo(
    () => new RIDB(
      {
          dbName: "test-database",
          schemas,
          migrations: {
            demo: {
              1: function (doc) {
                  return {
                    ...doc,
                    age: doc.age || 18
                  }
              }
            }
        }
      }
    ),
    []
  );
  const [isStarted, setIsStarted] = useState(false);
  const [demos, setDemos] = useState<Doc<typeof schemas.demo>[]>([]);
  const [newDemoId, setNewDemoId] = useState('');
  const [storageType, setStorageType] = useState<StorageType>(StorageType.IndexDB);

  const handleStart = async () => {
    if (db) {
      await db.start({ storageType });
      setIsStarted(true);
      fetchDemos();
    }
  };

  const handleClose = async() => {
    if (db) {
      await db.close();
      setIsStarted(false);
    }
  };

  const fetchDemos = async () => {
    if (db) {
      const demoCollection = db.collections.demo;
      const allDemos = await demoCollection.find({});
      setDemos(allDemos);
    }
  };

  const handleAddDemo = async () => {
    if (db && isStarted && newDemoId) {
      const demoCollection = db.collections.demo;
      debugger;
      await demoCollection.create({ id: newDemoId });
      debugger;
      setNewDemoId('');
      fetchDemos();
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
              className={`p-2 w-full rounded-md shadow-md transition-colors ${
                !newDemoId || !isStarted || !db ? 'bg-gray-300 cursor-not-allowed' : 'bg-green-500 hover:bg-green-600 text-white'
              }`}
              aria-disabled={!newDemoId || !isStarted || !db}
            >
              Add Demo
            </button>
          </div>
        )}
      </main>
    </div>
  );
}
