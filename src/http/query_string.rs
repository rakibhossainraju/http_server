use std::collections::HashMap;

#[derive(Debug)]
pub enum Value<'buff> {
    Single(&'buff str),
    Multiple(Vec<&'buff str>),
}
#[derive(Debug)]
pub struct QueryString<'buff> {
    data: HashMap<&'buff str, Value<'buff>>,
}

impl<'buff> From<&'buff str> for QueryString<'buff> {
    fn from(s: &'buff str) -> Self {
        let mut data = HashMap::new();

        for sub_str in s.split('&') {
            if let Some((key, value)) = sub_str.split_once('=') {
                data.entry(key)
                    .and_modify(|existing: &mut Value| match existing {
                        Value::Single(prev_val) => *existing = Value::Multiple(vec![*prev_val, value]),
                        Value::Multiple(vec) => vec.push(value),
                    })
                    .or_insert(Value::Single(value));
            }
        }

        Self { data }
    }
}