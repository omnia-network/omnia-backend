pub mod virtual_persona;
pub mod environment;
pub mod gateway;
pub mod errors;
pub mod http;



pub trait MapCrudOperations {
    type Entry;
    type Index;
    type Value;
    fn create(&mut self, entry: Self::Entry);
    fn read(&self, index: &Self::Index) -> Self::Value;
}

pub trait MapEntry {
    type MapIndex;
    type MapValue;
    fn get_index(&self) -> Self::MapIndex;
    fn get_value(&self) -> Self::MapValue;
}