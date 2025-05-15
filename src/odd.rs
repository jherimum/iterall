use std::ops::AddAssign;
use std::{fmt::Debug, iter::Fuse};

use num_traits::ConstOne;
use num_traits::bounds::UpperBounded;

use num_traits::Num;
use num_traits::Zero;

trait OddEven {
    fn is_odd(&self) -> bool;
    fn is_even(&self) -> bool {
        !self.is_odd()
    }
}

impl<T: Num + From<u8> + Copy> OddEven for T {
    fn is_odd(&self) -> bool {
        let two = T::from(2u8);
        !self.rem(two).is_zero()
    }
}

pub struct OddOrEvenNumbers<N> {
    current: N,
    end: Option<N>,
    odd: bool,
}

impl<N: OddEven + Num + PartialOrd + Copy + ConstOne + UpperBounded + AddAssign + From<u8>> Iterator
    for OddOrEvenNumbers<N>
{
    type Item = N;

    fn next(&mut self) -> Option<Self::Item> {
        let end = &self.end.unwrap_or(Self::Item::max_value());
        if let Some(ord) = self.current.partial_cmp(end) {
            let two = N::from(2u8);
            while ord.is_le() {
                let val = self.current;
                if val.is_odd() == self.odd {
                    self.current += two;
                    return Some(val);
                }
                self.current += ConstOne::ONE
            }
        }
        None
    }
}

#[derive(Clone, Debug)]
#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct OddEvenIterator<I> {
    iter: Fuse<I>,
    odd: bool,
}

impl<I: Iterator> OddEvenIterator<I> {
    fn new(iter: I, odd: bool) -> Self {
        Self {
            iter: iter.fuse(),
            odd,
        }
    }
}

impl<I> Iterator for OddEvenIterator<I>
where
    I: Iterator,
    I::Item: Num + Copy + ConstOne,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        let one: I::Item = ConstOne::ONE;
        let two = one + one;

        for item in &mut self.iter {
            let odd = (item % two).is_zero();
            if odd == self.odd {
                return Some(item);
            }
            continue;
        }
        None
    }
}

/// Extension trait providing the `odd()` method for iterators.
pub trait IteratorExt: Iterator {
    /// Creates an iterator that yields only the odd numbers from this iterator.
    ///
    /// # Examples
    ///
    /// ```
    /// use odd_iterator::IteratorExt;
    ///
    /// let numbers = vec![1, 2, 3, 4, 5];
    /// let odd_numbers: Vec<_> = numbers.into_iter().odd().collect();
    /// assert_eq!(odd_numbers, vec![1, 3, 5]);
    /// ```
    fn odd(self) -> OddEvenIterator<Self>
    where
        Self: Sized,
    {
        OddEvenIterator::new(self, true)
    }

    fn even(self) -> OddEvenIterator<Self>
    where
        Self: Sized,
    {
        OddEvenIterator::new(self, false)
    }
}

// Blanket implementation
impl<I: Iterator> IteratorExt for I {}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_odd_even() {
        assert!(!4.is_odd());
        assert!(5.is_odd());

        assert!(4.is_even());
        assert!(!5.is_even());
    }

    #[test]
    fn test_odd_numbers() {
        let mut odd = OddOrEvenNumbers {
            current: -1,
            end: None,
            odd: false,
        };

        assert_eq!(odd.next(), Some(0));
        assert_eq!(odd.next(), Some(2));
        assert_eq!(odd.next(), Some(4));
        assert_eq!(odd.next(), Some(6));
    }

    #[test]
    fn test_basic_odd() {
        let numbers = vec![1, 2, 3, 4, 5];
        let odd_numbers: Vec<_> = numbers.clone().into_iter().odd().collect();
        assert_eq!(odd_numbers, vec![1, 3, 5]);

        let even_numbers: Vec<_> = numbers.into_iter().even().collect();
        assert_eq!(even_numbers, vec![2, 4]);
    }
}
