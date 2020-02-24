use core::{cmp, fmt};

/// The main structure of this crate.
/// Lets you define a "key function" over any structure, which will change the way the value
/// is being compared. Very useful when you need to enter your data to different data structures
/// (such as max heaps, sorted trees and so on), with different comparision criteria.
pub struct CmpByKey<'kf, KF, T> {
    inner: T,
    key_func: &'kf KF
}

impl<'kf, KF, T> CmpByKey<'kf, KF, T> {
    /// Let you wrap a value with a key function which defines the way to compare it to other
    /// values.
    ///
    /// # Example
    /// ```
    /// use cmp_wrap::CmpByKey;
    /// let x = CmpByKey::new(32, &|x: &i32| -> i32 { -*x });
    /// ```
    pub fn new<'rkf>(value: T, key_func: &'rkf KF) -> Self where
        'rkf: 'kf
    {
        Self{ inner: value, key_func: key_func }
    }

    /// Lets you get the original value which the wrapper wrappes, destroys the wrapper.
    ///
    /// # Example
    /// ```
    /// use cmp_wrap::CmpByKey;
    /// let v: i32 = 32;
    ///
    /// fn is_type_i32(x: i32) {};
    ///
    /// let x = CmpByKey::new(v, &|x: &i32| -> i32 { -*x });
    /// let v_again = x.remove_wrapper();
    ///
    /// is_type_i32(v_again);
    /// ```
    pub fn remove_wrapper(self) -> T {
        self.inner
    }

    fn get_key<K>(&self) -> K where
        KF: Fn(&T) -> K
    {
        (self.key_func)(&self.inner)
    }
}

impl<T, K, KF, OT, OKF, OK> PartialOrd<CmpByKey<'_, OKF, OT>> for CmpByKey<'_, KF, T>  where
    K: PartialOrd<OK>,
    KF:  Fn(&T) -> K,
    OKF: Fn(&OT) -> OK,
{
    fn partial_cmp(&self, other: &CmpByKey<'_, OKF, OT>) -> Option<cmp::Ordering> {
        self.get_key().partial_cmp(&other.get_key())
    }
}

impl<T, K: Ord, KF> Ord for CmpByKey<'_, KF, T> where
    KF: Fn(&T) -> K
{
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.get_key().cmp(&other.get_key())
    }
}

impl<T, K, KF, OT, OKF, OK> PartialEq<CmpByKey<'_, OKF, OT>> for CmpByKey<'_, KF, T> where
    K: PartialEq<OK>,
    KF: Fn(&T) -> K,
    OKF: Fn(&OT) -> OK
{
fn eq(&self, other: &CmpByKey<'_, OKF, OT>) -> bool {
        self.get_key() == other.get_key()
    }
}

impl<T, K: Eq, KF> Eq for CmpByKey<'_, KF, T> where KF: Fn(&T) -> K {}

impl<T: fmt::Debug, KF> fmt::Debug for CmpByKey<'_, KF, T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        self.inner.fmt(f)
    }
}

#[cfg(test)]
mod tests {
    use crate::CmpByKey;

    #[test]
    fn can_compare_two_i32_with_reversed_order() {
        let i_to_key = |x: &i32| -> i32 { -*x };
        let x32 = CmpByKey::new(32, &i_to_key);
        let y33 = CmpByKey::new(33, &i_to_key);

        assert!(x32 > y33, "should be in reversed order" );

    }

    #[test]
    fn works_with_references() {
        let i_to_key = |x: &&i32| -> i32 { -**x };

        let x32 = 32;
        let x33 = 33;

        let r32 = CmpByKey::new(&x32, &i_to_key);
        let r33 = CmpByKey::new(&x33, &i_to_key);

        assert!(r32 > r33, "should be in reversed order" );

    }

    #[test]
    fn dosnt_have_to_be_the_same_closure() {
        let x32 = CmpByKey::new(32, &|x: &i32| -> i32 { -*x });
        let y33 = CmpByKey::new(33, &|x: &i32| -> i32 { -*x });

        assert!(x32 > y33, "should be in reversed order" );
    }

    #[test]
    fn can_compare_different_types_with_same_key() {
        struct A {v: i32}
        struct B {v: i32}

        let a = CmpByKey::new(A{v:3}, &|t: &A| t.v);
        let b = CmpByKey::new(B{v:4}, &|t: &B| t.v);

        assert!(a < b)
    }
}
