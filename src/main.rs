mod handlers;

use inquire::Select;
use sqlx::{mysql::MySqlPoolOptions, MySqlPool};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    let db_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in your .env file");

    println!("Connecting to library database...");
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await?;
    println!("Connected!\n");

    loop {
        let main_choices = vec![
            "1. Views & Search (Catalog, Members, Checkouts)",
            "2. Circulation Desk (Check Out / Check In)",
            "3. Study Rooms (View & Book)",
            "4. Exit",
        ];

        let selection = Select::new("Library Management System:", main_choices).prompt()?;

        match selection {
            "1. Views & Search (Catalog, Members, Checkouts)" => views_menu(&pool).await?,
            "2. Circulation Desk (Check Out / Check In)" => circulation_menu(&pool).await?,
            "3. Study Rooms (View & Book)" => rooms_menu(&pool).await?,
            _ => break,
        }
    }

    println!("Goodbye!");
    Ok(())
}

async fn views_menu(pool: &MySqlPool) -> Result<(), Box<dyn std::error::Error>> {
    loop {
        let choices = vec![
            "1. View Catalog (Items)",
            "2. Search Catalog",
            "3. View Members",
            "4. View Active Checkouts",
            "<- Back to Main Menu",
        ];

        let selection = Select::new("Views & Search:", choices).prompt()?;

        match selection {
            "1. View Catalog (Items)" => handlers::view_items(pool).await?,
            "2. Search Catalog" => handlers::search_items(pool).await?,
            "3. View Members" => handlers::view_members(pool).await?,
            "4. View Active Checkouts" => handlers::view_active_checkouts(pool).await?,
            _ => break,
        }
    }
    Ok(())
}

async fn circulation_menu(pool: &MySqlPool) -> Result<(), Box<dyn std::error::Error>> {
    loop {
        let choices = vec![
            "1. Check Out Item",
            "2. Check In Item",
            "3. View Active Checkouts",
            "<- Back to Main Menu",
        ];

        let selection = Select::new("Circulation Desk:", choices).prompt()?;

        match selection {
            "1. Check Out Item" => handlers::checkout_item(pool).await?,
            "2. Check In Item" => handlers::checkin_item(pool).await?,
            "3. View Active Checkouts" => handlers::view_active_checkouts(pool).await?,
            _ => break,
        }
    }
    Ok(())
}

async fn rooms_menu(pool: &MySqlPool) -> Result<(), Box<dyn std::error::Error>> {
    loop {
        let choices = vec![
            "1. View Study Rooms",
            "2. View Room Bookings",
            "3. Book a Study Room",
            "<- Back to Main Menu",
        ];

        let selection = Select::new("Study Rooms:", choices).prompt()?;

        match selection {
            "1. View Study Rooms" => handlers::view_rooms(pool).await?,
            "2. View Room Bookings" => handlers::view_room_bookings(pool).await?,
            "3. Book a Study Room" => handlers::book_room(pool).await?,
            _ => break,
        }
    }
    Ok(())
}