INSERT INTO stock (ean, cost)
VALUES (?, ?)
RETURNING *;
