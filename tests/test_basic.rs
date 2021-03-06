extern crate be_generust;

use be_generust::giver;

#[giver]
fn empty() -> impl Iterator<Item = i64> {}

#[test]
fn test_empty() {
    assert_eq!(empty().collect::<Vec<_>>(), vec![]);
}

#[giver]
fn single() -> impl Iterator<Item = i64> {
    give!(1);
}

#[test]
fn test_single() {
    assert_eq!(single().collect::<Vec<_>>(), vec![1]);
}

#[giver]
fn multi() -> impl Iterator<Item = i64> {
    give!(1);
    give!(2);
    give!(3);
}

#[test]
fn test_multi() {
    assert_eq!(multi().collect::<Vec<_>>(), vec![1, 2, 3]);
}
