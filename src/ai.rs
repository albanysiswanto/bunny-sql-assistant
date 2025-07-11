use reqwest::Client;
use serde::{Deserialize, Serialize};
use sqlx::sqlite::SqlitePoolOptions;
use std::env;

#[derive(Serialize)]
struct GroqRequest {
    model: String,
    messages: Vec<Message>,
}

#[derive(Serialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(Deserialize)]
struct GroqResponse {
    choices: Vec<Choice>,
}

#[derive(Deserialize)]
struct Choice {
    message: MessageContent,
}

#[derive(Deserialize)]
struct MessageContent {
    content: String,
}

async fn extract_schema(db_url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let pool = SqlitePoolOptions::new().connect(db_url).await?;

    let rows = sqlx::query!("SELECT name FROM sqlite_master WHERE type='table'")
        .fetch_all(&pool)
        .await?;

    let mut schema_statements = Vec::new();

    for row in rows {
        if let Some(name) = row.name {
            let stmt_row = sqlx::query_scalar::<_, String>(&format!(
                "SELECT sql FROM sqlite_master WHERE name='{}';",
                name
            ))
            .fetch_one(&pool)
            .await?;

            schema_statements.push(stmt_row);
        }
    }

    Ok(schema_statements.join("\n"))
}

pub async fn generate_sql(prompt: &str) -> Result<String, Box<dyn std::error::Error>> {
    let api_key = env::var("GROQ_API_KEY")?;
    let model = env::var("GROQ_MODEL")?;

    let db_url = crate::config::load_db_url()?;

    let schema = extract_schema(&db_url).await?;

    let system_prompt = format!(
        "Gunakan skema database berikut untuk menjawab pertanyaan. Berikan hanya SQL valid tanpa komentar atau penjelasan:\n\n{}",
        schema
    );

    let messages = vec![
        Message {
            role: "system".into(),
            content: system_prompt,
        },
        Message {
            role: "user".into(),
            content: prompt.into(),
        },
    ];

    let body = GroqRequest { model, messages };

    let client = Client::new();
    let res = client
        .post("https://api.groq.com/openai/v1/chat/completions")
        .bearer_auth(api_key)
        .json(&body)
        .send()
        .await?;

    let status = res.status();
    let raw_text = res.text().await?;
    // println!("DEBUG: HTTP Status = {status}");
    // println!("DEBUG: Raw response = {raw_text}");

    if !status.is_success() {
        return Err(format!("Groq API error: {}", raw_text).into());
    }

    let data: GroqResponse = serde_json::from_str(&raw_text)?;
    let sql = data
        .choices
        .first()
        .map(|c| c.message.content.trim().to_string());

    sql.ok_or("No response from Groq".into())
}
