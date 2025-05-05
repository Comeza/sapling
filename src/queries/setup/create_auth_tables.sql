CREATE TABLE user (
    user_id  INTEGER    PRIMARY KEY,
    username VARCHAR    UNIQUE NOT NULL,
    password VARCHAR    NOT NULL,
    created  INTEGER    DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE session (
    token   VARCHAR(32) NOT NULL PRIMARY KEY,
    user_id INTEGER     REFERENCES user(user_id),
    created INTEGER     DEFAULT CURRENT_TIMESTAMP
);
