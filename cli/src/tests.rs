use crate::count_by_filepath;

#[test]
fn valid_result() {
    let res = count_by_filepath("./src/example.txt");
    assert_eq!(res.words, 10);
    assert_eq!(res.lines, 16);
    assert_eq!(res.chars, 40);
}

#[test]
#[should_panic]
fn paniced() {
    count_by_filepath("invalid_path");
}
