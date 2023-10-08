use std::collections::HashMap;

//Lifetime will be same as buffer
pub struct QueryString<'life_buf>{
    data: HashMap<&'life_buf str, Value<'life_buf>>
}

pub enum Value<'life_buf> {
    Single(&'life_buf str),
    Multiple(Vec<&'life_buf str>)
}

//Function to return strings from hashmap
impl<'life_buf> QueryString <'life_buf> {
    pub fn get(&self,key: &str) -> Option<&Value>{
        self.data.get(key)
    }
}

//\&b=2&c&d=&e===&d=7&d=abc
impl<'life_buf> From<&'life_buf str> for QueryString<'life_buf> {
    fn from (s: &'life_buf str) -> Self{
        let mut data = HashMap::new();
        for sub_string in s.split('&') {
            let mut key = sub_string;
            let mut val = "";
            if let Some(i) = sub_string.find('=') {
                key = &sub_string[..i];
                val = &sub_string[i + 1..];
            }
            data.entry(key).and_modify(|existing: &mut Value | match existing {
                Value::Single(prev_val) => {
                    *existing = Value::Multiple(vec![prev_val, val]);
                }
                Value::Multiple(vec) => vec.push(val),
            })
            .or_insert(Value::Single(val));
        }
        QueryString { data }
    }
}