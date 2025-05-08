SELECT * FROM product_tag
JOIN tag ON tag.tag_id = product_tag.tag_id
WHERE product_tag.ean = ?;
