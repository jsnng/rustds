//! # Design Rationale
//! The original rationale is to allow different fed-auth providers to be used without
//! coupling them to the TDS driver. The plugin runs in a completely separate process 
//! via `fork()`. If it crashes or hangs, the TDS driver will just kill it. This is a 
//! pragmatic trade-off between using IPC or directly integrating the plugin into the 
//! parent process. The design is for **fault**, not security isolation. Thus, this
//! design assumes trusted plugins - supporting untrusted third-party plugins requires
//! a different architecture and adds complexity.
//!
//! # Security Considerations
//! - `fork()` inherits the parent's entire address space — a plugin can read
//!   sensitive data from the TDS driver's memory snapshot.
//! - The trust boundary is at the plugin — it is already handling credentials
//!   and communicating with the identity provider. Therefore, adding isolation 
//!   beyond fault tolerance provides no additional security guarantee.
extern crate alloc;

pub mod handle;
pub mod isolation;