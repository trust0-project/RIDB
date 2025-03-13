use web_sys::IdbDatabase;
use std::sync::Arc;
use parking_lot::Mutex;
use std::collections::HashMap;
use std::sync::Weak;
use lazy_static::lazy_static;



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
            if db.is_closed() {
                // Remove the closed connection
                connections.remove(name);
                None
            } else {
                Some(db.clone())
            }
        } else {
            None
        }
    }

    /// Stores a new connection in the pool
    pub fn store_connection(&self, name: String, db: Weak<IdbDatabase>) {
        let mut connections = self.connections.lock();
        if let Some(arc_db) = db.upgrade() {
            connections.insert(name, arc_db);
        }
    }

    /// Removes a connection from the pool
    pub fn remove_connection(&self, name: &str) {
        let mut connections = self.connections.lock();
        connections.remove(name);
    }
}

// Add this extension trait
trait IdbDatabaseExt {
    fn is_closed(&self) -> bool;
}

impl IdbDatabaseExt for IdbDatabase {
    fn is_closed(&self) -> bool {
        // Attempt to start a dummy transaction to see if the database is closed
        match self.transaction_with_str("__non_existent_store__") {
            Ok(_) => false,
            Err(_) => true,
        }
    }
}