-- Your SQL goes here
CREATE TABLE users (
    id         SERIAL       PRIMARY KEY,
    email      TEXT         NOT NULL UNIQUE,
    username   TEXT         NOT NULL UNIQUE,
    password   TEXT         NOT NULL,
    img        VARCHAR(255),
    created_at TIMESTAMPTZ  NOT NULL,
    updated_at TIMESTAMPTZ
);
CREATE TABLE article (
    id          SERIAL       PRIMARY KEY,
    user_id     INTEGER      NOT NULL,
    title       VARCHAR(255) NOT NULL UNIQUE,
    description TEXT,
    img         VARCHAR(255),
    completed   BOOLEAN      NOT NULL DEFAULT false,
    created_at  TIMESTAMPTZ  NOT NULL,
    updated_at  TIMESTAMPTZ,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);
CREATE TABLE provision_d (
    id          SERIAL       PRIMARY KEY,
    user_id     INTEGER      NOT NULL,
    title       VARCHAR(255) NOT NULL UNIQUE,
    description TEXT,
    st_date     Date,
    en_date     Date,
    s_dates     Date[],
    e_dates     Date[],
    dates       Date[],
    completed   BOOLEAN      NOT NULL DEFAULT false,
    created_at  TIMESTAMPTZ  NOT NULL,
    updated_at  TIMESTAMPTZ,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);
CREATE TABLE provision_h (
    id          SERIAL       PRIMARY KEY,
    user_id     INTEGER      NOT NULL,
    title       VARCHAR(255) NOT NULL UNIQUE,
    description TEXT,
    st_hour     TIMESTAMP,
    en_hour     TIMESTAMP,
    s_hours     TIMESTAMP[],
    e_hours     TIMESTAMP[],
    hours       TIMESTAMP[],
    completed   BOOLEAN      NOT NULL DEFAULT false,
    created_at  TIMESTAMPTZ  NOT NULL,
    updated_at  TIMESTAMPTZ,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);
CREATE TABLE booking (
    id             SERIAL       PRIMARY KEY,
    user_id        INTEGER      NOT NULL,
    provision_d_id INTEGER,
    provision_h_id INTEGER,
    title          VARCHAR(255) NOT NULL UNIQUE,
    description    TEXT,
    st_date        Date,
    en_date        Date,
    st_hour        TIMESTAMP,
    en_hour        TIMESTAMP,
    completed      BOOLEAN      NOT NULL DEFAULT false,
    created_at     TIMESTAMPTZ  NOT NULL,
    updated_at     TIMESTAMPTZ,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    FOREIGN KEY (provision_d_id) REFERENCES provision_d(id) ON DELETE CASCADE,
    FOREIGN KEY (provision_h_id) REFERENCES provision_h(id) ON DELETE CASCADE
);
CREATE TABLE sessions (
    session_token BYTEA             PRIMARY KEY,
    id integer REFERENCES users(id) ON DELETE CASCADE
);