CREATE TABLE IF NOT EXISTS product (
    ean VARCHAR(20),
    product_name VARCHAR NOT NULL,
    common_name VARCHAR,

    PRIMARY KEY(ean)
);