//! # interface - Database Driver API
//! 
//! Provides a unified interface for connecting to SQL databases.
//!
//! ## Support
//! - MSSQL (via `rustds`)

/// A database connection.
pub trait Connection: Sized {
    type Error;
    
    /// Connect to a database using a DSN string.
    /// e.g. `mssql://user:pass@host:1433/db` or `postgres://user:pass@host:5432/db`
    fn connect(dsn: &str) -> Result<Self, Self::Error>;
    
    /// Executes a SQL query and return a row iterator.
    fn query(&mut self, sql: &str) -> Result<impl Rows, Self::Error>;
    
    /// Close the connection.
    fn disconnect(self) -> Result<(), Self::Error>;
}

/// A query result
pub trait Rows {
    
    /// Get the next row or None.
    fn next(&mut self) -> Option<impl Row>;
}

/// A single row
pub trait Row {
    
    /// Get the bytes of the row.
    fn get(&self, col: &str) -> Option<&[u8]>;
}