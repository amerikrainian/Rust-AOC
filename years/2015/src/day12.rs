use serde_json::Value;

fn sum_all(v: &Value) -> i64 {
    match v {
        Value::Number(n) => n.as_i64().unwrap(),
        Value::Array(a) => a.iter().map(sum_all).sum(),
        Value::Object(o) => o.values().map(sum_all).sum(),
        _ => 0,
    }
}

fn sum_no_red(v: &Value) -> i64 {
    match v {
        Value::Number(n) => n.as_i64().unwrap(),

        Value::Array(a) => a.iter().map(sum_no_red).sum(),

        Value::Object(o) => {
            if o.values().any(|v| v == "red") {
                0
            } else {
                o.values().map(sum_no_red).sum()
            }
        }

        _ => 0,
    }
}

pub fn solve(input: &str) -> (String, String) {
    let v: Value = serde_json::from_str(input).unwrap();

    (sum_all(&v).to_string(), sum_no_red(&v).to_string())
}
