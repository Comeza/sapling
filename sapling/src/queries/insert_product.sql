INSERT INTO product (ean, name)
VALUES (?, ?)
RETURNING *;