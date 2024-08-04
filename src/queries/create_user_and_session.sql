INSERT INTO user (username, password)
VALUES (?, ?);

INSERT INTO session (user_id)
SELECT last_insert_rowid() AS user_id
RETURNING *;
