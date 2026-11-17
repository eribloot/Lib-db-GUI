use comfy_table::Table;
use inquire::Select;
use sqlx::{mysql::MySqlPoolOptions, Row};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    let db_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env file");

    println!("Connecting to database...");
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await?;
    println!("Connected!\n");

    loop {
        let choices = vec![
            "1. Test Connection (Show Tables)",
            "2. Exit",
        ];

        let selection = Select::new("Choose an action:", choices).prompt()?;

        match selection {
            "1. Test Connection (Show Tables)" => list_tables(&pool).await?,
            _ => break,
        }
    }
    Ok(())
}

async fn list_tables(pool: &sqlx::MySqlPool) -> Result<(), sqlx::Error> {
    let rows = sqlx::query("SHOW TABLES").fetch_all(pool).await?;

    let mut table = Table::new();
    table.set_header(vec!["Tables in Database"]);

    for row in rows {
        // col 0 holds table name returned by SHOW TABLES
        let table_name: String = row.try_get(0)?;
        table.add_row(vec![table_name]);
    }

    println!("\n{table}\n");
    Ok(())
}