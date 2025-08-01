-- Your SQL goes here

CREATE TABLE customers (
    customer_id INT NOT NULL AUTO_INCREMENT,
    first_name VARCHAR(50), 
    last_name VARCHAR(50), 
    phone VARCHAR(12), 
    email VARCHAR(50),
    CONSTRAINT pk_customer PRIMARY KEY (customer_id)
);
