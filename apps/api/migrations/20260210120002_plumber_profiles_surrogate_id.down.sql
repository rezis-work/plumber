ALTER TABLE plumber_profiles DROP CONSTRAINT IF EXISTS plumber_profiles_user_id_key;

ALTER TABLE plumber_profiles DROP CONSTRAINT plumber_profiles_pkey;

ALTER TABLE plumber_profiles ADD PRIMARY KEY (user_id);

ALTER TABLE plumber_profiles DROP COLUMN id;
