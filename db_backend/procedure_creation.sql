-- TO-DO:
-- * MORE DOCUMENTATION

-- Statements to remove every procedure before remaking them to make updating easier and prevent "already exists" errors

USE library;

DROP PROCEDURE IF EXISTS Checkin_Item;
DROP PROCEDURE IF EXISTS Checkout_Item;
DROP PROCEDURE IF EXISTS Book_Room;

DELIMITER //

-- Checking in item
CREATE PROCEDURE Checkin_Item(
    IN itemnum INT
)
BEGIN
    DECLARE v_checkout_id INT;

    START TRANSACTION;
        SELECT checkoutID INTO v_checkout_id
        FROM checkout
        WHERE itemID = itemnum AND return_date IS NULL
        ORDER BY checkout_date ASC
        LIMIT 1;

        IF v_checkout_id IS NOT NULL THEN
            UPDATE checkout
            SET return_date = CURDATE()
            WHERE checkoutID = v_checkout_id;

            COMMIT;
            SELECT CONCAT('Success: Item #', itemnum, ' (Checkout #', v_checkout_id, ') checked in.') AS status_msg;
        ELSE
            ROLLBACK;
            SELECT CONCAT('Notice: No active checkout found for Item #', itemnum, '.') AS status_msg;
        END IF;
END //

-- Checking Out an Item 
CREATE PROCEDURE Checkout_Item(
    IN membernum INT,
    IN itemnum INT
)
BEGIN
    DECLARE v_copies INT;
    DECLARE v_loan_days INT;
    DECLARE v_exp_date DATE;
    DECLARE v_queue_pos INT;

    START TRANSACTION;
        -- Validate member and check card expiration
        SELECT exp_date INTO v_exp_date
        FROM member
        WHERE memberID = membernum;

        IF v_exp_date IS NULL THEN
            ROLLBACK;
            SELECT CONCAT('Blocked: Member #', membernum, ' does not exist.') AS status_msg;
        ELSEIF v_exp_date < CURDATE() THEN
            ROLLBACK;
            SELECT CONCAT('Blocked: Member #', membernum, ' card expired on ', v_exp_date, '.') AS status_msg;
        ELSE
            -- Validate item and get copies + loan period
            SELECT i.copies, t.loan_period_days INTO v_copies, v_loan_days
            FROM item i
            JOIN type t ON i.typeID = t.typeID
            WHERE i.itemID = itemnum
            FOR UPDATE;

            IF v_copies IS NULL THEN
                ROLLBACK;
                SELECT CONCAT('Blocked: Item #', itemnum, ' does not exist.') AS status_msg;
            ELSEIF v_copies > 0 THEN
                INSERT INTO checkout (checkout_date, due_date, return_date, itemID, memberID)
                VALUES (CURDATE(), DATE_ADD(CURDATE(), INTERVAL v_loan_days DAY), NULL, itemnum, membernum);

                COMMIT;
                SELECT CONCAT('Success: Item #', itemnum, ' checked out to Member #', membernum, ' (Due in ', v_loan_days, ' days).') AS status_msg;
            ELSE
                -- 0 copies available: add member to reservation queue
                SELECT COALESCE(queue_size, 0) + 1 INTO v_queue_pos
                FROM queue
                WHERE itemID = itemnum
                FOR UPDATE;

                INSERT INTO reserved (reserved_date, queue_position, notification_date, hold_exp_date, reservation_status, memberID, itemID)
                VALUES (NOW(), v_queue_pos, NULL, NULL, 'Pending', membernum, itemnum);

                UPDATE queue
                SET queue_size = v_queue_pos
                WHERE itemID = itemnum;

                COMMIT;
                SELECT CONCAT('Waitlisted: No copies available. Member #', membernum, ' placed at queue position #', v_queue_pos, '.') AS status_msg;
            END IF;
        END IF;
END //

-- Booking a Study Room (checks capacity and time overlap)
CREATE PROCEDURE Book_Room(
    IN p_memberID INT,
    IN p_room INT,
    IN p_date DATE,
    IN p_time TIME,
    IN p_people INT
)
BEGIN
    DECLARE v_capacity INT;
    DECLARE v_member_exists INT;
    DECLARE v_conflict_count INT;
    DECLARE v_end_time TIME;

    SET v_end_time = ADDTIME(p_time, '01:30:00');

    START TRANSACTION;
        SELECT COUNT(*) INTO v_member_exists FROM member WHERE memberID = p_memberID;
        SELECT capacity INTO v_capacity FROM study_room WHERE roomID = p_room;

        IF v_member_exists = 0 THEN
            ROLLBACK;
            SELECT CONCAT('Blocked: Member #', p_memberID, ' does not exist.') AS status_msg;
        ELSEIF v_capacity IS NULL THEN
            ROLLBACK;
            SELECT CONCAT('Blocked: Study Room #', p_room, ' does not exist.') AS status_msg;
        ELSEIF p_people <= 0 OR p_people > v_capacity THEN
            ROLLBACK;
            SELECT CONCAT('Blocked: Party size (', p_people, ') invalid or exceeds room capacity (', v_capacity, ').') AS status_msg;
        ELSE
            -- Check for overlapping confirmed bookings on the same date
            SELECT COUNT(*) INTO v_conflict_count
            FROM room_booking
            WHERE roomID = p_room
              AND booking_date = p_date
              AND booking_status = 'Confirmed'
              AND p_time < end_time
              AND v_end_time > start_time;

            IF v_conflict_count > 0 THEN
                ROLLBACK;
                SELECT CONCAT('Blocked: Room #', p_room, ' is already booked during that time window on ', p_date, '.') AS status_msg;
            ELSE
                INSERT INTO room_booking (booking_date, start_time, end_time, booking_status, roomID, memberID)
                VALUES (p_date, p_time, v_end_time, 'Confirmed', p_room, p_memberID);

                COMMIT;
                SELECT CONCAT('Success: Room #', p_room, ' booked on ', p_date, ' from ', p_time, ' to ', v_end_time, '.') AS status_msg;
            END IF;
        END IF;
END //

DELIMITER ;