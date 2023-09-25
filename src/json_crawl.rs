use std::fmt::Display;

use serde_json::Value;

#[derive(Clone)]
pub enum JsonPathPart {
    Field(String),
    Index(usize),
}

pub struct JsonPath(Vec<JsonPathPart>);

impl JsonPath {
    pub fn new() -> Self {
        JsonPath(vec![])
    }
    pub fn append(&self, part: JsonPathPart) -> Self {
        let mut vec = self.0.clone();
        vec.push(part);
        JsonPath(vec)
    }
}
impl Display for JsonPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out = "".to_owned();
        for (i, part) in self.0.iter().enumerate() {
            let z = match (i, part) {
                (_, JsonPathPart::Index(index)) => format!("[{}]", index),
                (0, JsonPathPart::Field(field)) => field.to_string(),
                (_, JsonPathPart::Field(field)) => format!(".{}", field),
            };
            out.push_str(&z);
        }
        write!(f, "{}", out)
    }
}
pub fn crawl_json<F>(value: Value, path: JsonPath, predicate: &F, out: &mut Vec<(JsonPath, i64)>)
where
    F: Fn(i64) -> bool,
{
    match value {
        Value::Number(num) => {
            if let Some(num) = num.as_i64() {
                if predicate(num) {
                    out.push((path, num))
                }
            }
        }
        Value::Array(arr) => {
            for (i, sub_val) in arr.into_iter().enumerate() {
                let sub_path = path.append(JsonPathPart::Index(i));
                crawl_json(sub_val, sub_path, predicate, out);
            }
        }
        Value::Object(obj) => {
            for (key, sub_val) in obj.into_iter() {
                let sub_path = path.append(JsonPathPart::Field(key));
                crawl_json(sub_val, sub_path, predicate, out)
            }
        }
        _ => {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let data = r#"
        {
            "name": "John Doe",
            "age": 43,
            "phones": [
                123,
                456
            ],
            "nested": {
                "num": -13.4,
                "more_num": 12,
                "list": [
                    1,
                    "a"
                ]
            },
            "obj_list": [
                {"a": 1, "b": 2},
                {"a": 10, "b": "1"}
            ]
        }"#;
        let value = serde_json::from_str(data).unwrap();
        let mut out = vec![];
        let predicate = |num| num > 8;
        crawl_json(value, JsonPath::new(), &predicate, &mut out);
    }
}
