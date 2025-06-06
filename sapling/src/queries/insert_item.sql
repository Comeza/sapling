INSERT INTO item (ean, cost)
VALUES (?, ?)
RETURNING *;
