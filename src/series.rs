
#[macro_export]
macro_rules! sum {
    ($df:ident, $column:ident) => (
        $df.data.iter().map(|row| row.$column).sum()
    )
}
