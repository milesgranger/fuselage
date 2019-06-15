use fuselage_macros::sum;

#[test]
fn df_sum() {


    #[sum("hello")]
    fn my_simple_function() -> usize {
        4
    }



}