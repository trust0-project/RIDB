import { RIDB, SchemaTypeRecord } from './index';

// The SharedWorkerGlobalScope interface
interface SharedWorkerGlobalScope {
    onconnect: (event: MessageEvent) => void;
}

/**
 * Maps a dbName -> RIDB instance
 */
const dbMap = new Map<string, RIDB>();

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

async function find(db: RIDB<SchemaTypeRecord>, data: any) {
    const { collection, body } = data;
    return db.collections[collection].find(body);
}

async function count(db: RIDB<SchemaTypeRecord>, data: any) {
    const { collection, body } = data;
    return db.collections[collection].count(body);
}

async function create(db: RIDB<SchemaTypeRecord>, data: any) {
    const { collection, body } = data;
    return db.collections[collection].create(body);
}

async function update(db: RIDB<SchemaTypeRecord>, data: any) {
    const { collection, body } = data;
    return db.collections[collection].update(body);
}

async function findById(db: RIDB<SchemaTypeRecord>, data: any) {
    const { collection, id } = data;
    return db.collections[collection].findById(id);
}

function getRequest(action: string) {
    switch (action) {
        case 'find':
            return find;
        case 'count':
            return count;
        case 'create':
            return create;
        case 'findById':
            return findById;
        case 'update':
            return update;
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
                    db = new RIDB({ 
                        dbName, 
                        schemas, 
                        migrations, 
                        worker: false 
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
                            console.log(`[Worker] Tearing down RIDB instance for: ${dbName}`);
                            // If your RIDB class has a close or teardown method,
                            // call it here. For example:
                            // await dbMap.get(dbName)?.close();
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
                // For other actions, we require the database and run the request
                const db = requireDB(data);
                const request = getRequest(action);
                if (!request) {
                    throw new Error(`Unknown action: ${action}`);
                }
                const result = await request(db, data);
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
        port.postMessage({
            status: 'error',
            data: {
                code: (err as any).code,
                type: (err as any).type,
                message: (err as any).message,
            },
            action,
            requestId,
        });
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

