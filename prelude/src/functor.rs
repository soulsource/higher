use std::{cell::RefCell, mem::MaybeUninit, rc::Rc};

/// A `Functor` lets you change the type parameter of a generic type.
///
/// A `Functor` defines a method `fmap` on a type `F<_>: Functor` which converts
/// an `F<A>` to `F<B>` using a function `Fn(A) -> B` applied to the `A`s inside
/// it.
///
/// You can also use this just to modify the values inside your container value
/// without changing their type, if the mapping function returns a value of the
/// same type.  This is called an "endofunctor."
pub trait Functor<'a, A>
where
    A: 'a,
{
    type Target<T>
    where
        T: 'a;

    /// Map a functor of `A` to a functor of `B` using a function from `A`
    /// to `B`.
    fn fmap<B, F>(self, f: F) -> Self::Target<B>
    where
        B: 'a,
        F: Fn(A) -> B + 'a;

    /// Map the functor to the provided constant value.
    fn fconst<B>(self, b: B) -> Self::Target<B>
    where
        Self: Sized,
        B: Clone,
    {
        self.fmap(move |_| b.clone())
    }

    /// Map the functor to the unit value `()`.
    fn void(self) -> Self::Target<()>
    where
        Self: Sized,
    {
        self.fconst(())
    }

    /// Turn the functor into an iterator.
    ///
    /// ```
    /// # use higher::Functor;
    /// let my_functor = vec![1, 2, 3];
    /// let iter = my_functor.f_into_iter();
    /// let my_vec: Vec<i32> = iter.collect();
    /// assert_eq!(my_vec, vec![1, 2, 3]);
    /// ```
    fn f_into_iter(self) -> Box<dyn Iterator<Item = A>>
    where
        Self: Sized,
        A: 'static,
    {
        let store = Rc::new(RefCell::new(Vec::new()));
        let istore = store.clone();
        self.fmap(move |a| istore.borrow_mut().push(a));
        Box::new(
            match Rc::try_unwrap(store) {
                Ok(store) => store,
                Err(_) => unreachable!(),
            }
            .into_inner()
            .into_iter(),
        )
    }

    fn funzip<L: 'a, R: 'a, FL, FR, Z>(self, f: Z) -> (FL, FR)
    where
        A: 'static,
        Self: Sized + Functor<'a, A, Target<L> = FL> + Functor<'a, A, Target<R> = FR>,
        FL: Functor<'a, L, Target<A> = Self> + Default + Extend<L>,
        FR: Functor<'a, R, Target<A> = Self> + Default + Extend<R>,
        Z: Fn(A) -> (L, R) + 'a,
    {
        self.f_into_iter().map(|a| f(a)).unzip()
    }
}

impl<'a, A: 'a> Functor<'a, A> for Option<A> {
    type Target<T> = Option<T> where T: 'a;

    fn fmap<B, F>(self, f: F) -> Self::Target<B>
    where
        B: 'a,
        F: Fn(A) -> B,
    {
        self.map(f)
    }
}

impl<'a, A: 'a, E> Functor<'a, A> for Result<A, E> {
    type Target<T> = Result<T, E> where T: 'a;

    fn fmap<B, F>(self, f: F) -> Self::Target<B>
    where
        B: 'a,
        F: Fn(A) -> B,
    {
        self.map(f)
    }
}

impl<'a, A: 'a, const N: usize> Functor<'a, A> for [A; N] {
    type Target<T> = [T; N]
    where
        T: 'a;

    #[allow(unsafe_code)]
    fn fmap<B, F>(self, f: F) -> Self::Target<B>
    where
        B: 'a,
        F: Fn(A) -> B + 'a,
    {
        let mut out: MaybeUninit<[B; N]> = MaybeUninit::uninit();
        let mut ptr: *mut B = out.as_mut_ptr().cast();
        for item in self.into_iter() {
            unsafe {
                ptr.write(f(item));
                ptr = ptr.add(1);
            }
        }
        unsafe { out.assume_init() }
    }
}

macro_rules! impl_fmap_from_iter {
    () => {
        fn fmap<B, F>(self, f: F) -> Self::Target<B>
        where
            B: 'a,
            F: Fn(A) -> B,
        {
            self.into_iter().map(f).collect()
        }
    };
}

impl<'a, A: 'a> Functor<'a, A> for Vec<A> {
    type Target<T> = Vec<T> where T: 'a;
    impl_fmap_from_iter!();
}

impl<'a, A: 'a> Functor<'a, A> for std::collections::VecDeque<A> {
    type Target<T> = std::collections::VecDeque<T> where T: 'a;
    impl_fmap_from_iter!();
}

impl<'a, A: 'a> Functor<'a, A> for std::collections::LinkedList<A> {
    type Target<T> = std::collections::LinkedList<T> where T: 'a;
    impl_fmap_from_iter!();
}

#[cfg(test)]
mod test {
    use crate::Functor;

    #[test]
    fn option_functor() {
        let a = Option::Some(31337);
        let b = a.fmap(|x| x + 2);
        assert_eq!(b, Option::Some(31339));
    }

    #[test]
    fn array_endofunctor() {
        let a: [usize; 5] = [1, 2, 3, 4, 5];
        let b = a.fmap(|x| x * 2);
        assert_eq!(b, [2, 4, 6, 8, 10]);
    }

    #[test]
    fn array_exofunctor() {
        let a: [u64; 5] = [1, 2, 3, 4, 5];
        let b = a.fmap(|x| ((x * 2) as u16));
        assert_eq!(b, [2, 4, 6, 8, 10]);
    }

    #[test]
    fn vec_endofunctor() {
        let a = vec![1, 2, 3, 4, 5];
        let b = a.fmap(|x| x * 2);
        assert_eq!(b, vec![2, 4, 6, 8, 10]);
    }

    #[test]
    fn vec_exofunctor() {
        let a = vec![1, 2, 3];
        let b = a.fmap(|x| (x as usize) * 2);
        assert_eq!(b, vec![2usize, 4usize, 6usize]);
    }

    #[test]
    fn unzip() {
        let a = vec![(1usize, 2i32), (2usize, 4i32), (3usize, 6i32)];
        let (l, r) = a.funzip(std::convert::identity);
        assert_eq!(l, vec![1usize, 2usize, 3usize]);
        assert_eq!(r, vec![2i32, 4i32, 6i32]);
    }
}
