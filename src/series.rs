

/// Sum on a column in a DataFrame
#[macro_export]
macro_rules! sum {
    ($df:ident, $column:ident) => (
        $df.data.iter().map(|row| row.$column).sum()
    )
}

/// Map a function over a specific column of the dataframe
#[macro_export]
macro_rules! map_column {
    ($df:ident, $column:ident, $func:expr) => (
        $df.data.iter().map(|r| $func(r.$column))
    )
}

/// Map a function over the rows of the dataframe
#[macro_export]
macro_rules! map {
    ($df:ident, $func:expr) => (
        $df.data.iter().map($func)
    )
}
