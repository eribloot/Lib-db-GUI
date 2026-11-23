USE library;

DROP TRIGGER IF EXISTS tr_increment_copies;
DROP TRIGGER IF EXISTS tr_decrement_copies;
DROP TRIGGER IF EXISTS tr_initialize_queues;

DELIMITER //

-- Trigger Ran on Check-In
CREATE TRIGGER tr_increment_copies
AFTER UPDATE
ON checkout
FOR EACH ROW
BEGIN
    IF OLD.return_date IS NULL AND NEW.return_date IS NOT NULL THEN
        UPDATE item
        SET copies = copies + 1
        WHERE itemID = NEW.itemID;
    END IF;
END //

-- Trigger Ran on Checkout
CREATE TRIGGER tr_decrement_copies
AFTER INSERT
ON checkout
FOR EACH ROW
BEGIN
    UPDATE item
    SET copies = copies - 1
    WHERE itemID = NEW.itemID;
END //

-- Trigger Ran on Item Creation (initializes a queue row for every new item)
CREATE TRIGGER tr_initialize_queues
AFTER INSERT
ON item
FOR EACH ROW
BEGIN
    INSERT INTO queue (itemID, queue_size)
    VALUES (NEW.itemID, 0);
END //

DELIMITER ;