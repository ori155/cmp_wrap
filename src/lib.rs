


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


#[cfg(test)]
mod tests {
    use crate::CmpByKey;

    #[test]
    fn can_wrap_i32_with_closure() {
        let x = CmpByKey::new(32, &|x: &i32| -> i32 { -*x });
    }
}
