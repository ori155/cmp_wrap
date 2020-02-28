//! The permissive option will let you compare different types
//! as long as they are mapped (keyed) to comparable Key types.
//!
//! Be careful as you may compare things you didn't want to compare.
//!
//! # Example
//! The permissive option will let you compare different types or by different
//! keys, as long as they are comparable.
//!
//! ```
//! use cmp_wrap::permissive::KeyCmpContext;
//!
//! let long_vec = vec![1,2,3,4,5];
//! let short_vec = vec![4,2];
//!
//! let by_length = KeyCmpContext::new(|v: &&Vec<_>| v.len());
//! let by_first_element = KeyCmpContext::new(|v: &&Vec<_>| v[0]);
//!
//! let by_length_long = by_length.wrap(&long_vec);
//!
//! let by_first_element_short = by_first_element.wrap(&short_vec);
//!
//! assert!(by_length_long > by_first_element_short,
//!         "The long array is longer then the first element of the sort array")
//! ```

mod wrappers;
mod context;

pub use wrappers::CmpByKey;
pub use context::KeyCmpContext;
