USE library;

DROP VIEW IF EXISTS vw_active_checkouts;
DROP VIEW IF EXISTS vw_room_bookings;
DROP VIEW IF EXISTS vw_catalog;

-- View That Shows Active Checkouts
CREATE VIEW vw_active_checkouts AS
SELECT *
FROM (item NATURAL JOIN checkout) NATURAL JOIN member
WHERE return_date IS NULL;

-- View That Shows Confirmed Room Bookings
CREATE VIEW vw_room_bookings AS
SELECT *
FROM (study_room NATURAL JOIN room_booking) NATURAL JOIN member
WHERE booking_status = 'Confirmed';

-- View That Shows All Items And Their Details
CREATE VIEW vw_catalog AS
SELECT *
FROM (item NATURAL JOIN type) NATURAL JOIN genre;