#![allow(dead_code)]

use serde::Deserialize;
use fuselage::{DataFrame, sum};


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

fn main() {
    let cart = DataFrame::from(
        vec![
            Product::new(1, "basketball", 20),
            Product::new(2, "apple sauce", 2),
            Product::new(2, "apple sauce", 2),
        ]
    );

    let _total: u32 = sum!(&cart.price);
}
