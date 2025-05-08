SELECT * FROM product_group
JOIN [group] ON [group].group_id = product_group.group_id
WHERE product_group.ean = ?;