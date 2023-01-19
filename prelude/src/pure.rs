/// `Pure` lets you construct a value of type `F<A>` from a single value of
/// `A`.
///
/// For instance:
///
/// * `Option::pure(5)` returns `Some(5)`.
/// * `Result::pure(5)` returns `Ok(5)`.
/// * `Vec::pure(5)` returns `vec![5]`.
pub trait Pure<A> {
    fn pure(value: A) -> Self;
}

impl<A> Pure<A> for Option<A> {
    fn pure(value: A) -> Self {
        Some(value)
    }
}

impl<A, E> Pure<A> for Result<A, E> {
    fn pure(value: A) -> Self {
        Ok(value)
    }
}

#[cfg(feature = "std")]
impl<A> Pure<A> for Vec<A> {
    fn pure(value: A) -> Self {
        vec![value]
    }
}

#[cfg(feature = "std")]
impl<A> Pure<A> for std::collections::VecDeque<A> {
    fn pure(value: A) -> Self {
        Self::from_iter([value])
    }
}

#[cfg(feature = "std")]
impl<A> Pure<A> for std::collections::LinkedList<A> {
    fn pure(value: A) -> Self {
        Self::from_iter([value])
    }
}

#[cfg(feature = "std")]
impl<A> Pure<A> for std::collections::BinaryHeap<A>
where
    A: Ord,
{
    fn pure(value: A) -> Self {
        Self::from_iter([value])
    }
}

#[cfg(feature = "std")]
impl<A> Pure<A> for std::collections::HashSet<A>
where
    A: std::hash::Hash + Eq,
{
    fn pure(value: A) -> Self {
        Self::from_iter([value])
    }
}

#[cfg(feature = "std")]
impl<A> Pure<A> for std::collections::BTreeSet<A>
where
    A: Ord,
{
    fn pure(value: A) -> Self {
        Self::from_iter([value])
    }
}

#[cfg(feature = "std")]
impl<K, V> Pure<(K, V)> for std::collections::HashMap<K, V>
where
    K: std::hash::Hash + Eq,
{
    fn pure(value: (K, V)) -> Self {
        Self::from_iter([value])
    }
}

#[cfg(feature = "std")]
impl<K, V> Pure<(K, V)> for std::collections::BTreeMap<K, V>
where
    K: Ord,
{
    fn pure(value: (K, V)) -> Self {
        Self::from_iter([value])
    }
}

#[cfg(test)]
mod test {
    use crate::{Functor, Pure};

    #[test]
    fn pure_vec() {
        let a = Vec::pure(31337);
        assert_eq!(a, vec![31337]);
        let b = a.fmap(|x| x.to_string());
        assert_eq!(b, vec!["31337".to_string()]);
    }
}
