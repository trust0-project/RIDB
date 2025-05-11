import { v4 as uuidv4 } from 'uuid';
import { PendingRequests } from './types';

export interface WorkerInstance {
  worker: SharedWorker;
  sessionId: string;
}

/**
 * Creates a shared worker instance for RIDB
 */
export function createWorkerInstance(): WorkerInstance {
  let worker: SharedWorker;
  const sessionId = uuidv4();

  try {
    worker = new SharedWorker(new URL('@trust0/ridb/worker', import.meta.url), { type: 'module' });
  } catch (err) {
    const workerPath = require.resolve('@trust0/ridb/worker');
    worker = new SharedWorker(workerPath, { type: 'module' });
  }
  
  return { worker, sessionId };
}

/**
 * Sets up worker message handling
 */
export function setupWorkerMessageHandler(
  worker: SharedWorker, 
  pendingRequests: PendingRequests, 
  onMessage?: (event: MessageEvent) => void
) {
  worker.port.onmessage = async (event: MessageEvent) => {
    const { requestId, status, data } = event.data || {};
    console.log('[RIDBWorker] Received message from worker:', event.data);
    
    if (requestId && pendingRequests.has(requestId)) {
      const pendingRequest = pendingRequests.get(requestId)!;
      if (status === 'success') {
        console.log(`[RIDBWorker] Request ${requestId} successful. Data:`, data);
        pendingRequest.resolve(data);
      } else {
        console.error(`[RIDBWorker] Request ${requestId} failed. Error:`, data);
        // We'll need to import RIDBError directly in the consumer
        pendingRequest.reject(data);
      }
      pendingRequests.delete(requestId);
    }

    if (onMessage) {
      onMessage(event);
    }
  };
} 