use comfy_table::Table;
use inquire::{CustomType, Text};
use sqlx::{MySqlPool, Row};

// ==========================================
// 1. VIEWS & SEARCH HANDLERS
// ==========================================

// View full catalog via vw_catalog
pub async fn view_items(pool: &MySqlPool) -> Result<(), Box<dyn std::error::Error>> {
    let rows = sqlx::query(
        "SELECT itemID, item_title, type_name, genre_name, section, copies 
         FROM vw_catalog 
         ORDER BY itemID ASC 
         LIMIT 25",
    )
    .fetch_all(pool)
    .await?;

    let mut table = Table::new();
    table.set_header(vec!["Item ID", "Title", "Type", "Genre", "Section", "Copies"]);

    for row in rows {
        let id: i32 = row.try_get("itemID")?;
        let title: String = row.try_get("item_title")?;
        let type_name: String = row.try_get("type_name")?;
        let genre_name: String = row.try_get("genre_name")?;
        let section: Option<String> = row.try_get("section")?;
        let copies: i32 = row.try_get("copies")?;

        table.add_row(vec![
            id.to_string(),
            title,
            type_name,
            genre_name,
            section.unwrap_or_else(|| "N/A".to_string()),
            copies.to_string(),
        ]);
    }

    println!("\n{table}\n");
    Ok(())
}

// Search catalog by keyword via vw_catalog
pub async fn search_items(pool: &MySqlPool) -> Result<(), Box<dyn std::error::Error>> {
    let keyword = Text::new("Search item title:").prompt()?;
    let escaped = keyword
        .trim()
        .replace('\\', "\\\\")
        .replace('%', "\\%")
        .replace('_', "\\_");
    let pattern = format!("%{escaped}%");

    let rows = sqlx::query(
        "SELECT itemID, item_title, type_name, copies 
         FROM vw_catalog 
         WHERE item_title LIKE ? 
         LIMIT 25",
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
    let query = "
        SELECT 
            checkoutID,
            itemID,
            item_title,
            memberID,
            member_name,
            DATE_FORMAT(checkout_date, '%Y-%m-%d') AS checkout_date,
            DATE_FORMAT(due_date, '%Y-%m-%d') AS due_date
        FROM vw_active_checkouts 
        ORDER BY due_date ASC
        LIMIT 25
    ";

    let rows = sqlx::query(query).fetch_all(pool).await?;

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
    let member_id = CustomType::<i32>::new("Enter Member ID:")
        .with_error_message("Please enter a valid numeric Member ID")
        .prompt()?;
    let item_id = CustomType::<i32>::new("Enter Item ID:")
        .with_error_message("Please enter a valid numeric Item ID")
        .prompt()?;

    let row = sqlx::query("CALL Checkout_Item(?, ?)")
        .bind(member_id)
        .bind(item_id)
        .fetch_one(pool)
        .await?;

    let msg: String = row.try_get("status_msg")?;
    println!("\n{msg}\n");

    Ok(())
}

// Check in an item via Checkin_Item(itemnum)
pub async fn checkin_item(pool: &MySqlPool) -> Result<(), Box<dyn std::error::Error>> {
    let item_id = CustomType::<i32>::new("Enter Item ID to check in:")
        .with_error_message("Please enter a valid numeric Item ID")
        .prompt()?;

    let row = sqlx::query("CALL Checkin_Item(?)")
        .bind(item_id)
        .fetch_one(pool)
        .await?;

    let msg: String = row.try_get("status_msg")?;
    println!("\n{msg}\n");

    Ok(())
}

// ==========================================
// 3. STUDY ROOM HANDLERS
// ==========================================

// View study rooms and amenities
pub async fn view_rooms(pool: &MySqlPool) -> Result<(), Box<dyn std::error::Error>> {
    let rows = sqlx::query(
        "SELECT roomID, room_name, capacity, has_whiteboard, has_projector 
         FROM study_room 
         ORDER BY roomID ASC",
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
    let query = "
        SELECT 
            bookingID,
            room_name,
            member_name,
            DATE_FORMAT(booking_date, '%Y-%m-%d') AS booking_date,
            TIME_FORMAT(start_time, '%H:%i:%s') AS start_time,
            TIME_FORMAT(end_time, '%H:%i:%s') AS end_time,
            booking_status
        FROM vw_room_bookings 
        ORDER BY booking_date DESC, start_time ASC
        LIMIT 25
    ";

    let rows = sqlx::query(query).fetch_all(pool).await?;

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
    let member_id = CustomType::<i32>::new("Enter Member ID:")
        .with_error_message("Please enter a valid numeric Member ID")
        .prompt()?;
    let room_id = CustomType::<i32>::new("Enter Study Room ID:")
        .with_error_message("Please enter a valid numeric Room ID")
        .prompt()?;
    let date = Text::new("Enter Date (YYYY-MM-DD):").prompt()?.trim().to_string();
    let time = Text::new("Enter Start Time (HH:MM:SS):").prompt()?.trim().to_string();
    let people = CustomType::<i32>::new("Number of people:")
        .with_error_message("Please enter a valid number of people")
        .prompt()?;

    let row = sqlx::query("CALL Book_Room(?, ?, ?, ?, ?)")
        .bind(member_id)
        .bind(room_id)
        .bind(&date)
        .bind(&time)
        .bind(people)
        .fetch_one(pool)
        .await?;

    let msg: String = row.try_get("status_msg")?;
    println!("\n{msg}\n");

    Ok(())
}