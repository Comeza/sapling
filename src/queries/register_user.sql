INSERT INTO user (username, hashed_password)
VALUES (?, ?);

INSERT INTO session (user_id)
VALUES (last_insert_rowid())
RETURNING *;