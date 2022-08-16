CREATE TABLE IF NOT EXISTS users (
    id SERIAL PRIMARY KEY,
    uuid UUID DEFAULT gen_random_uuid() UNIQUE,
    first_name VARCHAR NOT NULL,
    last_name VARCHAR NOT NULL,
    email VARCHAR NOT NULL UNIQUE,
    password VARCHAR NOT NULL,

    created_at TIMESTAMP NOT NULL DEFAULT now(),
    updated_at TIMESTAMP NOT NULL DEFAULT now()
);

CREATE TABLE IF NOT EXISTS projects (
    id SERIAL PRIMARY KEY,
    uuid UUID DEFAULT gen_random_uuid() UNIQUE,
    name VARCHAR NOT NULL,
    description TEXT,
    status VARCHAR,
    number SERIAL NOT NULL UNIQUE,
    created_at TIMESTAMP NOT NULL DEFAULT now(),
    updated_at TIMESTAMP NOT NULL DEFAULT now()
);

CREATE TABLE IF NOT EXISTS user_project (
    id SERIAL PRIMARY KEY,
    user_id SERIAL references users(id),
    project_id SERIAL references projects(id),
    created_at TIMESTAMP NOT NULL DEFAULT now(),
    updated_at TIMESTAMP NOT NULL DEFAULT now()
);

CREATE TABLE IF NOT EXISTS project_role (
    id SERIAL PRIMARY KEY,
    user_id SERIAL references users(id),
    project_id SERIAL references projects(id),
    name VARCHAR NOT NULL DEFAULT 'false',
    created_at TIMESTAMP NOT NULL DEFAULT now(),
    updated_at TIMESTAMP NOT NULL DEFAULT now()
);


INSERT INTO users (first_name, last_name, email, password) 
VALUES ('prenom', 'nom', 'admin@example.com', 'secret');

INSERT INTO projects (uuid, name, description, status, number) 
VALUES ('84f8afa1-36e8-438e-a824-23e213a211d6', 'project 3d', 'Test de mon projet', 'en cours', '1');

INSERT INTO user_project (user_id, project_id) 
VALUES ('1', '1');

INSERT INTO project_role (user_id, project_id, name) 
VALUES ('1', '1', 'creator');