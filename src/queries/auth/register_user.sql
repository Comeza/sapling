INSERT INTO user (username, password)
VALUES (?, ?)
RETURNING *;