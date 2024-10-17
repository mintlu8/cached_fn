//! Cache a function's result in a map.
//!
//! # Example
//!
//! ```rust
//! # use cached_fn::cached;
//! let mut cached_sqr = cached(|x| x * x);
//! assert_eq!(cached_sqr(3), 9);
//! ```
use rustc_hash::FxHashMap;
use std::{collections::BTreeMap, hash::Hash};

/// Transform a function by caching its result in a hashmap.
pub fn cached<'t, A: Clone + Hash + Eq + 't, B: Clone + 't>(
    mut f: impl FnMut(A) -> B + 't,
) -> impl FnMut(A) -> B + 't {
    let mut map = FxHashMap::<A, B>::default();
    move |x| {
        if let Some(val) = map.get(&x) {
            val.clone()
        } else {
            let val = f(x.clone());
            map.insert(x, val.clone());
            val
        }
    }
}

/// Transform a function by caching its result in a [`BTreeMap`].
pub fn cached_ord<'t, A: Clone + Ord + Eq + 't, B: Clone + 't>(
    mut f: impl FnMut(A) -> B + 't,
) -> impl FnMut(A) -> B + 't {
    let mut map = BTreeMap::<A, B>::default();
    move |x| {
        if let Some(val) = map.get(&x) {
            val.clone()
        } else {
            let val = f(x.clone());
            map.insert(x, val.clone());
            val
        }
    }
}

#[cfg(test)]
mod test {
    use std::sync::atomic::{AtomicU32, Ordering};

    use crate::cached;

    #[test]
    fn test() {
        let calls = AtomicU32::new(0);
        let mut cached_sqr = cached(|x| {
            calls.fetch_add(1, Ordering::Relaxed);
            x * x
        });
        assert_eq!(cached_sqr(1), 1);
        assert_eq!(calls.load(Ordering::Relaxed), 1);
        assert_eq!(cached_sqr(2), 4);
        assert_eq!(calls.load(Ordering::Relaxed), 2);
        assert_eq!(cached_sqr(3), 9);
        assert_eq!(calls.load(Ordering::Relaxed), 3);
        assert_eq!(cached_sqr(2), 4);
        assert_eq!(calls.load(Ordering::Relaxed), 3);
        assert_eq!(cached_sqr(1), 1);
        assert_eq!(calls.load(Ordering::Relaxed), 3);
        assert_eq!(cached_sqr(4), 16);
        assert_eq!(calls.load(Ordering::Relaxed), 4);
    }
}
