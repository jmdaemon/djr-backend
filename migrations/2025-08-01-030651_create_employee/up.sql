-- Your SQL goes here

CREATE TABLE employees (
    employee_id INT NOT NULL AUTO_INCREMENT,
    first_name VARCHAR(50), 
    last_name VARCHAR(50), 
    phone VARCHAR(12), 
    email VARCHAR(50),
    CONSTRAINT pk_employee PRIMARY KEY (employee_id)
);
