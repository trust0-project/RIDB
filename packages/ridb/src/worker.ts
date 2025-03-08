import { RIDB, SchemaTypeRecord } from './index';
const connections = new Map<string, RIDB>();

// The SharedWorkerGlobalScope interface
interface SharedWorkerGlobalScope {
    onconnect: (event: MessageEvent) => void;
}

function requireDB(data: any) {
    if (!data.dbName) {
        throw new Error('dbName is required');
    }
    if (!connections.has(data.dbName)) {
        throw new Error(`Database ${data.dbName} not found`);
    }
    return connections.get(data.dbName)!;
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

async function findById(db: RIDB<SchemaTypeRecord>, data: any) {
    const { collection, id } = data;
    return db.collections[collection].findById(id);
}


// Handle incoming port messages
async function handleMessage(event: MessageEvent, port: MessagePort) {
    // Expecting the shape { action: string, data: any }
    const { action, data, requestId } = event.data || {};
    console.log('[Worker] handleMessage:', action, data);
    try {
        switch (action) {
            case 'find':
            case 'findById':
            case 'create':
            case 'count': {
                const db = requireDB(data);
                const request = action === 'find' ? find :
                    action === 'count' ? count :
                        action === 'create' ? create :
                            action === 'findById' ? findById : null;

                if (!request) {
                    throw new Error(`Unknown action: ${action}`);
                }

                const result = await request(db, data);
                port.postMessage({ status: 'success', action: action, data: result, requestId });
                break;
            }
            case 'start': {
                // See if a DB is already in the map, else create it
                const { dbName, schemas, migrations, options } = data;
                if (!connections.has(dbName)) {
                    console.log(`[Worker] Creating new RIDB instance for: ${dbName}`);
                    const db = new RIDB({ dbName, schemas, migrations, worker: false });
                    await db.start(options);
                    connections.set(dbName, db);
                    port.postMessage({
                        status: 'success',
                        requestId,
                        action: 'start',
                        data: { message: `Database ${dbName} started in worker` },
                    });
                } else {
                    console.log(`[Worker] Re-using existing RIDB instance for: ${dbName}`);
                    port.postMessage({
                        status: 'success',
                        requestId,
                        action: 'start',
                        data: { message: `RE-USED Database ${dbName} in worker` },
                    });
                }
                break;
            }

            case 'closeDB': {
                const { dbName } = data;
                if (connections.has(dbName)) {
                    console.log(`[Worker] Deleting RIDB instance for: ${dbName}`);
                    // Make sure to do any teardown on DB if needed
                    connections.delete(dbName);
                }
                port.close()
                break;
            }

            default: {
                throw new Error(`Unknown action: ${action}`);
            }
        }
    } catch (err) {
        console.error('[Worker] Error:', err);
        port.postMessage({
            status: 'error',
            message: `Error: ${JSON.stringify((err as Error))}`,
            action,
            requestId,
        });
    }
}

const _self: SharedWorkerGlobalScope = self as any;

// The event comes from a connection; we get one port for each tab or client script
_self.onconnect = (connectEvent: MessageEvent) => {
    console.log('[Worker] onconnect event:', connectEvent);
    const port = (connectEvent as any).ports[0];
    // The port is always needed to communicate back.
    port.onmessage = (e: MessageEvent) => handleMessage(e, port);
    port.start(); // In older browsers, needed to begin listening
};