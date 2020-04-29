pub struct KvStore;

impl KvStore {
    pub fn new() -> KvStore {
        KvStore
    }

    pub fn set(&mut self, k : String, v : String) {
        panic!()
    }

    pub fn get(&self, k : String) -> Option<String> {
        panic!()
    }

    pub fn remove(&mut self, k : String) {
        panic!()
    }
}