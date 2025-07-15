-- Your SQL goes here

CREATE TABLE Customer(
    CustomerID int not null auto_increment,
    FirstName varchar(50) not null, 
    LastName varchar(50) not null, 
    Phone varchar(12) not null, 
    Email varchar(50) not null,
    CONSTRAINT PK_Customer PRIMARY KEY (CustomerID)
);
