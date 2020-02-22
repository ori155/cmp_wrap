# cmp_wrap


Have you ever needed to compare the same data by different fields, depending on context?
If so, this crate is for you!

## Example

### Using context
You probably have some kind of context in which you would like to compare your values, such as
length of vectors or the first value.


```rust
use cmp_wrap::KeyCmpContext;

let by_length = KeyCmpContext::new(&|v: &Vec<_>| v.len());

let long_vec = by_length.wrap(vec![1,2,3,4]);
let short_vec = by_length.wrap(vec![1,2]);

assert!(long_vec > short_vec, "The vec {:?} is longer then {:?}", long_vec, short_vec);

```

### By direct creation
you can define the key function on a "case by case" basis.
```rust
use cmp_wrap::CmpByKey;

let len_as_key = |v: &Vec<_>| v.len();

let long_vec = CmpByKey::new(vec![1,2,3,4], &len_as_key);
let short_vec = CmpByKey::new(vec![1,2], &len_as_key);

assert!(long_vec > short_vec, "The vector {:?} is longer then {:?}", long_vec, short_vec);
```

License: MIT
