use serde::Deserialize;

use fuselage::{DataFrame, sum};


#[derive(Deserialize)]
pub struct Record {
    pub name: String,
    pub age: usize
}


impl Record {
    pub fn new() -> Self {
        Record { name: "Miles".to_owned(), age: 30}
    }
}


#[test]
fn basic() {
    let mut df = DataFrame::<Record>::new();
    df.push(Record::new());

    let v: usize = sum!(df, age);
    assert_eq!(v, 30);
}