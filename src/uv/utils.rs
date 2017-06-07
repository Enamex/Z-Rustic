use std::string;
use std::str::pattern;
use std::ops::{Index, IndexMut};

// pub trait StringExtensions {
//     fn find<'a, P>(&'a self, pat: P, start: usize) -> Option<usize>
//         where P: pattern::Pattern<'a>;
// }

// impl StringExtensions for String  {
//     fn find<'a, P>(&'a self, pat: P, start: usize) -> Option<usize>
//         where P: pattern::Pattern<'a>
//     {
//         (&self)[start..].find(pat)
//     }
// }

pub enum Either<A, B> {
    A(A),
    B(B)
}

impl<A, B> Either<A, B> {
    fn is_a(&self) -> bool {
        match *self {
            Either::A(_) => true,
            _ => false,
        }
    }
    fn is_b(&self) -> bool {
        match *self {
            Either::B(_) => true,
            _ => false,
        }
    }
    fn a<'a>(&'a self) -> &'a A {
        match self {
            &Either::A(ref a) => &a,
            _ => panic!(),
        }
    }
    fn b<'b>(&'b self) -> &'b B {
        match self {
            &Either::B(ref b) => &b,
            _ => panic!(),
        }
    }
}

pub mod ascii_string {
    use super::*;
    use std::collections::range::RangeArgument;
    use std::ops::{Range, RangeFrom, RangeFull, RangeTo};
    use std::slice::SliceIndex;

    pub trait Bounds<T> {
        fn bounds(&self, s: T, e: T) -> (T, T);
        fn clamp(&self, s: T, e: T) -> Range<T> {
            let x = self.bounds(s, e);
            Range{ start: x.0, end: x.1 }
        }
    }

    impl Bounds<usize> for RangeFrom<usize> {
        fn bounds(&self, s: usize, e: usize) -> (usize, usize) {
            let start = self.start;
            let end = e;
            (start, end)
        }
    }
    impl Bounds<usize> for RangeTo<usize> {
        fn bounds(&self, s: usize, e: usize) -> (usize, usize) {
            let start = s;
            let end = self.end;
            (start, end)
        }
    }
    impl Bounds<usize> for Range<usize> {
        fn bounds(&self, s: usize, e: usize) -> (usize, usize) {
            let start = self.start;
            let end = self.end;
            (start, end)
        }
    }
    impl Bounds<usize> for RangeFull {
        fn bounds(&self, s: usize, e: usize) -> (usize, usize) {
            let start = s;
            let end = e;
            (start, end)
        }
    }

    pub trait StringExt {
        fn find_in<'a, P, R>(&'a self, r: R, pat: P) -> Option<usize>
            where
                P: pattern::Pattern<'a>,
                Self: Index<R, Output=str>,
                R: Bounds<usize> + Clone;
        fn put<R, S>(&mut self, r: R, src: S)
            where
                Self: Index<R, Output=str>,
                S: AsRef<str>,
                R: Clone + SliceIndex<[u8], Output=[u8]>
                ;
    }

    impl StringExt for String {
        fn find_in<'a, P, R>(&'a self, r: R, pat: P) -> Option<usize>
            where
                P: pattern::Pattern<'a>,
                Self: Index<R, Output=str>,
                R: Bounds<usize> + Clone
        {
            let sub = &self[r.clone()];
            if let Some(i) = sub.find(pat) {
                let bounds = Bounds::bounds(&r, 0, self.len());
                Some(bounds.0 + i)
            }
            else {
                None
            }
        }

        fn put<R, S>(&mut self, r: R, src: S)
            where
                Self: Index<R, Output=str>,
                S: AsRef<str>,
                R: Clone + SliceIndex<[u8], Output=[u8]>
        {
            // use std::mem::transmute;

            let src: &str = src.as_ref();
            let sslice = unsafe {
                &mut (*(self.as_mut_str() as *mut str as *mut [u8]))[r] };
            assert_eq!(sslice.len(), src.len());

            sslice.clone_from_slice(src.as_bytes());
        }
    }

    impl StringExt for str {
        fn find_in<'a, P, R>(&'a self, r: R, pat: P) -> Option<usize>
            where
                P: pattern::Pattern<'a>,
                Self: Index<R, Output=str>,
                R: Bounds<usize> + Clone
        {
            let sub = &self[r.clone()];
            let bounds = Bounds::bounds(&r, 0, self.len());
            sub.find(pat).map(|i| bounds.0 + i)
        }

        fn put<R, S>(&mut self, r: R, src: S)
            where
                Self: Index<R, Output=str>,
                S: AsRef<str>,
                R: Clone + SliceIndex<[u8], Output=[u8]>
        {
            // use std::mem::transmute;

            let src = src.as_ref();
            // let bounds = Bounds::clamp(&r, 0, src.len());
            let sslice = unsafe {
                &mut (*(self as *mut str as *mut [u8]))[r] };
                // &mut *(self.slice_mut_unchecked(bounds.0, bounds.1) as *mut str as *mut [u8]) };
            assert_eq!(sslice.len(), src.len());

            sslice.clone_from_slice(src.as_bytes());
        }
    }

    pub trait Test {
        fn foo<R, S>(&mut self, r: R, src: S)
            where
                Self: Index<R, Output=str>,
                S: AsRef<str>,
                ;
    }

    impl<'s> Test for str {
        fn foo<R, S>(&mut self, r: R, src: S)
            where
                S: AsRef<str>,
                Self: Index<R, Output=str>
        {}
    }
    impl<'s> Test for String {
        fn foo<R, S>(&mut self, r: R, src: S)
            where
                S: AsRef<str>,
                Self: Index<R, Output=str>
        {}
    }
}