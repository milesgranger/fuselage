use std::marker::PhantomData;
use serde::Deserialize;


pub struct DataFrame<'a, T>
    where T: Deserialize<'a>
{
    pub data: Vec<T>,
    phantom: PhantomData<&'a T>
}

#[macro_export]
macro_rules! sum {
    ($df:ident, $column:ident) => (
        $df.data.iter().map(|row| row.$column).sum()
    )
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