DROP TABLE IF EXISTS gestures CASCADE;
DROP TABLE IF EXISTS descriptions CASCADE;
DROP TABLE IF EXISTS meanings CASCADE;
DROP TABLE IF EXISTS pictures CASCADE;
DROP TABLE IF EXISTS users CASCADE;

CREATE TABLE gestures (
	id_gesture 	UUID PRIMARY KEY,
    tags		text[]
);

CREATE TABLE descriptions (
	id_description 	UUID PRIMARY KEY,
	id_gesture		UUID REFERENCES gestures ON DELETE CASCADE NOT NULL,
    val				text NOT NULL,
    langs			text[]
);

CREATE TABLE meanings (
    id_meaning		UUID PRIMARY KEY,
    id_description 	UUID REFERENCES descriptions ON DELETE CASCADE,
   	id_gesture 		UUID REFERENCES gestures ON DELETE CASCADE,
    val				text NOT NULL,
    langs			text[]
    CHECK (id_description IS NULL OR id_gesture IS NULL)
);

CREATE TABLE pictures (
	id_picture 	UUID PRIMARY KEY,
	id_gesture 	UUID REFERENCES gestures ON DELETE CASCADE NOT NULL,
    langs		text[]
);

CREATE TABLE users
(
    username       text PRIMARY KEY,
    PASSWORD    text
);