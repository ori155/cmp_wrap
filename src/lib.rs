use core::cmp;

struct CmpByKey<'k, T, K> {
    inner: T,
    key_func: &'k dyn Fn(&T) -> K
}

impl<'k, T,K> CmpByKey<'k, T,K> {
    pub fn new<'kf>(value: T, key_func: &'kf dyn Fn(&T) -> K) -> Self
        where 'kf: 'k
    {
        Self{ inner: value, key_func }
    }
}

impl<T, K: PartialOrd> PartialOrd for CmpByKey<'_, T, K> {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        let self_key:  K = (self.key_func)(&self.inner);
        let other_key: K = (other.key_func)(&other.inner);
        self_key.partial_cmp(&other_key)
    }
}

impl<T, K: PartialEq> PartialEq for CmpByKey<'_, T, K> {
    fn eq(&self, other: &Self) -> bool {
        let self_key:  K = (self.key_func)(&self.inner);
        let other_key: K = (other.key_func)(&other.inner);
        self_key == other_key
    }
}

impl<T, K: Eq> Eq for CmpByKey<'_, T, K> {}

#[cfg(test)]
mod tests {
    use crate::CmpByKey;

    #[test]
    fn can_wrap_i32_with_closure() {
        let x = CmpByKey::new(32, &|x: &i32| -> i32 { -*x });
    }

    #[test]
    fn can_compare_two_i32_with_reversed_order() {
        let x32 = CmpByKey::new(32, &|x: &i32| -> i32 { -*x });
        let y33 = CmpByKey::new(33, &|x: &i32| -> i32 { -*x });

        assert!(x32 > y33, "should be in reversed order" );

    }
}
