INSERT INTO session (token, user_id)
VALUES (?, ?)
RETURNING *;
