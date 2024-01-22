use kvs::KvStore;
#[test]
fn get_non_existent_value_2() {
    let mut store = KvStore::default();

    store.set("key1".to_owned(), "value1".to_owned());
    assert_eq!(store.get("key2".to_owned()), None);
}
