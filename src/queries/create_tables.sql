CREATE TABLE IF NOT EXISTS user (
    user_id     INTEGER NOT NULL,
    username    VARCHAR(20),
    created     INT DEFAULT CURRENT_TIMESTAMP,

    PRIMARY KEY(user_id)
);

CREATE TABLE IF NOT EXISTS entity_category (
    category_id     INTEGER NOT NULL,
    name            VARCHAR NOT NULL,
    description     VARCHAR,
    updated         INT DEFAULT CURRENT_TIMESTAMP,

    PRIMARY KEY(category_id)
);

CREATE TABLE IF NOT EXISTS entity (
    entity_id   INTEGER NOT NULL,
    name        VARCHAR NOT NULL,
    category_id INTEGER,
    updated     INT DEFAULT CURRENT_TIMESTAMP,

    PRIMARY KEY(entity_id),
    FOREIGN KEY(category_id) REFERENCES entity_category(category_id)
);



CREATE TABLE IF NOT EXISTS product (
	product_id  INTEGER NOT NULL,
	name        VARCHAR NOT NULL,
    description VARCHAR,
    updated     INT DEFAULT CURRENT_TIMESTAMP,

    PRIMARY KEY(product_id)
);

CREATE TABLE IF NOT EXISTS stock (
    stock_id    INTEGER NOT NULL,
    product_id  INTEGER NOT NULL,
    amount      INT UNSIGNED,
    updated     INT DEFAULT CURRENT_TIMESTAMP,

    PRIMARY KEY(stock_id),
    FOREIGN KEY(product_id) REFERENCES product(product_id)
);
