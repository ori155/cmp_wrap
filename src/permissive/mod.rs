//! The permissive option will let you compare different types
//! as long as they are mapped (keyed) to comparable Key types.
//!
//! Be careful as you may compare things you didn't want to compare.

mod wrappers;
mod context;

pub use wrappers::CmpByKey;
pub use context::KeyCmpContext;
