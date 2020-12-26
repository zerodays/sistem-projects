CREATE TABLE projects
(
    id           SERIAL PRIMARY KEY,
    name         TEXT                                               NOT NULL,
    description  TEXT                                               NOT NULL,

    date_created TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL
);

CREATE TABLE phases
(
    id           SERIAL PRIMARY KEY,
    project_id   INTEGER REFERENCES projects (id) ON DELETE CASCADE NOT NULL,

    name         TEXT                                               NOT NULL,
    description  TEXT                                               NOT NULL,

    deadline     TIMESTAMP WITH TIME ZONE                           NOT NULL,

    date_created TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL
);

CREATE TABLE tasks
(
    id          SERIAL PRIMARY KEY,
    phase_id    INTEGER REFERENCES phases (id) ON DELETE CASCADE NOT NULL,

    index       INTEGER DEFAULT 0                                NOT NULL,

    name        TEXT                                             NOT NULL,
    description TEXT                                             NOT NULL,
    completed   BOOLEAN DEFAULT FALSE                            NOT NULL
);
