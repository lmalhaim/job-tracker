DROP TABLE IF EXISTS user; 

CREATE TABLE IF NOT EXISTS users (
    id INT UNSIGNED AUTO_INCREMENT NOT NULL PRIMARY KEY,
    name VARCHAR(255),
    email_address VARCHAR(255) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL
);

CREATE TABLE IF NOT EXISTS jobs (
    id INT UNSIGNED AUTO_INCREMENT NOT NULL PRIMARY KEY,
    user_id INT UNSIGNED NOT NULL,
    role VARCHAR(255) NOT NULL,
    company VARCHAR(255) NOT NULL, 
    status ENUM('applied', 'interview', 'offer', 'rejected') NOT NULL, 
    FOREIGN KEY (user_id) REFERENCES users(id)
);  

CREATE TABLE IF NOT EXISTS email_accounts (
    id INT UNSIGNED AUTO_INCREMENT NOT NULL PRIMARY KEY,
    user_id INT UNSIGNED NOT NULL,
    email_address VARCHAR(255) UNIQUE NOT NULL,
    auth_token TEXT NOT NULL,
    provider ENUM('gmail', 'outlook', 'yahoo') NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users(id)
);

