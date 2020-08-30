use crate::watching::watch_exec::btree_to_string;
use std::collections::BTreeMap;

//pub fn btree_to_string(input: BTreeMap<String, String>) -> String {
#[test]
pub fn test_single_btree_to_string() {
    let mut btree = BTreeMap::new();
    btree.insert(String::from("foo"), String::from("bar"));
    let resp: String = String::from("foo=bar");
    assert_eq!(btree_to_string(btree), resp);
}
#[test]
pub fn test_multi_btree_to_string() {
    let mut btree = BTreeMap::new();
    btree.insert(String::from("foo"), String::from("bar"));
    btree.insert(String::from("baz"), String::from("bat"));
    btree.insert(String::from("ahh"), String::from("heck"));
    let resp: String = String::from("ahh=heck, baz=bat, foo=bar");
    assert_eq!(btree_to_string(btree), resp);
}
