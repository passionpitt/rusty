USE `db`;

CREATE TABLE `rusty`.`users` (
  `id` BIGINT NOT NULL AUTO_INCREMENT,
  `name` VARCHAR(64) CHARACTER SET 'utf8mb4' COLLATE 'utf8mb4_bin' NULL,
  `email` VARCHAR(64) CHARACTER SET 'utf8mb4' COLLATE 'utf8mb4_bin' NULL,
);

INSERT INTO `db`.`users` SET name = "mindo", email = "thisismindo@foobar.net";
