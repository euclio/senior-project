extern crate generic_array_list;

use generic_array_list::ArrayList;

// Empty list tests
#[test]
fn test_empty() {
    let list: ArrayList<i32> = ArrayList::new();
    assert_eq!(list.head(), None);
    assert_eq!(list.size(), 0);
}

#[test]
fn test_add_empty() {
    let mut list: ArrayList<i32> = ArrayList::new();
    list.insert(0, 2);
    assert_eq!(list.size(), 1);
    assert_eq!(list.head(), Some(2));
}

// Single element tests
#[test]
fn test_remove_one() {
    let mut list: ArrayList<i32> = ArrayList::new();
    list.insert(0, 47);
    assert_eq!(list.size(), 1);
    assert_eq!(list.remove(0), 47);
    assert_eq!(list.size(), 0);
}

#[test]
fn test_get_one() {
    let mut list: ArrayList<i32> = ArrayList::new();
    list.insert(0, 42);
    assert_eq!(list.get(0), 42);
    assert_eq!(list.size(), 1);
}

// Adding multiple elements
#[test]
fn test_add_two() {
    let mut list: ArrayList<i32> = ArrayList::new();
    list.insert(0, 1);
    list.insert(1, 2);
    assert_eq!(list.size(), 2);
    assert_eq!(list.head(), Some(1));
}

#[test]
fn test_remove_two() {
    let mut list: ArrayList<i32> = ArrayList::new();
    list.insert(0, 1);
    list.insert(1, 2);
    assert_eq!(list.remove(1), 2);
    assert_eq!(list.remove(0), 1);
}
