use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
struct DataA {
    a: i32,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
struct DataB {
    b: i32,
}

trait Distinguishable {
    fn key(&self) -> String;
}

impl Distinguishable for DataA {
    fn key(&self) -> String {
        "DataA".to_string()
    }
}

impl Distinguishable for DataB {
    fn key(&self) -> String {
        "DataB".to_string()
    }
}

#[derive(Debug)]
struct Map {
    inner: serde_json::Map<String, serde_json::Value>,
}

impl Map {
    pub fn new() -> Self {
        Map {
            inner: serde_json::Map::new(),
        }
    }

    fn get<T: Distinguishable + DeserializeOwned>(&self) -> Option<Vec<T>> {
        let key = std::any::type_name::<T>().rsplit("::").nth(0).unwrap();
        match self.inner.get(key) {
            Some(value) => {
                println!("value = {:?}", value);
                // FIXME: fix it to get concrete type of value from serde::Value
                // let value_str = value;
                // let tmp = serde_json::from_str(value_str).unwrap();
                // Some(tmp)
                return None;
            }
            None => None,
        }
    }

    fn put<T: Distinguishable + Serialize>(&mut self, value: &T) {
        let key = value.key();
        if self.inner.contains_key(&key) {
            let tmp = self.inner.get_mut(&key).unwrap();
            let tmp_arr = tmp.as_array_mut().unwrap();
            tmp_arr.push(serde_json::json!(value));
        } else {
            let new_value = serde_json::json!([value]);
            self.inner.insert(key, new_value);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut map = Map::new();

        let a = DataA { a: 1 };
        let a2 = DataA { a: 2 };
        map.put(&a);
        map.put(&a2);

        let b = DataB { b: 1 };
        map.put(&b);

        let data_a = map.get::<DataA>().unwrap();

        assert_eq!(vec![a, a2], data_a);
    }
}
