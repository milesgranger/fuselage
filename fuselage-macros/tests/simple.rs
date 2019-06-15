use fuselage_macros::sum;

#[test]
fn df_sum() {

    #[sum("hello there")]
    fn my_simple_function() -> usize {
        4
    }

    my_simple_function();

}