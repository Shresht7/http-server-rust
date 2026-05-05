use std::collections::HashMap;

// ------------
// QUERY PARAMS
// ------------

/// A struct representing the query parameters of a URI.
/// The `QueryParams` struct provides methods to insert, retrieve, set, and remove query parameters, as well as to parse a query string into key-value pairs.
/// It maintains the order of keys as they were inserted.
///
/// It derefs to a `HashMap<String, String>` for convenient access to the query parameters as a standard map.
///
/// # Examples
///
/// ```
/// use lib::uri::QueryParams;
/// use std::str::FromStr;
///
/// let query_str = "q=rust&sort=desc&empty=&novalue";
/// let query_params = QueryParams::from_str(query_str).unwrap();
///
/// assert_eq!(query_params.get("q"), Some(&"rust".to_string()));
/// assert_eq!(query_params.get("sort"), Some(&"desc".to_string()));
/// assert_eq!(query_params.get("empty"), Some(&"".to_string()));
/// assert_eq!(query_params.get("novalue"), Some(&"".to_string()));
/// ```
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct QueryParams {
    ordered_keys: Vec<String>,
    params: HashMap<String, String>,
}

impl QueryParams {
    pub fn new() -> Self {
        Self {
            ordered_keys: Vec::new(),
            params: HashMap::new(),
        }
    }
}

// From
// ----

impl From<HashMap<String, String>> for QueryParams {
    fn from(params: HashMap<String, String>) -> Self {
        let ordered_keys = params.keys().cloned().collect();
        Self {
            ordered_keys,
            params,
        }
    }
}

impl From<Vec<(String, String)>> for QueryParams {
    fn from(pairs: Vec<(String, String)>) -> Self {
        let mut params = HashMap::new();
        let mut ordered_keys = Vec::new();

        for (key, value) in pairs {
            if !params.contains_key(&key) {
                ordered_keys.push(key.clone());
            }
            params.insert(key, value);
        }

        Self {
            ordered_keys,
            params,
        }
    }
}

// INTERFACE
// ---------

impl QueryParams {
    pub fn insert(&mut self, key: String, value: String) {
        if !self.params.contains_key(&key) {
            self.ordered_keys.push(key.clone());
            self.params.insert(key, value);
        }
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        self.params.get(key)
    }

    pub fn set(&mut self, key: String, value: String) -> &mut Self {
        if !self.params.contains_key(&key) {
            self.ordered_keys.push(key.clone());
        }
        self.params.insert(key, value);
        self
    }

    pub fn remove(&mut self, key: &str) -> Option<String> {
        if self.params.contains_key(key) {
            self.ordered_keys.retain(|k| k != key);
        }
        self.params.remove(key)
    }
}

// IntoIterator
// ------------

impl IntoIterator for QueryParams {
    type Item = (String, String);
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.ordered_keys
            .into_iter()
            .filter_map(|key| self.params.get(&key).cloned().map(|value| (key, value)))
            .collect::<Vec<(String, String)>>()
            .into_iter()
    }
}

// Display
// -------

impl std::fmt::Display for QueryParams {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let result = self
            .ordered_keys
            .iter()
            .filter_map(|key| {
                self.params
                    .get(key)
                    .map(|value| format!("{}={}", key, value))
            })
            .collect::<Vec<String>>()
            .join("&");
        write!(f, "?{}", result)
    }
}

// FromStr
// -------

impl std::str::FromStr for QueryParams {
    type Err = ParseQueryParamError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // If the query string is empty, we can return an empty QueryParams struct
        if s.is_empty() {
            return Ok(QueryParams::new());
        }

        // Otherwise, we need to parse the query string into key-value pairs.
        // The query string is typically formatted as a series of key-value pairs separated by '&',
        // where each key and value are separated by '='. For example: "key1=value1&key2=value2".

        // Instantiating a new HashMap to store the parsed query parameters
        let mut params = QueryParams::new();

        // Split the query string on `&` to get individual key-value pairs
        for param in s.split('&') {
            let mut parts = param.splitn(2, '=');
            if let Some(key) = parts.next() {
                let value = parts.next().unwrap_or("");
                params.insert(key.to_string(), value.to_string());
            } else {
                return Err(ParseQueryParamError::NoKey(param.to_string()));
            }
        }

        Ok(params)
    }
}

#[derive(Debug)]
pub enum ParseQueryParamError {
    NoKey(String),
}

impl std::fmt::Display for ParseQueryParamError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseQueryParamError::NoKey(param) => {
                write!(f, "No key found in query parameter: '{}'", param)
            }
        }
    }
}

impl std::error::Error for ParseQueryParamError {}

// Deref
// -----

impl std::ops::Deref for QueryParams {
    type Target = HashMap<String, String>;

    fn deref(&self) -> &Self::Target {
        &self.params
    }
}

// -----
// TESTS
// -----

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn should_instantiate_from_hashmap() {
        let mut map = HashMap::new();
        map.insert("key1".to_string(), "value1".to_string());
        map.insert("key2".to_string(), "value2".to_string());

        let query_params = QueryParams::from(map);
        assert_eq!(query_params.get("key1"), Some(&"value1".to_string()));
        assert_eq!(query_params.get("key2"), Some(&"value2".to_string()));
    }

    #[test]
    fn should_instantiate_from_vec() {
        let pairs = vec![
            ("key1".to_string(), "value1".to_string()),
            ("key2".to_string(), "value2".to_string()),
        ];

        let query_params = QueryParams::from(pairs);
        assert_eq!(query_params.get("key1"), Some(&"value1".to_string()));
        assert_eq!(query_params.get("key2"), Some(&"value2".to_string()));
    }

    #[test]
    fn should_create_correct_query_param_string() {
        let mut query_params = QueryParams::new();
        query_params.insert("q".to_string(), "rust".to_string());
        query_params.insert("sort".to_string(), "desc".to_string());
        assert_eq!(query_params.to_string(), "?q=rust&sort=desc");
    }

    #[test]
    fn should_parse_query_params() {
        let query_str = "q=rust&sort=desc&empty=&novalue";
        let query_params = QueryParams::from_str(query_str).unwrap();

        assert_eq!(query_params.get("q"), Some(&"rust".to_string()));
        assert_eq!(query_params.get("sort"), Some(&"desc".to_string()));
        assert_eq!(query_params.get("empty"), Some(&"".to_string()));
        assert_eq!(query_params.get("novalue"), Some(&"".to_string()));
    }

    #[test]
    fn should_handle_empty_query_string() {
        let query_str = "";
        let query_params = QueryParams::from_str(query_str).unwrap();
        assert!(query_params.is_empty());
    }

    #[test]
    fn should_handle_key_without_value() {
        let query_str = "key";
        let query_params = QueryParams::from_str(query_str).unwrap();
        assert_eq!(query_params.get("key"), Some(&"".to_string()));
    }

    #[test]
    fn should_handle_key_with_empty_value() {
        let query_str = "key=";
        let query_params = QueryParams::from_str(query_str).unwrap();
        assert_eq!(query_params.get("key"), Some(&"".to_string()));
    }

    #[test]
    fn should_handle_multiple_equals_in_value() {
        let query_str = "key=value=with=equals";
        let query_params = QueryParams::from_str(query_str).unwrap();
        assert_eq!(
            query_params.get("key"),
            Some(&"value=with=equals".to_string())
        );
    }

    #[test]
    fn should_handle_duplicate_keys() {
        let query_str = "key=value1&key=value2";
        let query_params = QueryParams::from_str(query_str).unwrap();
        assert_eq!(query_params.get("key"), Some(&"value1".to_string()));
    }

    #[test]
    fn should_allow_adding_new_key_value_pairs() {
        let mut query_params = QueryParams::new();
        query_params.insert("key1".to_string(), "value1".to_string());
        query_params.insert("key2".to_string(), "value2".to_string());

        assert_eq!(query_params.get("key1"), Some(&"value1".to_string()));
        assert_eq!(query_params.get("key2"), Some(&"value2".to_string()));
    }

    #[test]
    fn should_allow_chaining_set_calls() {
        let mut query_params = QueryParams::new();
        query_params
            .set("key1".to_string(), "value1".to_string())
            .set("key2".to_string(), "value2".to_string());

        assert_eq!(query_params.get("key1"), Some(&"value1".to_string()));
        assert_eq!(query_params.get("key2"), Some(&"value2".to_string()));
    }
}
