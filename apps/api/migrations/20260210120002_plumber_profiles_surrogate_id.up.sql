-- Implementation 003 §4: plumber-scoped FKs target plumber_profiles.id; surrogate key before child tables.

ALTER TABLE plumber_profiles ADD COLUMN id UUID NOT NULL DEFAULT gen_random_uuid();

ALTER TABLE plumber_profiles DROP CONSTRAINT plumber_profiles_pkey;

ALTER TABLE plumber_profiles ADD PRIMARY KEY (id);

ALTER TABLE plumber_profiles
    ADD CONSTRAINT plumber_profiles_user_id_key UNIQUE (user_id);
