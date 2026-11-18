use comfy_table::Table;
use inquire::Text;
use sqlx::{MySqlPool, Row};

// ==========================================
// 1. VIEWS & SEARCH HANDLERS
// ==========================================

// View full catalog via vw_catalog
pub async fn view_items(pool: &MySqlPool) -> Result<(), Box<dyn std::error::Error>> {
    let rows = sqlx::query("SELECT * FROM vw_catalog ORDER BY itemID ASC LIMIT 25")
        .fetch_all(pool)
        .await?;

    let mut table = Table::new();
    table.set_header(vec!["Item ID", "Title", "Type", "Genre", "Section", "Copies"]);

    for row in rows {
        let id: i32 = row.try_get("itemID")?;
        let title: String = row.try_get("item_title")?;
        let type_name: String = row.try_get("type_name")?;
        let genre_name: String = row.try_get("genre_name")?;
        let section: String = row.try_get("section")?;
        let copies: i32 = row.try_get("copies")?;

        table.add_row(vec![
            id.to_string(),
            title,
            type_name,
            genre_name,
            section,
            copies.to_string(),
        ]);
    }

    println!("\n{table}\n");
    Ok(())
}

// Search catalog by keyword via vw_catalog
pub async fn search_items(pool: &MySqlPool) -> Result<(), Box<dyn std::error::Error>> {
    let keyword = Text::new("Search item title:").prompt()?;
    let pattern = format!("%{}%", keyword.trim());

    let rows = sqlx::query(
        "SELECT itemID, item_title, type_name, copies FROM vw_catalog WHERE item_title LIKE ?",
    )
    .bind(pattern)
    .fetch_all(pool)
    .await?;

    let mut table = Table::new();
    table.set_header(vec!["Item ID", "Title", "Type", "Copies Available"]);

    for row in rows {
        let id: i32 = row.try_get("itemID")?;
        let title: String = row.try_get("item_title")?;
        let type_name: String = row.try_get("type_name")?;
        let copies: i32 = row.try_get("copies")?;

        table.add_row(vec![id.to_string(), title, type_name, copies.to_string()]);
    }

    println!("\n{table}\n");
    Ok(())
}

// View library members
pub async fn view_members(pool: &MySqlPool) -> Result<(), Box<dyn std::error::Error>> {
    let query = "
        SELECT 
            memberID,
            member_name,
            DATE_FORMAT(exp_date, '%Y-%m-%d') AS exp_date,
            phone
        FROM member
        ORDER BY memberID ASC
        LIMIT 25
    ";

    let rows = sqlx::query(query).fetch_all(pool).await?;

    let mut table = Table::new();
    table.set_header(vec!["Member ID", "Name", "Card Expires", "Phone"]);

    for row in rows {
        let id: i32 = row.try_get("memberID")?;
        let name: String = row.try_get("member_name")?;
        let exp_date: String = row.try_get("exp_date")?;
        let phone: Option<String> = row.try_get("phone")?;

        table.add_row(vec![
            id.to_string(),
            name,
            exp_date,
            phone.unwrap_or_else(|| "N/A".to_string()),
        ]);
    }

    println!("\n{table}\n");
    Ok(())
}

// View active checkouts via vw_active_checkouts
pub async fn view_active_checkouts(pool: &MySqlPool) -> Result<(), Box<dyn std::error::Error>> {
    let rows = sqlx::query("SELECT * FROM vw_active_checkouts ORDER BY due_date ASC")
        .fetch_all(pool)
        .await?;

    let mut table = Table::new();
    table.set_header(vec![
        "Checkout ID",
        "Item ID",
        "Title",
        "Member ID",
        "Member Name",
        "Checked Out",
        "Due Date",
    ]);

    for row in rows {
        table.add_row(vec![
            row.try_get::<i32, _>("checkoutID")?.to_string(),
            row.try_get::<i32, _>("itemID")?.to_string(),
            row.try_get("item_title")?,
            row.try_get::<i32, _>("memberID")?.to_string(),
            row.try_get("member_name")?,
            row.try_get("checkout_date")?,
            row.try_get("due_date")?,
        ]);
    }

    println!("\n{table}\n");
    Ok(())
}

// ==========================================
// 2. CIRCULATION HANDLERS (STORED PROCEDURES)
// ==========================================

// Check out an item via Checkout_Item(membernum, itemnum)
pub async fn checkout_item(pool: &MySqlPool) -> Result<(), Box<dyn std::error::Error>> {
    let member_id: i32 = Text::new("Enter Member ID:").prompt()?.trim().parse()?;
    let item_id: i32 = Text::new("Enter Item ID:").prompt()?.trim().parse()?;

    match sqlx::query("CALL Checkout_Item(?, ?)")
        .bind(member_id)
        .bind(item_id)
        .execute(pool)
        .await
    {
        Ok(res) => {
            if res.rows_affected() > 0 {
                println!("\nSuccess: Item #{item_id} checked out to Member #{member_id}.\n");
            } else {
                println!("\nBlocked: No copies available or invalid Item ID.\n");
            }
        }
        Err(sqlx::Error::Database(db_err)) => {
            println!("\nDatabase error: {}\n", db_err.message());
        }
        Err(e) => return Err(e.into()),
    }

    Ok(())
}

// Check in an item via Checkin_Item(itemnum)
pub async fn checkin_item(pool: &MySqlPool) -> Result<(), Box<dyn std::error::Error>> {
    let item_id: i32 = Text::new("Enter Item ID to check in:").prompt()?.trim().parse()?;

    match sqlx::query("CALL Checkin_Item(?)")
        .bind(item_id)
        .execute(pool)
        .await
    {
        Ok(res) => {
            if res.rows_affected() > 0 {
                println!("\nSuccess: Item #{item_id} checked in.\n");
            } else {
                println!("\nNotice: No active checkout found for Item #{item_id}.\n");
            }
        }
        Err(sqlx::Error::Database(db_err)) => {
            println!("\nDatabase error: {}\n", db_err.message());
        }
        Err(e) => return Err(e.into()),
    }

    Ok(())
}

// ==========================================
// 3. STUDY ROOM HANDLERS
// ==========================================

// View study rooms and amenities
pub async fn view_rooms(pool: &MySqlPool) -> Result<(), Box<dyn std::error::Error>> {
    let rows = sqlx::query(
        "SELECT roomID, room_name, capacity, has_whiteboard, has_projector FROM study_room",
    )
    .fetch_all(pool)
    .await?;

    let mut table = Table::new();
    table.set_header(vec!["Room ID", "Room Name", "Capacity", "Whiteboard", "Projector"]);

    for row in rows {
        let room_id: i32 = row.try_get("roomID")?;
        let room_name: String = row.try_get("room_name")?;
        let capacity: i32 = row.try_get("capacity")?;
        let whiteboard: Option<bool> = row.try_get("has_whiteboard")?;
        let projector: Option<bool> = row.try_get("has_projector")?;

        table.add_row(vec![
            room_id.to_string(),
            room_name,
            capacity.to_string(),
            if whiteboard.unwrap_or(false) { "Yes" } else { "No" }.to_string(),
            if projector.unwrap_or(false) { "Yes" } else { "No" }.to_string(),
        ]);
    }

    println!("\n{table}\n");
    Ok(())
}

// View room bookings via vw_room_bookings
pub async fn view_room_bookings(pool: &MySqlPool) -> Result<(), Box<dyn std::error::Error>> {
    let rows = sqlx::query("SELECT * FROM vw_room_bookings ORDER BY booking_date DESC, start_time ASC")
        .fetch_all(pool)
        .await?;

    let mut table = Table::new();
    table.set_header(vec![
        "Booking ID",
        "Room",
        "Member",
        "Date",
        "Start",
        "End",
        "Status",
    ]);

    for row in rows {
        table.add_row(vec![
            row.try_get::<i32, _>("bookingID")?.to_string(),
            row.try_get("room_name")?,
            row.try_get("member_name")?,
            row.try_get("booking_date")?,
            row.try_get("start_time")?,
            row.try_get("end_time")?,
            row.try_get("booking_status")?,
        ]);
    }

    println!("\n{table}\n");
    Ok(())
}

// Book a study room via Book_Room(memberID, room, date, time, people)
pub async fn book_room(pool: &MySqlPool) -> Result<(), Box<dyn std::error::Error>> {
    let member_id: i32 = Text::new("Enter Member ID:").prompt()?.trim().parse()?;
    let room_id: i32 = Text::new("Enter Study Room ID:").prompt()?.trim().parse()?;
    let date = Text::new("Enter Date (YYYY-MM-DD):").prompt()?.trim().to_string();
    let time = Text::new("Enter Start Time (HH:MM:SS):").prompt()?.trim().to_string();
    let people: i32 = Text::new("Number of people:").prompt()?.trim().parse()?;

    match sqlx::query("CALL Book_Room(?, ?, ?, ?, ?)")
        .bind(member_id)
        .bind(room_id)
        .bind(&date)
        .bind(&time)
        .bind(people)
        .execute(pool)
        .await
    {
        Ok(res) => {
            if res.rows_affected() > 0 {
                println!("\nSuccess: Room #{room_id} booked on {date} at {time}.\n");
            } else {
                println!("\nBlocked: Party size ({people}) exceeds capacity or invalid Room ID.\n");
            }
        }
        Err(sqlx::Error::Database(db_err)) => {
            println!("\nDatabase error: {}\n", db_err.message());
        }
        Err(e) => return Err(e.into()),
    }

    Ok(())
}