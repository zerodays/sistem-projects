CREATE TABLE users
(
    project_id INTEGER REFERENCES projects (id) ON DELETE CASCADE NOT NULL,
    user_id    TEXT                                               NOT NULL,

    UNIQUE (project_id, user_id)
);
