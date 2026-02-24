-- Your SQL goes here
CREATE TABLE `user_profiles` (
    `user_id` INTEGER NOT NULL,
    `bio` TEXT,
    `avatar_url` VARCHAR(255),
    PRIMARY KEY (`user_id`),
    FOREIGN KEY (`user_id`) REFERENCES `users` (`user_id`) ON DELETE CASCADE
);
