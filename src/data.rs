use std::{any::type_name, fmt::Display};

#[derive(Debug, Clone)]
/// One-dimensional array with axis labels.
pub struct Series<T: Clone, U: PartialEq> {
    data: Vec<T>,
    labels: Vec<U>,
}

impl<T: Clone, U: PartialEq> Series<T, U> {
    /// Create new [`Series`] instance with array of `T`.<br>
    /// T should implement the trait: `Clone`
    pub fn from(v: &[T]) -> Series<T, usize> {
        Series {
            data: v.to_vec(),
            labels: (0..v.len()).collect(),
        }
    }

    /// Create new [`Series`] instance with axis labels.
    ///
    /// # Panics
    /// When length of given data and label is not equal, it panics.
    pub fn from_label(v: &[T], l: &[U]) -> Series<T, U>
    where
        U: Clone,
    {
        if v.len() != l.len() {
            panic!("Length of data and label should be equal.");
        }

        Series {
            data: v.to_vec(),
            labels: l.to_vec(),
        }
    }

    /// Display all values with indexes.<br>
    /// This is available when T and U has a trait [Display].
    ///
    /// ```
    /// use rudas::data::Series;
    /// let s = Series::<i32, usize>::from(&[1, 2, 3, 4, 5]);
    /// s.debug();
    /// ```
    pub fn debug(&self)
    where
        T: Display,
        U: Display,
    {
        for i in 0..self.data.len() {
            println!("{:}\t{:}", self.labels[i], self.data[i]);
        }

        println!("type : {}", type_name::<T>());
    }

    /// Return the transpose, which is by definition self.
    pub fn t(&self) -> &Self {
        self.clone()
    }
}

#[cfg(test)]
mod test {}
