use std::collections::HashMap;

// a=1&b=2&d=&e===&d=7&d=abc
#[derive(Debug)]
pub struct QueryString<'buf> {
    data: HashMap<&'buf str, Value<'buf>>
}

#[derive(Debug)]
pub enum Value<'buf> {
    Single(&'buf str),
    Multiple(Vec<&'buf str>),
}

impl<'buf> QueryString<'buf> {
    pub fn get(&self, key: &str) -> Option<&Value> {
        self.data.get(key)
    }
}

impl<'buf> From<&'buf str> for QueryString<'buf> {
    fn from(s: &'buf str) -> Self {
        let mut data = HashMap::new();

        for sub_str in s.split('&') {
            let mut key = sub_str;
            let mut value = "";
            if let Some(i) = sub_str.find('=') {
                key = &sub_str[..i];
                value = &sub_str[i + 1..];
            }

            data.entry(key)
                .and_modify(|existring: &mut Value| match existring {
                    Value::Single(prev_val) => {
                        *existring = Value::Multiple(vec![prev_val, value]);
                    }
                    Value::Multiple(vec) => vec.push(value)
                })
                .or_insert(Value::Single(value));
        }

        QueryString { data }
    }
}
