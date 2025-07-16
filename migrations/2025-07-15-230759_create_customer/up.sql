-- Your SQL goes here

CREATE TABLE customers (
    customer_id INT NOT NULL AUTO_INCREMENT,
    first_name VARCHAR(50) NOT NULL, 
    last_name VARCHAR(50) NOT NULL, 
    phone VARCHAR(12) NOT NULL, 
    email VARCHAR(50) NOT NULL,
    CONSTRAINT pk_customer PRIMARY KEY (customer_id)
);
