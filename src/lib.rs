//! # cmp_wrap
//!
//! Have you ever needed to compare the same data by different fields, depending on context?
//! If so, that crate is for you.


use core::cmp;

/// The main structure of this crate.
/// Lets you define a "key function" over any structure, which will change the way the value
/// is being compared. Very useful when you need to enter your data to different data structures
/// (such as max heaps, sorted trees and so on), with different comparision criteria.
pub struct CmpByKey<'k, T, K> {
    inner: T,
    key_func: &'k dyn Fn(&T) -> K
}

impl<'k, T, K> CmpByKey<'k, T,K> {
    /// Let you wrap a value with a key function which defines the way to compare it to other
    /// values.
    ///
    /// # Example
    /// ```
    /// use cmp_wrap::CmpByKey;
    /// let x = CmpByKey::new(32, &|x: &i32| -> i32 { -*x });
    /// ```
    pub fn new<'kf>(value: T, key_func: &'kf dyn Fn(&T) -> K) -> Self
        where 'kf: 'k
    {
        Self{ inner: value, key_func }
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

    fn get_key(&self) -> K {
        (self.key_func)(&self.inner)
    }
}

impl<T, K: PartialOrd> PartialOrd for CmpByKey<'_, T, K> {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        self.get_key().partial_cmp(&other.get_key())
    }
}

impl<T, K: PartialEq> PartialEq for CmpByKey<'_, T, K> {
    fn eq(&self, other: &Self) -> bool {
        self.get_key() == other.get_key()
    }
}

impl<T, K: Eq> Eq for CmpByKey<'_, T, K> {}

#[cfg(test)]
mod tests {
    use crate::CmpByKey;

    #[test]
    fn can_compare_two_i32_with_reversed_order() {
        let x32 = CmpByKey::new(32, &|x: &i32| -> i32 { -*x });
        let y33 = CmpByKey::new(33, &|x: &i32| -> i32 { -*x });

        assert!(x32 > y33, "should be in reversed order" );

    }

    #[test]
    fn works_with_references() {
        let x32 = 32;
        let x33 = 33;

        let r32 = CmpByKey::new(&x32, &|x: &&i32| -> i32 { -**x });
        let r33 = CmpByKey::new(&x33, &|x: &&i32| -> i32 { -**x });

        assert!(r32 > r33, "should be in reversed order" );

    }
}
