import { RIDBError } from '@trust0/ridb-core';
import { RIDBCore } from './core';
import { SchemaTypeRecord } from '@trust0/ridb-core';

// The SharedWorkerGlobalScope interface
interface SharedWorkerGlobalScope {
    onconnect: (event: MessageEvent) => void;
}

/**
 * Maps a dbName -> RIDBCore instance
 */
const dbMap = new Map<string, RIDBCore<any>>();

/**
 * Tracks which ports are using a given dbName.
 * Each time a port "starts" a db, we add the port to its usage set.
 * When a port "closes" for a db, we remove the port from the set.
 * If the set becomes empty, we tear down that DB.
 */
const dbUsageMap = new Map<string, Set<MessagePort>>();

function requireDB(data: any) {
    if (!data.dbName) {
        throw new Error('dbName is required');
    }
    if (!dbMap.has(data.dbName)) {
        throw new Error(`Database ${data.dbName} not found`);
    }
    return dbMap.get(data.dbName)!;
}

async function executeDBOperation(db: RIDBCore<any>, action: string, data: any) {
    const { collection, body } = data;
    const collections = db.collections;
    
    switch (action) {
        case 'find':
            return collections[collection].find(body);
        case 'count':
            return collections[collection].count(body);
        case 'create':
            return collections[collection].create(body);
        case 'update':
            return collections[collection].update(body);
        case 'findById':
            return collections[collection].findById(body);
        case 'delete':
            return collections[collection].delete(body);
        default:
            throw new Error(`Unknown action: ${action}`);
    }
}

// Handle incoming port messages
async function handleMessage(event: MessageEvent, port: MessagePort) {
    // Expecting the shape { action: string, data: any }
    const { action, data, requestId } = event.data || {};
    console.log('[Worker] handleMessage:', action, data);

    try {
        switch (action) {
            case 'start': {
                const { dbName, schemas, migrations, options } = data;
                let db = dbMap.get(dbName);

                if (!db) {
                    db = new RIDBCore({
                        dbName,
                        schemas,
                        migrations: migrations || {},
                    });
                    dbMap.set(dbName, db);
                } 
                
                // Track usage of this db by the current port
                let portSet = dbUsageMap.get(dbName);
                if (!portSet) {
                    portSet = new Set();
                    dbUsageMap.set(dbName, portSet);
                }
                portSet.add(port);

                await db.start(options);

                port.postMessage({
                    status: 'success',
                    requestId,
                    action: 'start',
                    data: { message: `Database ${dbName} started in worker` },
                });
                break;
            }
            case 'close': {
                const { dbName } = data;
                if (dbName) {
                    // Remove this port from tracking usage of dbName
                    const usageSet = dbUsageMap.get(dbName);
                    if (usageSet) {
                        usageSet.delete(port);

                        // If no ports are using this db anymore, tear it down
                        if (usageSet.size === 0) {
                            console.log(`[Worker] Tearing down DB instance for: ${dbName}`);
                            const db = dbMap.get(dbName);
                            if (db) {
                                await db.close();
                            }
                            dbMap.delete(dbName);
                            dbUsageMap.delete(dbName);
                        }
                    }
                }

                // Close this specific port
                port.close();
                break;
            }
            default: {
                // For other actions, we require the database and run the operation
                const db = requireDB(data);
                const result = await executeDBOperation(db, action, data);
                port.postMessage({ 
                    status: 'success', 
                    action: action, 
                    data: result, 
                    requestId 
                });
                break;
            }
        }
    } catch (err) {
        console.error('[Worker] Error:', err);
        if (err instanceof RIDBError) {
            port.postMessage({
                status: 'error',
                data: (err as any).toJSON(),
                action,
                requestId,
            });
        } else {
            port.postMessage({
                status: 'error',
                data: (err as any).message,
                action,
                requestId,
            });
        }
    }
}

const _self: SharedWorkerGlobalScope = self as any;

// The event comes from a connection; we get one port for each tab or client script.
_self.onconnect = (connectEvent: MessageEvent) => {
    console.log('[Worker] onconnect event:', connectEvent);
    const port: MessagePort = connectEvent.ports[0];

    // For each new port, handle all incoming messages via our main handler.
    port.onmessage = (e: MessageEvent) => handleMessage(e, port);
    // In older browsers, needed to begin listening.
    port.start();
};

