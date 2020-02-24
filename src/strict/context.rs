use super::wrappers::CmpByKey;

/// Used to easily wrap multiple values with the same key function.
pub struct KeyCmpContext<KF> {
    key_func: KF
}

impl<KF> KeyCmpContext<KF> {
    /// Creates new key context which lets you easily wrap values with the same
    /// key function (same context).
    pub fn new<T, K>(key_func: KF) -> KeyCmpContext<KF>
        where KF: Fn(&T) -> K
    {
        KeyCmpContext {key_func}
    }

    /// This is how you wrap values in the same context context.
    ///
    /// ```
    /// use cmp_wrap::strict::KeyCmpContext;
    ///
    /// let by_length = KeyCmpContext::new(|v: &Vec<_>| v.len());
    ///
    /// let long_vec = by_length.wrap(vec![1,2,3,4]);
    /// let short_vec = by_length.wrap(vec![1,2]);
    ///
    /// assert!(long_vec > short_vec, "The vec {:?} is longer then {:?}", long_vec, short_vec);
    ///
    /// ```
    pub fn wrap<T>(&self, value: T) -> CmpByKey<KF, T> where
    {
        CmpByKey::new(value, &self.key_func)
    }
}


#[cfg(test)]
mod test {
    #[test]
    fn context_new_accept_reference() {
        use super::KeyCmpContext;

        let _by_length = KeyCmpContext::new(&|v: &Vec<i32>| v.len());

    }

}