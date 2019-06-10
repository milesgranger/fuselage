use serde::Deserialize;
use std::marker::PhantomData;


pub struct DataFrame<'a, T>
    where
        T: Deserialize<'a>,
{
    pub data: Vec<T>,
    phantom: PhantomData<&'a T>,
}

impl<'a, T> DataFrame<'a, T>
    where
        T: Deserialize<'a>,
{

    /// As a slice
    pub fn as_slice(&self) -> &[T] {
        self.data.as_slice()
    }

    /// As a mutable slice
    pub fn as_mut_slice(&mut self) -> &mut [T] {
        self.data.as_mut_slice()
    }

    /// DataFrame capacity
    pub fn capacity(&self) -> usize {
        self.data.capacity()
    }

    /// Clear the dataframe
    pub fn clear(&mut self) {
        self.data.clear()
    }

    /// insert
    pub fn insert(&mut self, index: usize, row: T) {
        self.data.insert(index, row)
    }

    /// DataFrame length
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Construct an empty dataframe
    pub fn new() -> Self {
        DataFrame {
            data: Vec::new(),
            phantom: PhantomData,
        }
    }

    /// Push another row into the dataframe
    pub fn push(&mut self, row: T) {
        self.data.push(row)
    }

}


/// Construct dataframe from a `Vec<T>` where `T` implements `serde::Deserialize`
impl<'a, T> From<T> for DataFrame<'a, T::Item>
    where
        T: IntoIterator,
        T::Item: Deserialize<'a>,
{
    fn from(rows: T) -> Self {
        let mut df: DataFrame<T::Item> = DataFrame::new();
        rows.into_iter().for_each(|row| df.push(row));
        df
    }
}
