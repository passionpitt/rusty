CREATE SCHEMA IF NOT EXISTS `rusty_test` DEFAULT CHARACTER SET utf8mb4 COLLATE utf8mb4_bin;

USE `rusty_test`;

CREATE TABLE IF NOT EXISTS `rusty_test`.`users` (
    `id` INT PRIMARY KEY AUTO_INCREMENT,
    `name` VARCHAR(50) CHARACTER SET 'utf8mb4' COLLATE 'utf8mb4_bin' NOT NULL UNIQUE,
    `email` VARCHAR(100) CHARACTER SET 'utf8mb4' COLLATE 'utf8mb4_bin' NOT NULL UNIQUE,
    `created_at` TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    `updated_at` TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
)
ENGINE = InnoDB
DEFAULT CHARACTER SET = utf8mb4
COLLATE = utf8mb4_bin;

CREATE TABLE IF NOT EXISTS `rusty_test`.`user_activities` (
    `id` INT PRIMARY KEY AUTO_INCREMENT,
    `user_id` INT NOT NULL,
    `activity_type` VARCHAR(50) NOT NULL,
    `activity_data` JSON,
    `created_at` TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (`user_id`) REFERENCES `users`(`id`) ON DELETE CASCADE,
    INDEX `idx_user_activities_user_id` (`user_id`),
    INDEX `idx_user_activities_created_at` (`created_at`),
    INDEX `idx_user_activities_type` (`activity_type`)
)
ENGINE = InnoDB
DEFAULT CHARACTER SET = utf8mb4
COLLATE = utf8mb4_bin;

CREATE TABLE IF NOT EXISTS `rusty_test`.`user_sessions` (
    `id` INT PRIMARY KEY AUTO_INCREMENT,
    `user_id` INT NOT NULL,
    `login_time` TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    `logout_time` TIMESTAMP NULL,
    `ip_address` VARCHAR(45),
    `user_agent` TEXT,
    FOREIGN KEY (`user_id`) REFERENCES `users`(`id`) ON DELETE CASCADE,
    INDEX `idx_user_sessions_user_id` (`user_id`),
    INDEX `idx_user_sessions_login_time` (`login_time`)
)
ENGINE = InnoDB
DEFAULT CHARACTER SET = utf8mb4
COLLATE = utf8mb4_bin;

CREATE TABLE IF NOT EXISTS `rusty_test`.`transactions` (
    `id` INT PRIMARY KEY AUTO_INCREMENT,
    `user_id` INT NOT NULL,
    `amount` DECIMAL(15,2) NOT NULL,
    `currency` VARCHAR(3) NOT NULL DEFAULT 'USD',
    `transaction_type` ENUM('debit', 'credit') NOT NULL,
    `description` TEXT,
    `created_at` TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    `updated_at` TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    FOREIGN KEY (`user_id`) REFERENCES `users`(`id`) ON DELETE CASCADE,
    INDEX `idx_transactions_user_id` (`user_id`),
    INDEX `idx_transactions_created_at` (`created_at`),
    INDEX `idx_transactions_type` (`transaction_type`)
)
ENGINE = InnoDB
DEFAULT CHARACTER SET = utf8mb4
COLLATE = utf8mb4_bin;
