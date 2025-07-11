use sqlx::Column;
use sqlx::{Row, sqlite::SqlitePool};
use tabled::{builder::Builder, settings::Style};

pub async fn execute_sql_and_print(
    sql: &str,
    db_url: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let pool = SqlitePool::connect(db_url).await?;
    let rows = sqlx::query(sql).fetch_all(&pool).await?;

    if rows.is_empty() {
        println!("(No results)");
        return Ok(());
    }

    let columns = rows[0].columns();
    let column_names: Vec<String> = columns.iter().map(|c| c.name().to_string()).collect();

    let mut builder = Builder::default();
    builder.push_record(&column_names);

    for row in &rows {
        let mut row_data = Vec::new();

        for i in 0..columns.len() {
            let val = if let Ok(v) = row.try_get::<i64, _>(i) {
                v.to_string()
            } else if let Ok(v) = row.try_get::<f64, _>(i) {
                v.to_string()
            } else if let Ok(v) = row.try_get::<String, _>(i) {
                v
            } else {
                "NULL".to_string()
            };

            row_data.push(val);
        }

        builder.push_record(row_data);
    }

    let table = builder.build().with(Style::modern()).to_string();
    println!("{}", table);

    Ok(())
}
