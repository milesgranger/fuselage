use std::collections::HashMap;
use serde::Deserialize;
use fuselage::{DataFrame, CsvReaderBuilder, GroupBy, groupby, group_op};


// This represents a row within the dataframe
#[derive(Deserialize)]
struct Flower {
    sepal_length: f32,
    sepal_width: f32,
    petal_length: f32,
    petal_width: f32,
    species: String
}




fn main() -> Result<(), Box<std::error::Error>> {

    // Construct the reader
    let reader = CsvReaderBuilder::default()
        .delimiter(b',')
        .has_headers(true)
        .path(format!("{}/data/iris.csv", env!("CARGO_MANIFEST_DIR")))
        .build()?;

    // Read the dataframe
    let df: DataFrame<Flower> = reader.read()?;
    assert_eq!(df.len(), 150);

    // Get the groups of species
    let species_groups: GroupBy<String, &Flower> = groupby!(&df.species);

    let petal_width_sums: HashMap<&str, f32> = group_op!(&species_groups.petal_width.sum());
    assert_eq!(petal_width_sums["setosa"], 12.199999);
    assert_eq!(petal_width_sums["versicolor"], 66.299995);
    assert_eq!(petal_width_sums["virginica"], 101.30002);

    Ok(())
}
