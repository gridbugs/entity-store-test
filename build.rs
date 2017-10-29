#[macro_use] extern crate entity_store_code_gen;

fn main() {
    generate_entity_store!("specs/simple.toml", "simple.rs");
    generate_entity_store!("specs/all_aggregates.toml", "all_aggregates.rs");
}
