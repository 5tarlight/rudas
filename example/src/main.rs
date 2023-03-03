use rudas::data::Series;

fn main() {
    let s = Series::from_label(&[1, 2, 3, 4, 5], &["a", "b", "c", "d", "e"]);
    s.debug();
}
