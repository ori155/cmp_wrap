//! The strict option will not let you compare different types
//! even if they're mapped (keyed) to comparable Key types.
//!
//! Please notice that you have to use the same function signature for all of the
//! strict Cmp types, which means you'll have to build them using reference to the
//! same closure if you chose to create CmpByKey yourselves and not use the context.
mod wrappers;
mod context;

pub use wrappers::CmpByKey;
pub use context::KeyCmpContext;
