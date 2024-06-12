use std::collections::HashMap;
use itertools::{Itertools, join};

pub mod gmdfile;

pub fn deserialise_kv(input: &str, sep: &str) -> HashMap<String, String> {
    let mut map = HashMap::new();
    for (k, v) in input.split(sep).tuples() {
        map.insert(
            k.to_string(),
            v.to_string(),
        );
    }
    map
}

pub fn serialise_kv(map: &HashMap<String, String>, sep: &str) -> String {
    let joined_kvs = map.iter().map(|(k, v)| join([k, v], sep));
    
    join(joined_kvs, sep)
}

// tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialise() {
        let mut map: HashMap<String, String> = HashMap::new();
        map.insert("1".to_string(), "2".to_string());
        map.insert("3".to_string(), "4".to_string());
        map.insert("5".to_string(), "6".to_string());
        map.insert("8".to_string(), "shit".to_string());
        
        let serialised = serialise_kv(&map, ":");
        println!("{}", serialised);
        
        // there's no guarantee that the map will output in a specific order,
        // so we have to split and iterate to check
        for (k, v) in serialised.split(":").tuples() {
            if k == "1" { assert_eq!(v, "2"); }
            else if k == "3" { assert_eq!(v, "4"); }
            else if k == "5" { assert_eq!(v, "6"); }
            else if k == "8" { assert_eq!(v, "shit"); }
            else { panic!(); }
        }
    }
    #[test]
    fn deserialise() {
        let object = "1:2:3:4:5:6:8:shit";
        let map = deserialise_kv(object, ":");
        assert_eq!(map.get("1").unwrap(), "2");
        assert_eq!(map.get("3").unwrap(), "4");
        assert_eq!(map.get("5").unwrap(), "6");
        assert_eq!(map.get("8").unwrap(), "shit");
    }
}


