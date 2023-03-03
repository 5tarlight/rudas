use core::panic;
use std::{any::type_name, fmt::Display};

#[derive(Debug, Clone)]
/// One-dimensional array with axis labels.
pub struct Series<T: Clone, U: PartialEq> {
    pub data: Vec<T>,
    pub label: Vec<U>,
}

impl<T: Clone, U: PartialEq> Series<T, U> {
    /// Create new [`Series`] instance with array of `T`.<br>
    /// T should implement the trait: `Clone`
    pub fn from(v: &[T]) -> Series<T, usize> {
        Series {
            data: v.to_vec(),
            label: (0..v.len()).collect(),
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
            label: l.to_vec(),
        }
    }

    /// Create new [`Series`] with same data and label of given Series.
    /// Works same as `.clone()`
    pub fn from_self(s: &Series<T, U>) -> Series<T, U>
    where
        U: Clone,
    {
        Series {
            data: s.data.clone(),
            label: s.label.clone(),
        }
    }

    /// Create new [`Series`] instance with data and default label.
    ///
    /// Works same as
    /// ```
    /// use rudas::data::Series;
    /// let s = Series::from_label(&[1, 2, 3], &[10, 20, 30]);
    /// let d = Series::from_data(&s);
    /// assert_eq!(d.label[1], 1);
    /// ```
    pub fn from_data(s: &Series<T, U>) -> Series<T, usize> {
        Series {
            data: s.data.clone(),
            label: (0..s.data.len()).collect(),
        }
    }

    /// Create new [`Series`] instance with data and new label.
    ///
    /// # Panics
    /// Length of original `Series` instance and new label should be equal.
    pub fn from_data_new_label<L: PartialEq + Clone>(s: &Series<T, U>, l: &[L]) -> Series<T, L> {
        if s.data.len() != l.len() {
            panic!("Length of data and label should be equal.");
        }

        Series {
            data: s.data.clone(),
            label: l.to_vec(),
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
            println!("{:}\t{:}", self.label[i], self.data[i]);
        }

        println!("type : {}", type_name::<T>());
    }

    /// Return the transpose, which is by definition self.
    pub fn t(&self) -> &Self {
        self.clone()
    }
}

#[cfg(test)]
mod test {
    use std::iter::zip;

    use super::Series;

    fn equal<T: Clone, U: Clone + PartialEq>(a: &Series<T, U>, b: &Series<T, U>) -> bool
    where
        T: PartialEq,
    {
        let mut equal = true;

        for (va, vb) in zip(a.data.clone(), b.data.clone()) {
            if va != vb {
                equal = false;
            }
        }

        for (va, vb) in zip(a.label.clone(), b.label.clone()) {
            if va != vb {
                equal = false;
            }
        }

        equal
    }

    #[test]
    fn should_equal() {
        let a = Series::from_label(&[1, 2, 3], &[1, 2, 3]);
        let b = Series::from_label(&[1, 2, 3], &[1, 2, 3]);

        assert!(equal(&a, &b));
    }

    #[test]
    #[should_panic]
    fn diffrent_length_of_data_and_label_should_panic() {
        Series::from_label(&[1, 2, 3], &[1]);
    }

    #[test]
    fn duplicate_self_should_equal() {
        let a = Series::from_label(&[1, 2, 3], &[1, 2, 3]);
        let b = Series::from_self(&a);

        assert!(equal(&a, &b));
    }

    #[test]
    #[should_panic]
    fn duplicate_with_differnet_length_should_panic() {
        let a = Series::<i32, usize>::from(&[1, 2, 3]);
        let _b = Series::from_data_new_label(&a, &[1]);
    }
}
