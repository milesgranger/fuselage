use fuselage_macros::sum;


#[test]
fn df_sum() {

    #[sum("hello there")]
    fn my_simple_function() -> usize {
        4
    }

    let v = my_simple_function();
    assert_eq!(v, 4);

}