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

INSERT INTO public.trainings (id, "name", description, image, created_at, updated_at) VALUES(uuid_generate_v4(), 'Hit', 'Le HIIT (Hight Intensity Interval Training), ou Entraînement par Intervalles à Haute Intensité en français, est une méthode d''entraînement qui consiste à alterner des périodes courtes d''effort intenses avec des temps de récupération également très brefs.', 'image', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);

INSERT INTO public.trainings (id, "name", description, image, created_at, updated_at) VALUES(uuid_generate_v4(), 'Split', 'Dans ce programme, on va faire une split routine, c''est-à-dire essayer de séparer au mieux chaque groupe musculaire pour le travailler de manière séparée. Avec ce type de programme, on cherche à solliciter chaque muscle une seule fois par semaine.2 ', 'image', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);
