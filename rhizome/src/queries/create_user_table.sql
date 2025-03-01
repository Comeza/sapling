CREATE TABLE IF NOT EXISTS user (
    user_id     INTEGER PRIMARY KEY,
    username    VARCHAR UNIQUE NOT NULL ,
    hashed_password VARCHAR NOT NULL,
    created         INTEGER DEFAULT CURRENT_TIMESTAMP
);