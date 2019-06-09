use std::marker::PhantomData;
use serde::Deserialize;

pub mod series;

pub struct DataFrame<'a, T>
    where T: Deserialize<'a>
{
    pub data: Vec<T>,
    phantom: PhantomData<&'a T>
}


impl<'a, T> DataFrame<'a, T>
    where T: Deserialize<'a>
{

    pub fn new() -> Self {
        DataFrame { data: Vec::new(), phantom: PhantomData}
    }

    pub fn push(&mut self, row: T) -> () {
        self.data.push(row)
    }

}