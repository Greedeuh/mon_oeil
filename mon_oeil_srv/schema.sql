DROP TABLE IF EXISTS gestures CASCADE;
DROP TABLE IF EXISTS descriptions CASCADE;
DROP TABLE IF EXISTS meanings CASCADE;
DROP TABLE IF EXISTS pictures CASCADE;
DROP TABLE IF EXISTS users CASCADE;

CREATE TABLE gestures (
	id_gesture 		UUID PRIMARY KEY,
    tags			text[] NOT NULL,
	creation_date	TIMESTAMP NOT NULL DEFAULT NOW(),
    document 		tsvector
);

CREATE TABLE descriptions (
	id_description 	UUID PRIMARY KEY,
	id_gesture		UUID REFERENCES gestures ON DELETE CASCADE NOT NULL,
    val				text NOT NULL,
    langs			text[] NOT NULL,
	creation_date	TIMESTAMP NOT NULL DEFAULT NOW(),
	document 		tsvector
);

CREATE TABLE meanings (
    id_meaning		UUID PRIMARY KEY,
    id_description 	UUID REFERENCES descriptions ON DELETE CASCADE,
   	id_gesture 		UUID REFERENCES gestures ON DELETE CASCADE,
    val				text NOT NULL,
    langs			text[] NOT NULL,
    creation_date	TIMESTAMP NOT NULL DEFAULT NOW(),
	document 		tsvector,
    CHECK (id_description IS NULL OR id_gesture IS NULL)
);

CREATE TABLE pictures (
	id_picture 		UUID PRIMARY KEY,
	id_gesture 		UUID REFERENCES gestures ON DELETE CASCADE NOT NULL,
	langs			text[] NOT NULL,
    format			text NOT NULL,
	creation_date	TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE TABLE users
(
    username    	text PRIMARY KEY,
    PASSWORD    	text NOT NULL,
	creation_date	TIMESTAMP NOT NULL DEFAULT NOW()
);

------- VIEWS             -------

CREATE VIEW meanings_with_gesture_id AS
    SELECT meanings.*, descriptions.id_gesture as id_description_gesture
    FROM meanings
	LEFT JOIN descriptions ON (meanings.id_description = descriptions.id_description);

CREATE VIEW searchable as
	SELECT g.id_gesture, g.document FROM gestures as g
	UNION 
	SELECT d.id_gesture, d.document FROM descriptions as d
	UNION 
	SELECT COALESCE(m.id_description_gesture, m.id_gesture) as id_gesture , m.document FROM meanings_with_gesture_id as m;

------- RESEARCH TRIGGERS -------
------- Gestures          -------

DROP FUNCTION IF EXISTS gestures_document_trigger();

CREATE FUNCTION gestures_document_trigger() RETURNS trigger AS $$
	BEGIN
		new.document := array_to_tsvector(new.tags);
		return new;
	END
$$ LANGUAGE plpgsql;

CREATE TRIGGER  gestures_document_update BEFORE INSERT OR UPDATE
	ON gestures FOR EACH ROW EXECUTE PROCEDURE gestures_document_trigger();

CREATE INDEX gesture_document_index
	ON gestures
	USING GIN (document);
	
------- Descriptions -------

DROP FUNCTION IF EXISTS descriptions_document_trigger();

CREATE FUNCTION descriptions_document_trigger() RETURNS trigger AS $$
	BEGIN
		new.document := to_tsvector('french', new.val);
		return new;
	END
$$ LANGUAGE plpgsql;

CREATE TRIGGER  descriptions_document_update BEFORE INSERT OR UPDATE
	ON descriptions FOR EACH ROW EXECUTE PROCEDURE descriptions_document_trigger();

CREATE INDEX descriptions_document_index
	ON descriptions
	USING GIN (document);
	
------- Meanings -------

DROP FUNCTION IF EXISTS meanings_document_trigger();

CREATE FUNCTION meanings_document_trigger() RETURNS trigger AS $$
	BEGIN
		new.document := to_tsvector('french', new.val);
		return new;
	END
$$ LANGUAGE plpgsql;

CREATE TRIGGER  meanings_document_update BEFORE INSERT OR UPDATE
	ON meanings FOR EACH ROW EXECUTE PROCEDURE meanings_document_trigger();

CREATE INDEX meanings_document_index
	ON meanings
	USING GIN (document);