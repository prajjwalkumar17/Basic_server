//! Query string parsing

use std::collections::HashMap;

/// Parsed query string
#[derive(Debug)]
pub struct QueryString<'buf> {
    data: HashMap<&'buf str, Value<'buf>>,
}

/// Query string value (single or multiple)
#[derive(Debug)]
pub enum Value<'buf> {
    Single(&'buf str),
    Multiple(Vec<&'buf str>),
}

impl<'buf> QueryString<'buf> {
    /// Get a value by key
    pub fn get(&self, key: &str) -> Option<&Value> {
        self.data.get(key)
    }
}

impl<'buf> From<&'buf str> for QueryString<'buf> {
    fn from(s: &'buf str) -> Self {
        let mut data = HashMap::new();
        for sub_str in s.split('&') {
            let mut key = sub_str;
            let mut val = "";
            if let Some(i) = sub_str.find('=') {
                key = &sub_str[..i];
                val = &sub_str[i + 1..];
            }
            data.entry(key)
                .and_modify(|existing: &mut Value| {
                    match existing {
                        Value::Single(prev_val) => {
                            *existing = Value::Multiple(vec![prev_val, val])
                        }
                        Value::Multiple(vec) => vec.push(val),
                    }
                })
                .or_insert(Value::Single(val));
        }
        QueryString { data }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_query_string_single_key_value() {
        let qs: QueryString = "name=value".into();
        match qs.get("name") {
            Some(Value::Single(v)) => assert_eq!(*v, "value"),
            _ => panic!("Expected Single value"),
        }
    }

    #[test]
    fn test_query_string_multiple_key_values() {
        let qs: QueryString = "key=val1&key=val2".into();
        match qs.get("key") {
            Some(Value::Multiple(vals)) => {
                assert_eq!(vals.len(), 2);
                assert_eq!(vals[0], "val1");
                assert_eq!(vals[1], "val2");
            }
            _ => panic!("Expected Multiple values"),
        }
    }

    #[test]
    fn test_query_string_three_values_same_key() {
        let qs: QueryString = "key=a&key=b&key=c".into();
        match qs.get("key") {
            Some(Value::Multiple(vals)) => {
                assert_eq!(vals.len(), 3);
                assert_eq!(vals[0], "a");
                assert_eq!(vals[1], "b");
                assert_eq!(vals[2], "c");
            }
            _ => panic!("Expected Multiple values"),
        }
    }

    #[test]
    fn test_query_string_empty_value() {
        let qs: QueryString = "key=".into();
        match qs.get("key") {
            Some(Value::Single(v)) => assert_eq!(*v, ""),
            _ => panic!("Expected Single value"),
        }
    }

    #[test]
    fn test_query_string_key_without_value() {
        let qs: QueryString = "key".into();
        match qs.get("key") {
            Some(Value::Single(v)) => assert_eq!(*v, ""),
            _ => panic!("Expected Single value"),
        }
    }

    #[test]
    fn test_query_string_empty_string() {
        let qs: QueryString = "".into();
        // Empty string should still create a QueryString with one empty key
        assert!(qs.get("").is_some());
    }

    #[test]
    fn test_query_string_multiple_keys() {
        let qs: QueryString = "name=john&age=30".into();
        match qs.get("name") {
            Some(Value::Single(v)) => assert_eq!(*v, "john"),
            _ => panic!("Expected Single value for name"),
        }
        match qs.get("age") {
            Some(Value::Single(v)) => assert_eq!(*v, "30"),
            _ => panic!("Expected Single value for age"),
        }
    }

    #[test]
    fn test_query_string_missing_key() {
        let qs: QueryString = "name=value".into();
        assert!(qs.get("nonexistent").is_none());
    }

    #[test]
    fn test_query_string_special_characters_in_value() {
        let qs: QueryString = "url=http%3A%2F%2Fexample.com".into();
        match qs.get("url") {
            Some(Value::Single(v)) => assert_eq!(*v, "http%3A%2F%2Fexample.com"),
            _ => panic!("Expected Single value"),
        }
    }

    #[test]
    fn test_query_string_empty_key_with_value() {
        let qs: QueryString = "=value".into();
        // Empty key should still work
        match qs.get("") {
            Some(Value::Single(v)) => assert_eq!(*v, "value"),
            _ => panic!("Expected Single value"),
        }
    }
}
