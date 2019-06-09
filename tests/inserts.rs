use serde::Deserialize;

use fuselage::{DataFrame, sum, map_column, map};


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
fn basic_macros() {
    let mut df = DataFrame::<Record>::new();
    df.push(Record::new());

    let v: usize = sum!(df, age);
    assert_eq!(v, 30);

    df.push(Record::new());
    let v: usize = sum!(df, age);
    assert_eq!(v, 60);

    let v: Vec<usize> = map_column!(df, age, |n: usize| {n * 2}).collect();
    assert_eq!(v, vec![60, 60]);

    let v: Vec<usize> = map!(df, |row| {row.age * 3}).collect();
    assert_eq!(v, vec![90, 90]);
}