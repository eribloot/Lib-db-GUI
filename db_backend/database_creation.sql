DROP DATABASE IF EXISTS library;
CREATE DATABASE library;

USE library;

DROP TABLE IF EXISTS queue;
DROP TABLE IF EXISTS fine;
DROP TABLE IF EXISTS reserved;
DROP TABLE IF EXISTS checkout;
DROP TABLE IF EXISTS room_booking;
DROP TABLE IF EXISTS study_room;
DROP TABLE IF EXISTS item;
DROP TABLE IF EXISTS type;
DROP TABLE IF EXISTS genre;
DROP TABLE IF EXISTS member;

CREATE TABLE member (
    memberID INT PRIMARY KEY,
    member_name VARCHAR(100) NOT NULL,
    exp_date DATE NOT NULL,
    phone VARCHAR(20),
    address VARCHAR(255),
    birthday DATE
);

CREATE TABLE type (
    typeID INT PRIMARY KEY,
    type_name VARCHAR(50) NOT NULL,
    loan_period_days INT NOT NULL, 
    daily_late_fee DECIMAL(5,2)
);

CREATE TABLE genre (
    genreID INT PRIMARY KEY,
    genre_name VARCHAR(50) NOT NULL
);

CREATE TABLE item (
    itemID INT PRIMARY KEY,
    item_title VARCHAR(255) NOT NULL,
    ISBN VARCHAR(20),
    dewey VARCHAR(20),
    section VARCHAR(50),
    copies INT NOT NULL,
    typeID INT NOT NULL,
    genreID INT NOT NULL,

    FOREIGN KEY (typeID) REFERENCES type(typeID),
    FOREIGN KEY (genreID) REFERENCES genre(genreID)
);

CREATE TABLE queue (
    itemID INT PRIMARY KEY,
    queue_size INT NOT NULL DEFAULT 0,

    FOREIGN KEY (itemID) REFERENCES item(itemID)
);

CREATE TABLE checkout (
    checkoutID INT AUTO_INCREMENT PRIMARY KEY,
    checkout_date DATE NOT NULL,
    due_date DATE NOT NULL,
    return_date DATE, 
    itemID INT NOT NULL,
    memberID INT NOT NULL,

    FOREIGN KEY (itemID) REFERENCES item(itemID),
    FOREIGN KEY (memberID) REFERENCES member(memberID)
);

CREATE TABLE reserved (
    reservedID INT AUTO_INCREMENT PRIMARY KEY,
    reserved_date DATETIME NOT NULL,
    queue_position INT NOT NULL,
    notification_date DATETIME,
    hold_exp_date DATE,
    reservation_status VARCHAR(30) NOT NULL,
    memberID INT NOT NULL,
    itemID INT NOT NULL,

    FOREIGN KEY (memberID) REFERENCES member(memberID),
    FOREIGN KEY (itemID) REFERENCES item(itemID)
);

CREATE TABLE study_room (
    roomID INT PRIMARY KEY,
    room_name VARCHAR(50) NOT NULL,
    capacity INT NOT NULL,
    has_whiteboard BOOLEAN,
    has_projector BOOLEAN
);

CREATE TABLE room_booking (
    bookingID INT AUTO_INCREMENT PRIMARY KEY,
    booking_date DATE NOT NULL,
    start_time TIME NOT NULL,
    end_time TIME NOT NULL,
    booking_status VARCHAR(20) NOT NULL,
    roomID INT NOT NULL,
    memberID INT NOT NULL,

    FOREIGN KEY (roomID) REFERENCES study_room(roomID),
    FOREIGN KEY (memberID) REFERENCES member(memberID)
);

CREATE TABLE fine (
    fineID INT AUTO_INCREMENT PRIMARY KEY,
    fine_amount DECIMAL(6,2) NOT NULL,
    assessed_date DATE NOT NULL,
    paid_date DATE,
    fine_status VARCHAR(20) NOT NULL,
    checkoutID INT NOT NULL,
    memberID INT NOT NULL,

    FOREIGN KEY (checkoutID) REFERENCES checkout(checkoutID),
    FOREIGN KEY (memberID) REFERENCES member(memberID)
);