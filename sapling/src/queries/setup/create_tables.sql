CREATE TABLE brand (
    brand_id    INTEGER     PRIMARY KEY,
    name        VARCHAR     NOT NULL,
    created_at  TIMESTAMP   NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE tag (
    tag_id      INTEGER     PRIMARY KEY,
    name        VARCHAR     NOT NULL
);

CREATE TABLE product_tag (
    ean         INTEGER     REFERENCES product(ean),
    tag_id      INTEGER     REFERENCES tag(tag_id),
    created_at  TIMESTAMP   NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE [group] (
    group_id    INTEGER     PRIMARY KEY,
    name        VARCHAR     NOT NULL
);

CREATE TABLE product_group (
    ean         INTEGER     REFERENCES product(ean),
    group_id    INTEGER     REFERENCES [group](group_id)
);

CREATE TABLE product (
    ean         INTEGER     PRIMARY KEY,
    name        VARCHAR     NOT NULL,
    description VARCHAR,
    brand_id    INTEGER     REFERENCES brand(brand_id),
    inserted_at TIMESTAMP   NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at  TIMESTAMP   NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE item (
    item_id    INTEGER     PRIMARY KEY,
    ean         INTEGER     REFERENCES product(ean),
    created_at  TIMESTAMP   NOT NULL DEFAULT CURRENT_TIMESTAMP,
    cost        INTEGER
);
