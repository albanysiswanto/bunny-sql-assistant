use chrono::NaiveDate;
use sqlx::{postgres::PgRow, sqlite::SqliteRow, Column, Row};
use tabled::{builder::Builder, settings::Style};

pub async fn execute_sql_and_print(
    sql: &str,
    db_url: &str,
    db_type: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    match db_type {
        "postgres" => {
            let pool = sqlx::postgres::PgPoolOptions::new().connect(db_url).await?;
            let rows = sqlx::query(sql).fetch_all(&pool).await?;
            print_pg_table(rows);
        }
        "sqlite" => {
            let pool = sqlx::sqlite::SqlitePoolOptions::new()
                .connect(db_url)
                .await?;
            let rows = sqlx::query(sql).fetch_all(&pool).await?;
            print_sqlite_table(rows);
        }
        other => {
            return Err(format!("Unsupported database type: {}", other).into());
        }
    }

    Ok(())
}

fn print_pg_table(rows: Vec<PgRow>) {
    if rows.is_empty() {
        println!("(No results)");
        return;
    }

    let columns = rows.get(0).unwrap().columns();
    let mut builder = Builder::default();

    let header: Vec<_> = columns.iter().map(|c| c.name().to_string()).collect();
    builder.push_record(&header);

    for row in &rows {
        let mut record = Vec::new();

        for i in 0..columns.len() {
            let val = try_get_pg_value_as_string(row, i).unwrap_or_else(|| "NULL".to_string());
            record.push(val);
        }

        builder.push_record(record);
    }

    println!("{}", builder.build().with(Style::modern()));
}

fn print_sqlite_table(rows: Vec<SqliteRow>) {
    if rows.is_empty() {
        println!("(No results)");
        return;
    }

    let columns = rows.get(0).unwrap().columns();
    let mut builder = Builder::default();

    let header: Vec<_> = columns.iter().map(|c| c.name().to_string()).collect();
    builder.push_record(&header);

    for row in &rows {
        let mut record = Vec::new();

        for i in 0..columns.len() {
            let val = try_get_sqlite_value_as_string(row, i).unwrap_or_else(|| "NULL".to_string());
            record.push(val);
        }

        builder.push_record(record);
    }

    println!("{}", builder.build().with(Style::modern()));
}

fn try_get_pg_value_as_string(row: &PgRow, idx: usize) -> Option<String> {
    if let Ok(v) = row.try_get::<i32, _>(idx) {
        return Some(v.to_string());
    }
    if let Ok(v) = row.try_get::<i64, _>(idx) {
        return Some(v.to_string());
    }
    if let Ok(v) = row.try_get::<f32, _>(idx) {
        return Some(v.to_string());
    }
    if let Ok(v) = row.try_get::<f64, _>(idx) {
        return Some(v.to_string());
    }
    if let Ok(v) = row.try_get::<bool, _>(idx) {
        return Some(v.to_string());
    }
    if let Ok(v) = row.try_get::<String, _>(idx) {
        return Some(v);
    }
    if let Ok(v) = row.try_get::<&str, _>(idx) {
        return Some(v.to_string());
    }
    if let Ok(v) = row.try_get::<NaiveDate, _>(idx) {
        return Some(v.to_string());
    }
    if let Ok(v) = row.try_get::<NaiveDate, _>(idx) {
        return Some(v.to_string());
    }
    if let Ok(Some(v)) = row.try_get::<Option<NaiveDate>, _>(idx) {
        return Some(v.to_string());
    }
    if let Ok(None) = row.try_get::<Option<NaiveDate>, _>(idx) {
        return Some("NULL".to_string());
    }

    // Alternatif: ISO format
    if let Ok(v) = row.try_get::<String, _>(idx) {
        if v.parse::<NaiveDate>().is_ok() {
            return Some(v);
        }
    }
    None
}

fn try_get_sqlite_value_as_string(row: &SqliteRow, idx: usize) -> Option<String> {
    if let Ok(v) = row.try_get::<i32, _>(idx) {
        return Some(v.to_string());
    }
    if let Ok(v) = row.try_get::<i64, _>(idx) {
        return Some(v.to_string());
    }
    if let Ok(v) = row.try_get::<f64, _>(idx) {
        return Some(v.to_string());
    }
    if let Ok(v) = row.try_get::<bool, _>(idx) {
        return Some(v.to_string());
    }
    if let Ok(v) = row.try_get::<String, _>(idx) {
        return Some(v);
    }
    if let Ok(v) = row.try_get::<&str, _>(idx) {
        return Some(v.to_string());
    }
    if let Ok(v) = row.try_get::<NaiveDate, _>(idx) {
        return Some(v.to_string());
    }
    if let Ok(Some(v)) = row.try_get::<Option<NaiveDate>, _>(idx) {
        return Some(v.to_string());
    }
    if let Ok(None) = row.try_get::<Option<NaiveDate>, _>(idx) {
        return Some("NULL".to_string());
    }

    if let Ok(v) = row.try_get::<String, _>(idx) {
        if v.parse::<NaiveDate>().is_ok() {
            return Some(v);
        }
    }
    None
}
