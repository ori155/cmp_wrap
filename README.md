# cmp_wrap


Have you ever needed to compare the same data by different fields, depending on context?
If so, this crate is for you!

## Example
```rust
use cmp_wrap::CmpByKey;

let len_as_key = |v: &Vec<_>| v.len();

let long_vec = CmpByKey::new(vec![1,2,3,4], &len_as_key);
let short_vec = CmpByKey::new(vec![1,2], &len_as_key);

assert!(long_vec > short_vec, "The vector {:?} is longer then {:?}", long_vec, short_vec);
```

License: MIT
