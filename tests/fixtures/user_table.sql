INSERT INTO "user_table"
(id, username, password_hash, is_superuser)
VALUES
-- password: "12345"
(1, "admin", "$2b$12$YekZMZ9vDYzH4BXZbTMaJOhqhbVyQN2IvNyLEJRpFj2UUky/otnd6", 1),
(2, "manager", "$2b$12$YekZMZ9vDYzH4BXZbTMaJOhqhbVyQN2IvNyLEJRpFj2UUky/otnd6", 0);
