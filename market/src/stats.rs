use num_traits::Zero;
use rust_decimal::{Decimal, MathematicalOps};
use std::iter::FromIterator;

pub fn stddev<I>(samples: I) -> I::Item
where
    I: Iterator,
    I::Item: StatsType,
{
    samples.collect::<Stats<I::Item>>().stddev()
}

pub fn variance<I>(samples: I) -> I::Item
where
    I: Iterator,
    I::Item: StatsType,
{
    samples.collect::<Stats<I::Item>>().variance()
}

pub fn mean<I>(samples: I) -> I::Item
where
    I: Iterator,
    I::Item: StatsType,
{
    samples.collect::<Stats<I::Item>>().mean()
}

#[derive(Debug, Clone, Default)]
pub struct Stats<T> {
    size: usize,
    mean: T,
    variance: T,
}

pub trait Sqrt: Sized {
    fn sqrt(&self) -> Option<Self>;
}

pub trait StatsType:
    Sqrt
    + std::ops::Mul<Self, Output = Self>
    + From<usize>
    + Copy
    + std::ops::Add<Self, Output = Self>
    + std::ops::Sub<Self, Output = Self>
    + std::ops::Div<Self, Output = Self>
    + Zero
{
}

impl<T> StatsType for T where
    T: Sqrt
        + std::ops::Mul<T, Output = T>
        + From<usize>
        + Copy
        + std::ops::Add<T, Output = T>
        + std::ops::Sub<T, Output = T>
        + std::ops::Div<T, Output = T>
        + Zero
{
}

impl<T> Stats<T>
where
    T: StatsType,
{
    #[must_use]
    pub fn new() -> Self {
        Self {
            size: 0,
            mean: T::zero(),
            variance: T::zero(),
        }
    }

    #[must_use]
    pub fn mean(&self) -> T {
        self.mean
    }

    #[must_use]
    pub fn variance(&self) -> T {
        self.variance
    }

    #[must_use]
    pub fn stddev(&self) -> T {
        self.variance.sqrt().unwrap()
    }

    pub fn add(&mut self, sample: T) {
        let oldmean = self.mean;
        let prevq = self.variance * T::from(self.size);

        self.size += 1;
        let size = T::from(self.size);

        self.mean = self.mean + (sample - oldmean) / size;
        self.variance = (prevq + (sample - oldmean) * (sample - self.mean)) / size;
    }

    #[must_use]
    pub fn size(&self) -> usize {
        self.size
    }
}

impl<U, T> Extend<U> for Stats<T>
where
    U: Into<T>,
    T: StatsType,
{
    fn extend<I: IntoIterator<Item = U>>(&mut self, iter: I) {
        for sample in iter {
            self.add(sample.into())
        }
    }
}

impl<U, T> FromIterator<U> for Stats<T>
where
    U: Into<T>,
    T: StatsType,
{
    fn from_iter<Iter: IntoIterator<Item = U>>(iter: Iter) -> Self {
        let mut stats = Stats::new();
        stats.extend(iter);
        stats
    }
}

// impls

impl Sqrt for Decimal {
    fn sqrt(&self) -> Option<Self> {
        <Decimal as MathematicalOps>::sqrt(self)
    }
}
