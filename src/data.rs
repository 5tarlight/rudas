use std::{any::type_name, fmt::Display};

#[derive(Debug, Clone)]
pub struct Series<T: Clone>(Vec<T>);

impl<T: Clone> Series<T> {
    pub fn from(v: &[T]) -> Series<T> {
        Series(v.to_vec())
    }

    pub fn debug(&self)
    where
        T: Display,
    {
        for (i, v) in self.0.iter().enumerate() {
            println!("{}\t{:}", i, v);
        }
        println!("type : {}", type_name::<T>());
    }
}
