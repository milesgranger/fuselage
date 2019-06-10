#![allow(dead_code)]

use serde::Deserialize;
use fuselage::{DataFrame, sum, map};


#[derive(Deserialize)]
struct Product {
    id: u32,
    name: String,
    price: u32
}

impl Product {
    pub fn new(id: u32, name: &str, price: u32) -> Self {
        Product { id, name: name.to_owned(), price }
    }
}

fn _basic_cart() -> DataFrame<'static, Product> {
    let cart = DataFrame::from(
        vec![
            Product::new(1, "basketball", 20),
            Product::new(2, "apple sauce", 2),
            Product::new(2, "apple sauce", 2),
        ]
    );
    cart
}


#[test]
fn sum() {
    let cart = _basic_cart();
    let total: u32 = sum!(cart.price);
    assert_eq!(total, 24);
}

#[test]
fn map() {
    let cart = _basic_cart();
    let new_prices: Vec<u32> = map!(&cart, |row| row.price * 2).collect();
    assert_eq!(new_prices, vec![40, 4, 4]);

    // map a mutable ref function
    let mut cart = _basic_cart();
    fn mutate(product: &mut Product) {
        product.price *= 2;
    }
    let _: Vec<()> = map!(&mut cart, mutate).collect();
    let new_total: u32 = sum!(cart.price);
    assert_eq!(new_total, 48);


    // Map a mutable ref inline
    let mut cart = _basic_cart();
    let _: Vec<()> = map!(&mut cart, |mut row| {row.price += 2;}).collect();
    let new_total: u32 = sum!(cart.price);
    assert_eq!(new_total, 30);

}
