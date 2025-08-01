-- Your SQL goes here

CREATE TABLE employees (
    employee_id INT NOT NULL AUTO_INCREMENT,
    first_name VARCHAR(50) NOT NULL, 
    last_name VARCHAR(50) NOT NULL, 
    phone VARCHAR(12) NOT NULL, 
    email VARCHAR(50) NOT NULL,
    CONSTRAINT pk_employee PRIMARY KEY (employee_id)
);
