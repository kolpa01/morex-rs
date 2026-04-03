CREATE TABLE IF NOT EXISTS users (
    user_id bigint PRIMARY KEY, 
    wallet bigint NOT NULL, 
    bank bigint NOT NULL, 
    hp bigint NOT NULL, 
    level bigint NOT NULL, 
    xp bigint NOT NULL, 
    total_xp bigint NOT NULL, 
    version varchar(8) NOT NULL, 
    timestamp timestamptz(0) NOT NULL
);

CREATE TABLE IF NOT EXISTS emojis (
    path varchar(255) PRIMARY KEY, 
    emoji_id bigint NOT NULL, 
    checksum varchar(64) NOT NULL
)
