CREATE TABLE IF NOT EXISTS session (
    token   VARCHAR(30) DEFAULT (lower(hex(randomblob(30)))),
    user_id INTEGER     NOT NULL,
    created INTEGER     DEFAULT CURRENT_TIMESTAMP,

    PRIMARY KEY(token),
    FOREIGN KEY(user_id) REFERENCES user(user_id)
)
