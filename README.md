# cmp_wrap


Have you ever needed to compare the same data by different fields, depending on context?
If so, this crate is for you!

The main feature of this crate is a lightweight wrapper around types which lets you compare
them after a mappeing (keying) function, meaning the parital_cmp operators ( < , > , == ...)
are applied on the result of the keying function.

There are two main modules in this crate: [permissive] and [strict].
The permissive lets you compare different types as long as the keying function returns
comparable types.

The strict implementation woun't let you compare different types, and will save you from
comparing by two different contexts.

## Examples

### Using context
You probably have some kind of context in which you would like to compare your values, such as
length of vectors or the first value.


```rust
use cmp_wrap::permissive::KeyCmpContext;

let by_length = KeyCmpContext::new(|v: &Vec<_>| v.len());

let long_vec = by_length.wrap(vec![1,2,3,4]);
let short_vec = by_length.wrap(vec![1,2]);

assert!(long_vec > short_vec, "The vec {:?} is longer then {:?}", long_vec, short_vec);

```

One might want to use multiple contexts for the same data

License: MIT
