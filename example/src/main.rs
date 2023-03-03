use rudas::data::Series;

fn main() {
    let s = Series::from(&[1, 2, 3, 4, 5]);
    s.debug();
}
