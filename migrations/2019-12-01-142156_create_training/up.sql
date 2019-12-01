-- Your SQL goes here

CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE trainings (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name TEXT NOT NULL,
    description TEXT,
    UNIQUE (name),
    image TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL
);

SELECT diesel_manage_updated_at('trainings');

INSERT INTO public.trainings (id, "name", description, image, created_at, updated_at) VALUES(uuid_generate_v4(), 'Circuit training', '   Le circuit training est une méthode d’entrainement consistant à enchaîner un certain nombre d’exercice prédéfinit en amont. L’organisation se fera en fonction de votre objectif de séance et objectif général.
   Les exercices s’exécutent les uns à la suite des autres avec un temps de récupération très réduit voir nul. Il se présente sous forme de « tour » puisque c’est un circuit, lorsque j’ai fais la totalité de mes exercices j’ai effectué 1 tour. On définit avant de débuter le nombre de tours effectués.', 'image', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);

