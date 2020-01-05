-- Your SQL goes here

CREATE TABLE exercises (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name TEXT NOT NULL UNIQUE,
    description TEXT,
    category TEXT NOT NULL,
    category_icon TEXT,
    image TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL
);

CREATE TABLE asso_muscle_tags_exercises (
    exercise_id UUID REFERENCES exercises (id),
    muscle_tags UUID REFERENCES muscle_tags(id),
    CONSTRAINT muscle_tag_exercices_pk PRIMARY KEY (exercise_id, muscle_tags)
);

SELECT diesel_manage_updated_at('exercises');


--PECS

INSERT INTO public.exercises (id, "name", description, category, category_icon ,image, created_at, updated_at) VALUES (uuid_generate_v4(),'Développé assis à la machine convergente',NULL,'EXERCICES DE BASE','*',NULL,CURRENT_TIMESTAMP,CURRENT_TIMESTAMP);
INSERT INTO public.exercises (id, "name", description, category, category_icon ,image, created_at, updated_at) VALUES (uuid_generate_v4(),'Développé couché',NULL,'EXERCICES DE BASE','*',NULL,CURRENT_TIMESTAMP,CURRENT_TIMESTAMP);
INSERT INTO public.exercises (id, "name", description, category, category_icon ,image, created_at, updated_at) VALUES (uuid_generate_v4(),'Développé couché avec haltères',NULL,'EXERCICES DE BASE','*',NULL,CURRENT_TIMESTAMP,CURRENT_TIMESTAMP);
INSERT INTO public.exercises (id, "name", description, category, category_icon ,image, created_at, updated_at) VALUES (uuid_generate_v4(),'Développé couché à la machine convergente',NULL,'EXERCICES DE BASE','*',NULL,CURRENT_TIMESTAMP,CURRENT_TIMESTAMP);
INSERT INTO public.exercises (id, "name", description, category, category_icon ,image, created_at, updated_at) VALUES (uuid_generate_v4(),'Développé décliné',NULL,'EXERCICES DE BASE','*',NULL,CURRENT_TIMESTAMP,CURRENT_TIMESTAMP);
INSERT INTO public.exercises (id, "name", description, category, category_icon ,image, created_at, updated_at) VALUES (uuid_generate_v4(),'Développé décliné avec haltères',NULL,'EXERCICES DE BASE','*',NULL,CURRENT_TIMESTAMP,CURRENT_TIMESTAMP);
INSERT INTO public.exercises (id, "name", description, category, category_icon ,image, created_at, updated_at) VALUES (uuid_generate_v4(),'Développé incliné',NULL,'EXERCICES DE BASE','*',NULL,CURRENT_TIMESTAMP,CURRENT_TIMESTAMP);
INSERT INTO public.exercises (id, "name", description, category, category_icon ,image, created_at, updated_at) VALUES (uuid_generate_v4(),'Développé incliné avec haltères',NULL,'EXERCICES DE BASE','*',NULL,CURRENT_TIMESTAMP,CURRENT_TIMESTAMP);


INSERT INTO public.asso_muscle_tags_exercises(exercise_id, muscle_tags) SELECT (SELECT id from  public.exercises WHERE "name" = 'Développé assis à la machine convergente'), (SELECT id from public.muscle_tags WHERE "name" = 'Pectoraux');
INSERT INTO public.asso_muscle_tags_exercises(exercise_id, muscle_tags) SELECT (SELECT id from  public.exercises WHERE "name" = 'Développé couché'), (SELECT id from public.muscle_tags WHERE "name" = 'Pectoraux');
INSERT INTO public.asso_muscle_tags_exercises(exercise_id, muscle_tags) SELECT (SELECT id from  public.exercises WHERE "name" = 'Développé couché avec haltères'), (SELECT id from public.muscle_tags WHERE "name" = 'Pectoraux');
INSERT INTO public.asso_muscle_tags_exercises(exercise_id, muscle_tags) SELECT (SELECT id from  public.exercises WHERE "name" = 'Développé couché à la machine convergente'), (SELECT id from public.muscle_tags WHERE "name" = 'Pectoraux');
INSERT INTO public.asso_muscle_tags_exercises(exercise_id, muscle_tags) SELECT (SELECT id from  public.exercises WHERE "name" = 'Développé décliné'), (SELECT id from public.muscle_tags WHERE "name" = 'Pectoraux');
INSERT INTO public.asso_muscle_tags_exercises(exercise_id, muscle_tags) SELECT (SELECT id from  public.exercises WHERE "name" = 'Développé décliné avec haltères'), (SELECT id from public.muscle_tags WHERE "name" = 'Pectoraux');
INSERT INTO public.asso_muscle_tags_exercises(exercise_id, muscle_tags) SELECT (SELECT id from  public.exercises WHERE "name" = 'Développé incliné'), (SELECT id from public.muscle_tags WHERE "name" = 'Pectoraux');
INSERT INTO public.asso_muscle_tags_exercises(exercise_id, muscle_tags) SELECT (SELECT id from  public.exercises WHERE "name" = 'Développé incliné avec haltères'), (SELECT id from public.muscle_tags WHERE "name" = 'Pectoraux');
