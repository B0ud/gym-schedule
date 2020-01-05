-- Your SQL goes here

CREATE TABLE muscle_tags (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name TEXT NOT NULL UNIQUE,
    code TEXT NOT NULL UNIQUE,
    description TEXT,
    image TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL
);

SELECT diesel_manage_updated_at('muscle_tags');

INSERT INTO public.muscle_tags (id, "name", code, description, image, created_at, updated_at)  VALUES(uuid_generate_v4(), 'Pectoraux', 'PECS', 'Exercices pour les pectoraux', NULL,  CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);
INSERT INTO public.muscle_tags (id, "name", code, description, image, created_at, updated_at)  VALUES(uuid_generate_v4(), 'Deltoides', 'DELTOIDES', 'Exercices pour les deltoides', NULL,  CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);
INSERT INTO public.muscle_tags (id, "name", code, description, image, created_at, updated_at)  VALUES(uuid_generate_v4(), 'Abdominaux', 'ABDOS', 'Exercices pour les abdominaux', NULL,  CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);
INSERT INTO public.muscle_tags (id, "name", code, description, image, created_at, updated_at)  VALUES(uuid_generate_v4(), 'Quadriceps', 'QUADRIS', 'Exercices pour les quadriceps', NULL,  CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);
INSERT INTO public.muscle_tags (id, "name", code, description, image, created_at, updated_at)  VALUES(uuid_generate_v4(), 'Biceps', 'BICEPS', 'Exercices pour les biceps', NULL,  CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);
INSERT INTO public.muscle_tags (id, "name", code, description, image, created_at, updated_at)  VALUES(uuid_generate_v4(), 'Avants-bras', 'AVANTS-BRAS', 'Exercices pour les avants-bras', NULL,  CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);
INSERT INTO public.muscle_tags (id, "name", code, description, image, created_at, updated_at)  VALUES(uuid_generate_v4(), 'Trapezes', 'TRAPEZES', 'Exercices pour les trapezes', NULL,  CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);
INSERT INTO public.muscle_tags (id, "name", code, description, image, created_at, updated_at)  VALUES(uuid_generate_v4(), 'Lombaires', 'LOMBAIRES', 'Exercices pour les lombaires', NULL,  CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);
INSERT INTO public.muscle_tags (id, "name", code, description, image, created_at, updated_at)  VALUES(uuid_generate_v4(), 'Ischios', 'ISCHIOS', 'Exercices pour les ischios', NULL,  CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);
INSERT INTO public.muscle_tags (id, "name", code, description, image, created_at, updated_at)  VALUES(uuid_generate_v4(), 'Dorsaux', 'DORSAUX', 'Exercices pour les dorsaux', NULL,  CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);
INSERT INTO public.muscle_tags (id, "name", code, description, image, created_at, updated_at)  VALUES(uuid_generate_v4(), 'Fessiers', 'FESSIERS', 'Exercices pour les fessiers', NULL,  CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);
INSERT INTO public.muscle_tags (id, "name", code, description, image, created_at, updated_at)  VALUES(uuid_generate_v4(), 'Mollets', 'MOLLETS', 'Exercices pour les mollets', NULL,  CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);