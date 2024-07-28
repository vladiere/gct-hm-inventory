CREATE OR REPLACE TABLE `user_detail` (
	`id` INT NOT NULL AUTO_INCREMENT UNIQUE,
	`first_name` VARCHAR(255) NOT NULL,
	`last_name` VARCHAR(255) NOT NULL,
	`email_address` VARCHAR(255) NOT NULL,
	`id_number` INT NOT NULL,
	`department` ENUM("'it'", "'educ'", "'hm'") NOT NULL,
	`user_status` ENUM("'active'", "'inactive'") NOT NULL DEFAULT 'active',
	`ctime` TIMESTAMP NOT NULL DEFAULT current_timestamp,
	`mtime` TIMESTAMP NOT NULL DEFAULT current_timestamp,
	PRIMARY KEY(`id`)
);

CREATE OR REPLACE TABLE `user_login` (
	`id` INT NOT NULL AUTO_INCREMENT UNIQUE,
	`user_id` INT NOT NULL,
	`username` VARCHAR(255) NOT NULL,
	`password` VARCHAR(255) NOT NULL,
	`role` ENUM("'super'", "'admin'", "'user'") NOT NULL,
	`secret_code` VARCHAR(255) NOT NULL,
	`ctime` TIMESTAMP NOT NULL DEFAULT current_timestamp,
	`mtime` TIMESTAMP NOT NULL DEFAULT current_timestamp,
	PRIMARY KEY(`id`)
);

CREATE OR REPLACE TABLE `item` (
	`id` INT NOT NULL AUTO_INCREMENT UNIQUE,
	`category_id` INT NOT NULL,
	`name` VARCHAR(255) NOT NULL,
	`count` INT NOT NULL,
	`total` INT NOT NULL,
	`item_status` ENUM("'active'", "'inactive'", "'remove'", "'damage'", "'repair'") NOT NULL DEFAULT 'active',
	`ctime` TIMESTAMP NOT NULL DEFAULT current_timestamp,
	`mtime` TIMESTAMP NOT NULL DEFAULT current_timestamp,
	PRIMARY KEY(`id`)
);

CREATE OR REPLACE TABLE `category` (
	`id` INT NOT NULL AUTO_INCREMENT UNIQUE,
	`name` VARCHAR(255) NOT NULL,
	`ctime` TIMESTAMP NOT NULL,
	`mtime` TIMESTAMP NOT NULL,
	PRIMARY KEY(`id`)
);

CREATE OR REPLACE TABLE `transaction` (
	`id` INT NOT NULL AUTO_INCREMENT UNIQUE,
	`item_id` INT NOT NULL,
	`user_id` INT NOT NULL,
	`transac_date` TIMESTAMP NOT NULL,
	`issued` TIMESTAMP NOT NULL,
	`ctime` TIMESTAMP NOT NULL DEFAULT current_timestamp,
	`mtime` TIMESTAMP NOT NULL DEFAULT current_timestamp,
	PRIMARY KEY(`id`)
);

CREATE OR REPLACE TABLE `notification` (
	`id` INT NOT NULL AUTO_INCREMENT UNIQUE,
	`item_id` INT NOT NULL,
	`user_id` INT NOT NULL,
	`title` VARCHAR(255) NOT NULL,
	`message` TEXT(65535) NOT NULL,
	`notif_type` ENUM("'info'", "'alert'", "'succes'", "'warning'") NOT NULL DEFAULT 'info',
	`notif_status` ENUM("'read'", "'unread'", "'archived'") NOT NULL DEFAULT 'unread',
	`ctime` TIMESTAMP NOT NULL DEFAULT current_timestamp,
	`mtime` TIMESTAMP NOT NULL DEFAULT current_timestamp,
	PRIMARY KEY(`id`)
);

ALTER TABLE `user_login`
ADD FOREIGN KEY(`user_id`) REFERENCES `user_detail`(`id`)
ON UPDATE NO ACTION ON DELETE NO ACTION;
ALTER TABLE `category`
ADD FOREIGN KEY(`id`) REFERENCES `item`(`category_id`)
ON UPDATE NO ACTION ON DELETE NO ACTION;
ALTER TABLE `transaction`
ADD FOREIGN KEY(`item_id`) REFERENCES `item`(`id`)
ON UPDATE NO ACTION ON DELETE NO ACTION;
ALTER TABLE `transaction`
ADD FOREIGN KEY(`user_id`) REFERENCES `user_login`(`id`)
ON UPDATE NO ACTION ON DELETE NO ACTION;
ALTER TABLE `notification`
ADD FOREIGN KEY(`item_id`) REFERENCES `item`(`id`)
ON UPDATE NO ACTION ON DELETE NO ACTION;
ALTER TABLE `notification`
ADD FOREIGN KEY(`user_id`) REFERENCES `user_detail`(`id`)
ON UPDATE NO ACTION ON DELETE NO ACTION;
