use reqwest::Client;
use serde::Deserialize;
use sqlx::Row;
use sqlx::sqlite::SqlitePool;

use crate::config::load_db_url;

#[derive(Debug, Deserialize)]
struct OllamaResponse {
    response: String,
}

pub async fn get_sqlite_schema(db_url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let pool = SqlitePool::connect(db_url).await?;

    let rows = sqlx::query("SELECT sql FROM sqlite_master WHERE type='table';")
        .fetch_all(&pool)
        .await?;

    let mut schema = String::new();
    for row in rows {
        let ddl: String = row.try_get(0)?;
        schema.push_str(&ddl);
        schema.push_str(";\n");
    }

    Ok(schema)
}

pub async fn generate_sql(prompt: &str) -> Result<String, Box<dyn std::error::Error>> {
    let client = Client::new();

    let db_url = load_db_url()?;
    let schema = get_sqlite_schema(&db_url).await.unwrap_or_default();

    let full_prompt = format!(
        "Gunakan skema tabel berikut:\n{}\n\nUbah permintaan berikut menjadi query SQL yang valid dan cocok dengan skema di atas. Jawab dengan SQL murni tanpa penjelasan:\n\"{}\"",
        schema, prompt
    );
    // println!("📤 Prompt sent to AI:\n{}\n", full_prompt);

    let response = client
        .post("http://localhost:11434/api/generate")
        .json(&serde_json::json!({
            "model": "deepseek-coder",
            "prompt": full_prompt,
            "stream": false,
            "stop": ["\n\n", "--", "#"]
        }))
        .send()
        .await?;

    if !response.status().is_success() {
        let err_text = response.text().await?;
        return Err(format!("Gagal dari Ollama: {}", err_text).into());
    }

    let data: OllamaResponse = response.json().await?;
    let raw = data.response.trim();
    // println!("🔍 Raw response from AI:\n{}\n", raw);

    let cleaned_line = raw.lines().map(str::trim).find(|line| {
        let upper = line.to_uppercase();
        upper.starts_with("SELECT")
            || upper.starts_with("WITH")
            || upper.starts_with("INSERT")
            || upper.starts_with("UPDATE")
            || upper.starts_with("DELETE")
    });

    match cleaned_line {
        Some(sql_line) => Ok(sql_line.to_string()),
        None => Err("❌ The response from the AI does not contain valid SQL.".into()),
    }
}
