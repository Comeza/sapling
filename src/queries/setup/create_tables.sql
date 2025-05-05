CREATE TABLE brand (
    brand_id    INTEGER     PRIMARY KEY,
    name        VARCHAR     NOT NULL,
    created_at  TIMESTAMP   DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE product (
    ean         INTEGER     PRIMARY KEY,
    name        VARCHAR     NOT NULL,
    description VARCHAR,
    brand_id    INTEGER     REFERENCES brand(brand_id),
    inserted_at TIMESTAMP   NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at  TIMESTAMP   NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE stock (
    stock_id    INTEGER     PRIMARY KEY,
    product_ean INTEGER     REFERENCES product(ean),
    amount      INTEGER     NOT NULL,
    inserted    TIMESTAMP   NOT NULL DEFAULT CURRENT_TIMESTAMP,
    last_used   TIMESTAMP,
    cost        INTEGER
);

CREATE TABLE product_group (
    group_id    INTEGER     PRIMARY KEY,
    name        VARCHAR     NOT NULL,
    created_at  TIMESTAMP   DEFAULT CURRENT_TIMESTAMP
);
