use std::{
    iter::{DoubleEndedIterator, ExactSizeIterator, FusedIterator, Iterator},
    num::NonZeroUsize,
};

pub trait Window<'a, T: 'a>: Extend<T> {
    type OrderedIter: Iterator<Item = &'a T> + ExactSizeIterator + FusedIterator;
    type UnorderedIter: Iterator<Item = &'a T>
        + ExactSizeIterator
        + DoubleEndedIterator
        + FusedIterator;

    /// Adds a new value to the window. Possibly overwriting the oldest value
    /// if the window is fully populated.
    fn push(&mut self, value: T) -> (&mut T, T);

    /// Returns the period size of the window.
    fn period(&self) -> usize;

    /// Get the most recently added item from the window.
    fn newest(&self) -> &T;

    /// Get the oldest item from the window.
    fn oldest(&self) -> &T;

    /// Returns an iterator over the window's elements ordered
    /// from newest to oldest.
    fn iter(&'a self) -> Self::OrderedIter;

    /// Returns an iterator over the window.
    /// Unlike `iter` these items are not ordered based on their addition to the window.
    fn iter_unordered(&'a self) -> Self::UnorderedIter;
}

macro_rules! window_impl {
    () => {
        fn push(&mut self, value: T) -> (&mut T, T) {
            // SAFE: self.index is always < self.period()
            let old = std::mem::replace(unsafe { self.data.get_unchecked_mut(self.index) }, value);

            let capacity = self.period() - 1;

            // SAFE: see above
            let new = unsafe { self.data.get_unchecked_mut(self.index) };

            if self.index == capacity {
                self.index = 0;
            } else {
                self.index += 1;
            };

            (new, old)
        }

        fn newest(&self) -> &T {
            let index = self.index.checked_sub(1).unwrap_or(self.period() - 1);
            // SAFE: index is always < self.period()
            unsafe { self.data.get_unchecked(index) }
        }

        fn oldest(&self) -> &T {
            // SAFE: self.index is always < self.period()
            unsafe { self.data.get_unchecked(self.index) }
        }

        fn iter_unordered(&'a self) -> Self::UnorderedIter {
            self.data.iter()
        }
    };
}

macro_rules! window_iter_impl {
    () => {
        type Item = &'a T;

        fn next(&mut self) -> Option<Self::Item> {
            (self.size != 0).then(|| {
                self.size -= 1;
                self.index = self
                    .index
                    .checked_sub(1)
                    .unwrap_or_else(|| self.window.period() - 1);

                // SAFE: self.index is always < the slice's size
                unsafe { self.window.data.get_unchecked(self.index) }
            })
        }

        fn size_hint(&self) -> (usize, Option<usize>) {
            (self.size, Some(self.size))
        }

        fn count(self) -> usize {
            self.size
        }

        fn last(self) -> Option<Self::Item> {
            Some(self.window.oldest())
        }
    };
}

/// An array of rolling data with a dynamic (run-time) period size.
pub struct DynamicWindow<T> {
    data: Box<[T]>,
    period: usize,
    index: usize,
}

impl<T> std::iter::Extend<T> for DynamicWindow<T> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        for item in iter {
            self.push(item);
        }
    }
}

impl<'a, T: 'a> Window<'a, T> for DynamicWindow<T> {
    type OrderedIter = DynamicWindowIter<'a, T>;
    type UnorderedIter = std::slice::Iter<'a, T>;

    window_impl!();

    fn period(&self) -> usize {
        self.period
    }

    fn iter(&'a self) -> Self::OrderedIter {
        DynamicWindowIter::new(self)
    }
}

impl<T> DynamicWindow<T>
where
    T: Clone,
{
    /// Constructs a new `DynamicWindow<T>` with `period` size and initially populating
    /// with elements yielded from `closure`.
    pub fn new(closure: impl FnMut(usize) -> T, period: NonZeroUsize) -> Self {
        let period = period.get();
        Self {
            data: (0..period).map(closure).collect::<Vec<_>>().into(),
            period,
            index: 0,
        }
    }
}

pub struct DynamicWindowIter<'a, T> {
    window: &'a DynamicWindow<T>,
    index: usize,
    size: usize,
}

impl<'a, T> DynamicWindowIter<'a, T> {
    fn new(window: &'a DynamicWindow<T>) -> Self {
        Self {
            window,
            index: window.index,
            size: window.period,
        }
    }
}

impl<'a, T> Iterator for DynamicWindowIter<'a, T> {
    window_iter_impl!();
}

impl<'a, T> ExactSizeIterator for DynamicWindowIter<'a, T> {
    fn len(&self) -> usize {
        self.size
    }
}

impl<'a, T> FusedIterator for DynamicWindowIter<'a, T> {}

/// An array of rolling data with a compile-time period size.
pub struct StaticWindow<T, const PERIOD: usize> {
    data: [T; PERIOD],
    index: usize,
}

impl<T, const PERIOD: usize> std::iter::Extend<T> for StaticWindow<T, PERIOD> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        for item in iter {
            self.push(item);
        }
    }
}

impl<'a, T: 'a, const PERIOD: usize> Window<'a, T> for StaticWindow<T, PERIOD> {
    type OrderedIter = StaticWindowIter<'a, T, PERIOD>;
    type UnorderedIter = std::slice::Iter<'a, T>;

    window_impl!();

    fn period(&self) -> usize {
        PERIOD
    }

    fn iter(&'a self) -> Self::OrderedIter {
        StaticWindowIter::new(self)
    }
}

impl<T, const PERIOD: usize> StaticWindow<T, PERIOD> {
    /// Constructs a new `Window<T, PERIOD>` with `PERIOD` size and initially populating
    /// with elements yielded from `closure`.
    ///
    /// # Panics
    ///
    /// Panics if `PERIOD == 0`.
    pub fn new(closure: impl FnMut(usize) -> T) -> Self {
        if PERIOD == 0 {
            panic!("Window cannot have a period of size 0");
        }
        Self {
            data: crate::util::array(closure),
            index: 0,
        }
    }
}

pub struct StaticWindowIter<'a, T, const PERIOD: usize> {
    window: &'a StaticWindow<T, PERIOD>,
    index: usize,
    size: usize,
}

impl<'a, T, const PERIOD: usize> StaticWindowIter<'a, T, PERIOD> {
    fn new(window: &'a StaticWindow<T, PERIOD>) -> Self {
        Self {
            window,
            index: window.index,
            size: PERIOD,
        }
    }
}

impl<'a, T, const PERIOD: usize> Iterator for StaticWindowIter<'a, T, PERIOD> {
    window_iter_impl!();
}

impl<'a, T, const PERIOD: usize> ExactSizeIterator for StaticWindowIter<'a, T, PERIOD> {
    fn len(&self) -> usize {
        self.size
    }
}

impl<'a, T, const PERIOD: usize> FusedIterator for StaticWindowIter<'a, T, PERIOD> {}

#[cfg(test)]
mod test {
    use super::*;

    macro_rules! test_window {
        ($win: expr) => {
            let mut win = $win;

            let (one, prev) = win.push(1);
            assert_eq!(1, *one);
            assert_eq!(0, prev);
            assert_eq!(1, *win.newest());

            win.push(2);
            let (three, one) = win.push(3);
            assert_eq!(3, *three);
            assert_eq!(1, one);
            assert_eq!(3, *win.newest());
        };
    }

    macro_rules! test_window_iter {
        ($win: expr) => {
            let mut win = $win;
            win.push(3);
            win.push(2);
            win.push(1);

            assert_eq!(3, win.iter().count());
            assert_eq!(Some(&3), win.iter().last());

            let mut iter = win.iter();
            assert_eq!((3, Some(3)), iter.size_hint());
            assert_eq!(Some(&1), iter.next());

            assert_eq!((2, Some(2)), iter.size_hint());
            assert_eq!(Some(&2), iter.next());

            assert_eq!((1, Some(1)), iter.size_hint());
            assert_eq!(Some(&3), iter.next());

            assert_eq!((0, Some(0)), iter.size_hint());
            assert!(iter.next().is_none());
        };
    }

    #[test]
    fn test_static_window() {
        test_window!(StaticWindow::<i32, 2>::new(|_| 0));
    }

    #[test]
    fn test_static_window_iter() {
        test_window_iter!(StaticWindow::<i32, 3>::new(|_| 0));
    }

    #[test]
    #[should_panic = "Window cannot have a period of size 0"]
    fn test_static_window_panics() {
        std::panic::set_hook(Box::new(|_| {}));
        let _ = StaticWindow::<i32, 0>::new(|_| 0);
    }

    #[test]
    fn test_dynamic_window() {
        test_window!(DynamicWindow::new(|_| 0, NonZeroUsize::new(2).unwrap()));
    }

    #[test]
    fn test_dynamic_window_iter() {
        test_window_iter!(DynamicWindow::new(|_| 0, NonZeroUsize::new(3).unwrap()));
    }
}
