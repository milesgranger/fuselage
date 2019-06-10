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
/// use fuselage::{DataFrame, map_column, sum};
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
/// // Map function which will take values of the column by reference
/// let v: Vec<usize> = map_column!(&df.age, |n: &usize| { *n * 2 }).collect();
/// assert_eq!(v, vec![60, 60]);
///
/// // Map function which will take values of the column by mutable reference
/// let _: Vec<()> = map_column!(&mut df.age, |n: &mut usize| { *n *= 2 }).collect();
/// let v: usize = sum!(df.age);
/// assert_eq!(v, 120);
///
/// // Map function which will take values of the column by owned values
/// // this also consumes the dataframe!
/// let v: Vec<usize> = map_column!(df.age, |n: usize| { n }).collect();
/// assert_eq!(v, vec![60, 60]);
/// ```
#[macro_export]
macro_rules! map_column {

    // funcs & expressions taking values by refrence
    (&$df:ident.$column:ident, $func:expr) => {
        $df.data.iter().map(|r| $func(&r.$column))
    };
    (&$df:ident.$column:ident, $func:ident) => {
        $df.data.iter().map(|r| $func(&r.$column))
    };

    // funcs & expressions taking values by mutable reference
    (&mut $df:ident.$column:ident, $func:expr) => {
        $df.data.iter_mut().map(|r| $func(&mut r.$column))
    };
    (&mut $df:ident.$column:ident, $func:ident) => {
        $df.data.iter_mut().map(|r| $func(&mut r.$column))
    };

    // funcs & expressions taking owned values
    ($df:ident.$column:ident, $func:expr) => {
        $df.data.into_iter().map(|r| $func(r.$column))
    };
    ($df:ident.$column:ident, $func:ident) => {
        $df.data.into_iter().map(|r| $func(r.$column))
    };
}

/// Map a function over the rows of the dataframe
///
/// ## Example
///
/// ```
/// use serde::Deserialize;
/// use fuselage::{DataFrame, map, sum};
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
/// // Map a function which takes a reference:
/// let v: Vec<usize> = map!(&df, |row| { row.age * 3 }).collect();
/// assert_eq!(v, vec![90, 90]);
///
/// // Map a function which takes a mutable reference:
/// let _: Vec<()> = map!(&mut df, |row| { row.age *= 3 }).collect();
/// let new_sum: usize = sum!(df.age);
/// assert_eq!(new_sum, 180);
///
/// // Map a function which takes ownership of `df`'s rows:
/// // Note: should be 90,90 since we multiplied it inplace above.
/// let v: Vec<usize> = map!(df, |row| { row.age }).collect();
/// assert_eq!(v, vec![90, 90]);
/// ```
#[macro_export]
macro_rules! map {

    // Handle reference funcs and anonymous
    (&$df:ident, $func:expr) => {
        $df.data.iter().map($func)
    };
    (&$df:ident, $func:ident) => {
        $df.data.iter().map($func)
    };

    // mutabable reference funcs and anonymous
    (&mut $df:ident, $func:ident) => {
        $df.data.iter_mut().map($func)
    };
    (&mut $df:ident, $func:expr) => {
        $df.data.iter_mut().map($func)
    };

    // by owned value funcs and anonymous
    ($df:ident, $func:ident) => {
        $df.data.into_iter().map($func)
    };
    ($df:ident, $func:expr) => {
        $df.data.into_iter().map($func)
    };

}
