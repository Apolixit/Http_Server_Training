use std::{collections::HashMap};

//name=abc&sort=1&d=&e===&d=abc
#[derive(Debug)]
pub struct QueryString<'buf> {
    data: HashMap<&'buf str, Value<'buf>>
}

#[derive(Debug)]
pub enum Value<'buf> {
    Single(&'buf str),
    Multiple(Vec<&'buf str>)
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
            // let c = sub_str.split('=');
            let mut key = sub_str;
            let mut val = "";

            if let Some(i) = sub_str.find('=') {
                key = &sub_str[..i];
                val = &sub_str[i + 1..];
            }

            data.entry(key)
                .and_modify(|f:&mut Value| match f {
                    Value::Single(prev_val) => {
                        *f = Value::Multiple(vec![prev_val, val]);
                    },
                    Value::Multiple(prev_vec) => prev_vec.push(val)
                })
                .or_insert(Value::Single(val));

            // if(data.get(key).is_some()) {
            //     let mut valuePwet = data.get(key).unwrap();
            //     let mut newVec = Vec::new();
            //     newVec.push(key);
            //     valuePwet = &Value::Multiple(newVec);
            // }
        }

        QueryString { data }
    }
}