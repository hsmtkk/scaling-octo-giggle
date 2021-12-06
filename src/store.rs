use std::collections::HashMap;

pub struct URLStore {
    store: HashMap<String, String>,
}

const ID_LENGTH: usize = 8;

impl URLStore {
    pub fn new() -> URLStore {
        let store: HashMap<String, String> = HashMap::new();
        URLStore { store }
    }

    pub fn short(&mut self, url: &str) -> String {
        let id = nanoid::nanoid!(ID_LENGTH);
        self.store.insert(id.clone(), url.to_string());
        id
    }

    pub fn expand(&self, id: &str) -> Option<String> {
        match self.store.get(id) {
            Some(url) => Some(url.clone()),
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::URLStore;
    #[test]
    fn test_store() {
        let mut store = super::URLStore::new();
        let orig = "http://www.example.com";
        let id = store.short(orig);
        let expanded = store.expand(&id).unwrap();
        assert_eq!(orig, expanded);
    }
}
