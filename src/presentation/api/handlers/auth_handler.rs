use crate::domain::request::LoginRequest;
use crate::infrastructure::database::Database;
use axum::Json;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use mysql_async::Value as MySqlValue;
use mysql_async::prelude::Queryable;
use serde_json::{Map, Value};

pub async fn login(Json(params): Json<LoginRequest>) -> impl IntoResponse {
    let database = Database::new();

    let mut conn = match database.get_mysql_connection().await {
        Ok(c) => c,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    };

    let mut result = match conn.query_iter(params.query).await {
        Ok(r) => r,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    };

    let columns = result.columns().unwrap_or_default();

    let rows: Vec<mysql_async::Row> = match result.collect().await {
        Ok(r) => r,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    };

    let mut json_results = Vec::new();

    for row in rows {
        let mut map = Map::new();
        for (i, column) in columns.iter().enumerate() {
            let name = column.name_str().to_string();
            let value: MySqlValue = row.get(i).unwrap_or(MySqlValue::NULL);

            let json_value = match value {
                MySqlValue::NULL => Value::Null,
                MySqlValue::Bytes(b) => Value::String(String::from_utf8_lossy(&b).to_string()),
                MySqlValue::Int(i) => Value::Number(i.into()),
                MySqlValue::UInt(u) => Value::Number(u.into()),
                MySqlValue::Float(f) => Value::Number(
                    serde_json::Number::from_f64(f as f64).unwrap_or(serde_json::Number::from(0)),
                ),
                MySqlValue::Double(d) => Value::Number(
                    serde_json::Number::from_f64(d).unwrap_or(serde_json::Number::from(0)),
                ),
                MySqlValue::Date(y, m, d, h, min, s, ms) => Value::String(format!(
                    "{}-{:02}-{:02} {:02}:{:02}:{:02}.{:03}",
                    y, m, d, h, min, s, ms
                )),
                MySqlValue::Time(neg, d, h, m, s, ms) => Value::String(format!(
                    "{}{}.{:02}:{:02}:{:02}.{:03}",
                    if neg { "-" } else { "" },
                    d,
                    h,
                    m,
                    s,
                    ms
                )),
            };
            map.insert(name, json_value);
        }
        json_results.push(Value::Object(map));
    }

    Json(json_results).into_response()
}
