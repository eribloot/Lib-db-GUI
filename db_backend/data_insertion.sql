USE library;

-- Initialization of our 3 item types
INSERT INTO type (typeID, type_name, loan_period_days, daily_late_fee)
VALUES (1, 'Book', 30, 0.25);

INSERT INTO type (typeID, type_name, loan_period_days, daily_late_fee)
VALUES (2, 'DVD', 15, 0.10);

INSERT INTO type (typeID, type_name, loan_period_days, daily_late_fee)
VALUES (3, 'CD', 60, 0.50);

-- Genre initialization
INSERT INTO genre (genreID, genre_name) VALUES
(1, 'Science Fiction'),
(2, 'Fantasy'),
(3, 'Horror'),
(4, 'Romance'),
(5, 'Comedy'),
(6, 'Drama'),
(7, 'Rock'),
(8, 'Pop'),
(9, 'Metal'),
(10, 'Action');

-- Items (triggers will automatically populate the `queue` table for each item)
INSERT INTO item (itemID, item_title, ISBN, dewey, section, copies, typeID, genreID)
VALUES (FLOOR(100000 + RAND() * 900000), 'Fahrenheit 451', '978-1451673319', '813.54', 'Fiction', 3, 1, 1);

INSERT INTO item (itemID, item_title, section, copies, typeID, genreID)
VALUES (FLOOR(100000 + RAND() * 900000), 'Eye In The Sky', 'Disk', 1, 3, 7);

INSERT INTO item (itemID, item_title, ISBN, dewey, section, copies, typeID, genreID)
VALUES (FLOOR(100000 + RAND() * 900000), 'The Great Gatsby', '9780-743273565', '813.52', 'Fiction', 0, 1, 6);

INSERT INTO item (itemID, item_title, ISBN, dewey, section, copies, typeID, genreID)
VALUES (FLOOR(100000 + RAND() * 900000), '1984', '978-0451524935', '823.912', 'Fiction', 6, 1, 1);

INSERT INTO item (itemID, item_title, section, copies, typeID, genreID)
VALUES (FLOOR(100000 + RAND() * 900000), 'Die Hard', 'Disk', 1, 2, 10);

-- Members
INSERT INTO member (memberID, member_name, exp_date, phone, address, birthday)
VALUES (FLOOR(100000 + RAND() * 900000), 'Susan Harris', '1986-07-03', '345-908-3200', '205 Somewhere Blvd.', '1950-10-04');

INSERT INTO member (memberID, member_name, exp_date, phone, address, birthday)
VALUES (FLOOR(100000 + RAND() * 900000), 'Tyler Barron', '2027-07-01', '208-651-5217', '5021 Schmodel Harm Rd.', '2008-02-01');

INSERT INTO member (memberID, member_name, exp_date, phone, address, birthday)
VALUES (FLOOR(100000 + RAND() * 900000), 'Santa Claus', '2026-12-25', '100-200-3000', '1225 North Pole Ln.', '1800-12-25');

-- Rooms
INSERT INTO study_room (roomID, room_name, capacity, has_whiteboard, has_projector)
VALUES (305, 'Room 1', 5, FALSE, TRUE);

INSERT INTO study_room (roomID, room_name, capacity, has_whiteboard, has_projector)
VALUES (306, 'Room 2', 10, TRUE, TRUE);

INSERT INTO study_room (roomID, room_name, capacity, has_whiteboard, has_projector)
VALUES (400, 'Lab', 30, TRUE, FALSE);