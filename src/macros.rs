/// Sum on a column in a DataFrame
///
/// ## Example
///
/// ```
/// use serde::Deserialize;
/// use fuselage::{DataFrame, sum};
///
/// #[derive(Deserialize)]
/// pub struct Record {
///     pub name: String,
///     pub age: usize
/// }
///
/// impl Record {
///     pub fn new() -> Self { Record { name: "Miles".to_owned(), age: 30} }
/// }
///
/// let mut df: DataFrame<Record> = DataFrame::from(vec![Record::new(), Record::new()]);
///
/// let v: usize = sum!(df.age);
/// assert_eq!(v, 60);
/// ```
#[macro_export]
macro_rules! sum {
    ($df:ident.$column:ident) => {
        $df.data.iter().map(|row| row.$column).sum()
    };
}

/// Map a function over a specific column of the dataframe
///
/// ## Example
///
/// ```
/// use serde::Deserialize;
/// use fuselage::{DataFrame, map_column};
///
/// #[derive(Deserialize)]
/// pub struct Record {
///     pub name: String,
///     pub age: usize
/// }
///
/// impl Record {
///     pub fn new() -> Self { Record { name: "Miles".to_owned(), age: 30} }
/// }
///
/// let mut df: DataFrame<Record> = DataFrame::from(vec![Record::new(), Record::new()]);
///
/// let v: Vec<usize> = map_column!(df.age, |n: usize| { n * 2 }).collect();
/// assert_eq!(v, vec![60, 60]);
/// ```
#[macro_export]
macro_rules! map_column {
    ($df:ident.$column:ident, $func:expr) => {
        $df.data.iter().map(|r| $func(r.$column))
    };
}

/// Map a function over the rows of the dataframe
///
/// ## Example
///
/// ```
/// use serde::Deserialize;
/// use fuselage::{DataFrame, map};
///
/// #[derive(Deserialize)]
/// pub struct Record {
///     pub name: String,
///     pub age: usize
/// }
///
/// impl Record {
///     pub fn new() -> Self { Record { name: "Miles".to_owned(), age: 30} }
/// }
///
/// let mut df: DataFrame<Record> = DataFrame::from(vec![Record::new(), Record::new()]);
///
/// let v: Vec<usize> = map!(df, |row| { row.age * 3 }).collect();
/// assert_eq!(v, vec![90, 90]);
/// ```
#[macro_export]
macro_rules! map {

    // Handle reference funcs and anonymous
    ($df:ident, $func:expr) => {
        $df.data.iter().map($func)
    };
    ($df:ident, $func:ident) => {
        $df.data.iter().map($func)
    };

    // mutabable reference funcs and anonymous
    (&mut $df:ident, $func:ident) => {
        $df.data.iter_mut().map($func)
    };
    (&mut $df:ident, $func:expr) => {
        $df.data.iter_mut().map($func)
    };
}



/// Map a function over the rows of the dataframe inplace; will not
/// return anything but will update the current rows of the dataframe
///
/// ## Example
///
/// ```
/// use serde::Deserialize;
/// use fuselage::{DataFrame, map_inplace, map};
///
/// #[derive(Deserialize)]
/// pub struct Record {
///     pub name: String,
///     pub age: usize
/// }
///
/// impl Record {
///     pub fn new() -> Self { Record { name: "Miles".to_owned(), age: 30} }
/// }
///
/// let mut df: DataFrame<Record> = DataFrame::from(vec![Record::new(), Record::new()]);
///
/// let v: Vec<usize> = map!(df, |row| { row.age * 3 }).collect();
/// assert_eq!(v, vec![90, 90]);
/// ```
#[macro_export]
macro_rules! map_inplace {
    ($df:ident, $func:expr) => {
        $df.data.iter().map($func)
    };
}

