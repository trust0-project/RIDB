use web_sys::IdbDatabase;
use std::sync::Arc;
use parking_lot::Mutex;
use std::collections::HashMap;
use std::sync::Weak;
use lazy_static::lazy_static;
use wasm_bindgen::JsValue;
use wasm_bindgen::JsCast;

// Global connection pool
lazy_static! {
    pub static ref POOL: IndexDBPool = IndexDBPool::new();
}

// Add these trait implementations before the IndexDBPool struct
unsafe impl Send for IndexDBPool {}
unsafe impl Sync for IndexDBPool {}

pub struct IndexDBPool {
    connections: Arc<Mutex<HashMap<String, Arc<IdbDatabase>>>>,
}

impl IndexDBPool {
    fn new() -> Self {
        Self {
            connections: Arc::new(Mutex::new(HashMap::new()))
        }
    }

    /// Retrieves a connection from the pool, recreating it if it's closed
    pub fn get_connection(&self, name: &str) -> Option<Arc<IdbDatabase>> {
        let mut connections = self.connections.lock();
        if let Some(db) = connections.get(name) {
            // Check if the database connection is still valid
            // We need to clone before checking to prevent deadlocks
            let db_clone = db.clone();
            
            // Release the lock before performing the potentially slow operation
            drop(connections);
            
            if is_db_closed(&db_clone) {
                // If closed, remove it from the pool (requires acquiring the lock again)
                let mut connections = self.connections.lock();
                connections.remove(name);
                None
            } else {
                Some(db_clone)
            }
        } else {
            None
        }
    }

    /// Stores a new connection in the pool
    pub fn store_connection(&self, name: String, db: Weak<IdbDatabase>) {
        let mut connections = self.connections.lock();
        if let Some(arc_db) = db.upgrade() {
            // Verify the database is not closed before storing
            if !is_db_closed(&arc_db) {
                connections.insert(name, arc_db);
            }
        }
    }

    /// Removes a connection from the pool
    pub fn remove_connection(&self, name: &str) {
        let mut connections = self.connections.lock();
        connections.remove(name);
    }
    
    /// Checks all connections in the pool and removes any that are closed
    pub fn clean_connections(&self) {
        let mut connections = self.connections.lock();
        let to_remove: Vec<String> = connections
            .iter()
            .filter(|(_, db)| is_db_closed(db))
            .map(|(name, _)| name.clone())
            .collect();
        
        // Remove closed connections
        for name in to_remove {
            connections.remove(&name);
        }
    }
}

// Move the is_closed check to a standalone function to avoid trait implementation
// that might cause issues with external types
fn is_db_closed(db: &IdbDatabase) -> bool {
    // Safer approach to check if database is closed
    match db.transaction_with_str_and_mode(
        "__non_existent_store__",
        web_sys::IdbTransactionMode::Readonly
    ) {
        // If we can create a transaction, the database is open
        // (even though this specific transaction will fail with store not found)
        Ok(_) => false,
        // Check the error - if it's a DOM exception with NOT_FOUND_ERR, the DB is open
        // but if it's an InvalidStateError, the DB is closed
        Err(err) => {
            if let Some(err_obj) = err.dyn_ref::<js_sys::Object>() {
                if let Ok(name) = js_sys::Reflect::get(err_obj, &JsValue::from_str("name")) {
                    if let Some(name_str) = name.as_string() {
                        return name_str == "InvalidStateError";
                    }
                }
            }
            // If we can't determine the error type, assume it's closed
            true
        }
    }
}