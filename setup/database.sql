CREATE DATABASE IF NOT EXISTS `rusty` DEFAULT CHARACTER SET utf8mb4 COLLATE utf8mb4_bin ;

USE `rusty`;

CREATE TABLE `rusty`.`users` (
  `id` BIGINT NOT NULL AUTO_INCREMENT,
  `name` VARCHAR(64) CHARACTER SET 'utf8mb4' COLLATE 'utf8mb4_bin' NULL,
  `username` VARCHAR(64) CHARACTER SET 'utf8mb4' COLLATE 'utf8mb4_bin' NULL,
  `created_at` TIMESTAMP NULL DEFAULT CURRENT_TIMESTAMP,
  `updated_at` TIMESTAMP NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
  PRIMARY KEY (`id`),
  UNIQUE INDEX `username_UNIQUE` (`username` ASC) VISIBLE,
  INDEX `created_at_idx` (`created_at` ASC) VISIBLE,
  INDEX `updated_at_idx` (`updated_at` ASC) VISIBLE);

INSERT INTO `rusty`.`users` SET name = "mindo", username = "thisismindo";
INSERT INTO `rusty`.`users` SET name = "john doe", username = "john.doe";
INSERT INTO `rusty`.`users` SET name = "jane doe", username = "jane.doe";
