use serde_json::Value;

#[derive(Default, Clone, PartialEq, Eq, Hash)]
pub enum JsonSelector {
    ObjectKey(String),
    ArrayIndex(usize),
    #[default]
    None,
}

impl JsonSelector {
    fn apply<'v>(&self, root: &'v Value) -> Option<&'v Value> {
        match (root, self) {
            (Value::Object(object), Self::ObjectKey(key)) => object.get(key),
            (Value::Array(array), Self::ArrayIndex(index)) => array.get(*index),
            _ => None,
        }
    }

    pub fn get_selection<'a>(root: &'a Value, selector: &[Self]) -> Option<&'a Value> {
        let mut current = root;
        for select in selector {
            current = select.apply(current)?;
        }
        Some(current)
    }
}

impl std::fmt::Display for JsonSelector {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ObjectKey(key) => fmt.write_str(key),
            Self::ArrayIndex(index) => fmt.write_str(&index.to_string()),
            Self::None => Ok(()),
        }
    }
}

#[test]
fn can_not_get_other_value() {
    let root = Value::Bool(false);
    let result = JsonSelector::ArrayIndex(2).apply(&root);
    assert_eq!(result, None);
}

#[test]
fn can_get_nth_array_value() {
    let root = Value::Array(vec![Value::String("bla".to_owned()), Value::Bool(true)]);
    let result = JsonSelector::ArrayIndex(1).apply(&root);
    assert_eq!(result, Some(&Value::Bool(true)));
}

#[test]
fn can_not_get_array_index_out_of_range() {
    let root = Value::Array(vec![Value::String("bla".to_owned()), Value::Bool(true)]);
    let result = JsonSelector::ArrayIndex(42).apply(&root);
    assert_eq!(result, None);
}

#[test]
fn can_get_object_value() {
    let mut object = serde_json::Map::new();
    object.insert("bla".to_owned(), Value::Bool(false));
    object.insert("blubb".to_owned(), Value::Bool(true));
    let root = Value::Object(object);
    let result = JsonSelector::ObjectKey("blubb".to_owned()).apply(&root);
    assert_eq!(result, Some(&Value::Bool(true)));
}

#[test]
fn can_not_get_object_missing_key() {
    let mut object = serde_json::Map::new();
    object.insert("bla".to_owned(), Value::Bool(false));
    object.insert("blubb".to_owned(), Value::Bool(true));
    let root = Value::Object(object);
    let result = JsonSelector::ObjectKey("foo".to_owned()).apply(&root);
    assert_eq!(result, None);
}

#[test]
fn can_not_get_object_by_index() {
    let mut object = serde_json::Map::new();
    object.insert("bla".to_owned(), Value::Bool(false));
    object.insert("blubb".to_owned(), Value::Bool(true));
    let root = Value::Object(object);
    let result = JsonSelector::ArrayIndex(42).apply(&root);
    assert_eq!(result, None);
}

#[test]
fn can_get_selected_value() {
    let mut inner = serde_json::Map::new();
    inner.insert("bla".to_owned(), Value::Bool(false));
    inner.insert("blubb".to_owned(), Value::Bool(true));

    let root = Value::Array(vec![
        Value::Bool(false),
        Value::Object(inner),
        Value::Bool(false),
    ]);

    let selector = vec![
        JsonSelector::ArrayIndex(1),
        JsonSelector::ObjectKey("blubb".to_owned()),
    ];

    let result = JsonSelector::get_selection(&root, &selector);
    assert_eq!(result, Some(&Value::Bool(true)));
}