//!
//! Have you ever needed to compare the same data by different fields, depending on context?
//! If so, this crate is for you!
//!
//! The main feature of this crate is a lightweight wrapper around types which lets you compare
//! them after a mappeing (keying) function, meaning the parital_cmp operators ( < , > , == ...)
//! are applied on the result of the keying function.
//!
//! There are two main modules in this crate: [permissive] and [strict].
//! The permissive lets you compare different types as long as the keying function returns
//! comparable types.
//!
//! The strict implementation woun't let you compare different types, and will save you from
//! comparing by two different contexts.
//!
//! # Examples
//!
//! ## Using context
//! You probably have some kind of context in which you would like to compare your values, such as
//! length of vectors or the first value.
//!
//!
//! ```
//! use cmp_wrap::permissive::KeyCmpContext;
//!
//! let by_length = KeyCmpContext::new(|v: &Vec<_>| v.len());
//!
//! let long_vec = by_length.wrap(vec![1,2,3,4]);
//! let short_vec = by_length.wrap(vec![1,2]);
//!
//! assert!(long_vec > short_vec, "The vec {:?} is longer then {:?}", long_vec, short_vec);
//!
//! ```
//!
//! One might want to use multiple contexts for the same data
// TODO: I would like to be able to take keying function for &T even when T = &T' (as bellow)
//!
//! ```
//! use cmp_wrap::strict::KeyCmpContext;
//!
//!
//! let long_vec = vec![1,2,3,4];
//! let short_vec = vec![4,2];
//!
//! let by_length = KeyCmpContext::new(|v: &&Vec<_>| v.len());
//! let by_first_element = KeyCmpContext::new(|v: &&Vec<_>| v[0]);
//!
//! let by_length_long = by_length.wrap(&long_vec);
//! let by_length_short = by_length.wrap(&short_vec);
//!
//! let by_first_element_long = by_first_element.wrap(&long_vec);
//! let by_first_element_short = by_first_element.wrap(&short_vec);
//!
//! assert!(by_length_long > by_length_short,
//!                     "The vec {:?} is longer then {:?}", long_vec, short_vec);
//! assert!(by_first_element_long < by_first_element_short,
//!                     "The vec's {:?} first element is smaller then {:?}'s", long_vec, short_vec);
//!
//! ```
//!
//! ## By direct creation
//! you can define the key function on a "case by case" basis.
//! ```
//! use cmp_wrap::permissive::CmpByKey;
//!
//! let len_as_key = |v: &Vec<_>| v.len();
//!
//! let long_vec = CmpByKey::new(vec![1,2,3,4], &len_as_key);
//! let short_vec = CmpByKey::new(vec![1,2], &len_as_key);
//!
//! assert!(long_vec > short_vec, "The vector {:?} is longer then {:?}", long_vec, short_vec);
//! ```

pub mod strict;
pub mod permissive;

