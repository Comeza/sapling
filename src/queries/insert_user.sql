INSERT INTO user (username, password)
VALUES (?, ?)
RETURNING user_id;
