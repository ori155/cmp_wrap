use crate::KeyFunction;
use crate::wrappers::CmpByKey;

/// Used to easily wrap multiple values with the same key function.
pub struct KeyCmpContext<'k, T, K> {
    key_func: &'k KeyFunction<T, K>
}

impl<'k, T, K> KeyCmpContext<'k, T, K> {
    /// Creates new key context which lets you easily wrap values with the same
    /// key function (same context).
    pub fn new<'kf: 'k>(key_func: &'kf KeyFunction<T, K>) -> KeyCmpContext<'k, T, K> {
        KeyCmpContext {key_func}
    }

    /// This is how you wrap values in the same context context.
    ///
    /// ```
    /// use cmp_wrap::KeyCmpContext;
    ///
    /// let by_length = KeyCmpContext::new(&|v: &Vec<_>| v.len());
    ///
    /// let long_vec = by_length.wrap(vec![1,2,3,4]);
    /// let short_vec = by_length.wrap(vec![1,2]);
    ///
    /// assert!(long_vec > short_vec, "The vec {:?} is longer then {:?}", long_vec, short_vec);
    ///
    /// ```
    pub fn wrap<'kw>(&self, value: T) -> CmpByKey<'kw, T, K> where 'k: 'kw {
        CmpByKey::new(value, self.key_func)
    }
}