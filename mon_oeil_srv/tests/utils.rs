#[allow(dead_code)]
pub mod setup {

    use lazy_static::lazy_static;
    use postgres::NoTls;
    use std::env::var;
    use std::net::TcpListener;

    use mon_oeil_db::GestureClientPool;
    use mon_oeil_storage::*;

    pub struct ConfTest {
        pub db_pool: GestureClientPool,
        pub hs256_private_key: String,
    }

    lazy_static! {
        pub static ref CONF: ConfTest = {
            dotenv::dotenv().ok();

            let db_pool = mon_oeil_db::connect_db();
            ConfTest {
                db_pool,
                hs256_private_key: var("HS256_PRIVATE_KEY")
                    .expect("Need HS256_PRIVATE_KEY env var for test."),
            }
        };
    }

    pub fn spawn_app() -> String {
        spawn_app_with_storage(Storage::default)
    }

    pub fn spawn_app_with_storage(build_storage: fn() -> Storage) -> String {
        let _ = env_logger::builder().is_test(true).try_init();

        let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
        // We retrieve the port assigned to us by the OS
        let port = listener.local_addr().unwrap().port();

        let server = mon_oeil_srv::run_with_storage(listener, build_storage)
            .expect("Failed to spawn our app.");

        let _ = tokio::spawn(server);

        format!("http://127.0.0.1:{}", port)
    }

    pub const ADMIN_TOKEN: &str = "Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJleHAiOjIyMDM0MDcxOTgsImxldmVsIjoiQWRtaW4ifQ.RLE2du-ICZ0mlFl02YytZC02Xk0U5qyNRBxhi_-SvY8";

    pub fn insert_gesture_without_links() {
        let mut client = connect();
        client.execute(r#"INSERT INTO gestures(id_gesture, tags) VALUES ('ce27c124-e47b-490f-b8fe-3f37d5dbbef6', '{"tag1", "tag2"}')"#, &[]).unwrap();
    }

    pub fn insert_gesture_with_meaning() {
        let mut client = connect();
        client.execute(r#"INSERT INTO gestures(id_gesture, tags) VALUES ('ce27c124-e47b-490f-b8fe-3f37d5dbbef6', '{"tag1", "tag2"}')"#, &[]).unwrap();
        client.execute(r#"INSERT INTO meanings(
            id_meaning, id_description, id_gesture, val, langs)
            VALUES ('59c25147-021e-4584-9c35-97cbf060cc89', null, 'ce27c124-e47b-490f-b8fe-3f37d5dbbef6', 'Un petit meaning', '{"fr", "us"}');"#, &[]).unwrap();
    }

    pub fn insert_gesture_with_description() {
        let mut client = connect();
        client.execute(r#"INSERT INTO gestures(id_gesture, tags) VALUES ('ce27c124-e47b-490f-b8fe-3f37d5dbbef6', '{"tag1", "tag2"}')"#, &[]).unwrap();
        client.execute(r#"INSERT INTO descriptions(
                id_description, id_gesture, val, langs)
                VALUES ('2ae70884-97bd-401d-8f43-d1778d4502d2', 'ce27c124-e47b-490f-b8fe-3f37d5dbbef6', 'Une petite description', '{"fr", "us"}');"#, &[]).unwrap();
    }

    pub fn insert_gesture_with_picture() {
        let mut client = connect();
        client.execute(r#"INSERT INTO gestures(id_gesture, tags) VALUES ('ce27c124-e47b-490f-b8fe-3f37d5dbbef6', '{"tag1", "tag2"}')"#, &[]).unwrap();
        client.execute(r#"INSERT INTO pictures(
            id_picture, id_gesture, langs, format)
            VALUES ('283e7b04-7c13-4154-aafe-8e55b6960fe3', 'ce27c124-e47b-490f-b8fe-3f37d5dbbef6', '{"fr", "us"}', 'png');"#, &[]).unwrap();
    }

    pub fn insert_gesture_with_description_with_meaning() {
        let mut client = connect();
        client.execute(r#"INSERT INTO gestures(id_gesture, tags) VALUES ('ce27c124-e47b-490f-b8fe-3f37d5dbbef6', '{"tag1", "tag2"}')"#, &[]).unwrap();
        client.execute(r#"INSERT INTO descriptions(
                id_description, id_gesture, val, langs)
                VALUES ('2ae70884-97bd-401d-8f43-d1778d4502d2', 'ce27c124-e47b-490f-b8fe-3f37d5dbbef6', 'Une petite description', '{"fr", "us"}');"#, &[]).unwrap();
        client.execute(r#"INSERT INTO meanings(
                    id_meaning, id_description, id_gesture, val, langs)
                    VALUES ('e2c6eee0-49a7-49c4-9a0f-a9c6e6f668d8', '2ae70884-97bd-401d-8f43-d1778d4502d2', null, 'Un petit meaning', '{"fr", "us"}');"#, &[]).unwrap();
    }

    pub fn insert_user() {
        let mut client = connect();
        client
            .execute(
                r#"INSERT INTO users(username, password) VALUES ('user_test', 'password_test')"#,
                &[],
            )
            .unwrap();
    }

    pub fn insert_2_gestures_with_full_links() {
        let mut client = connect();
        client.execute(r#"INSERT INTO gestures(id_gesture, tags) VALUES ('ce27c124-e47b-490f-b8fe-3f37d5dbbef6', '{"tag1", "tag2"}')"#, &[]).unwrap();
        client.execute(r#"INSERT INTO gestures(id_gesture, tags) VALUES ('16991982-1752-4aa0-bb22-db3fbceb3780', '{"tag1", "tag2"}')"#, &[]).unwrap();

        client.execute(r#"INSERT INTO descriptions(
                id_description, id_gesture, val, langs)
                VALUES ('2ae70884-97bd-401d-8f43-d1778d4502d2', 'ce27c124-e47b-490f-b8fe-3f37d5dbbef6', 'Une petite description', '{"fr", "us"}');"#, &[]).unwrap();
        client.execute(r#"INSERT INTO descriptions(
                id_description, id_gesture, val, langs)
                VALUES ('1c53f9ad-98b4-444c-9ec9-e8f92f1e5d28', 'ce27c124-e47b-490f-b8fe-3f37d5dbbef6', 'Une petite description', '{"fr", "us"}');"#, &[]).unwrap();
        client.execute(r#"INSERT INTO descriptions(
                id_description, id_gesture, val, langs)
                VALUES ('cdbcd8fb-3d6d-4f09-86ba-37a6ec1dd293', '16991982-1752-4aa0-bb22-db3fbceb3780', 'Une petite description', '{"fr", "us"}');"#, &[]).unwrap();

        client.execute(r#"INSERT INTO meanings(
                id_meaning, id_description, id_gesture, val, langs)
                VALUES ('59c25147-021e-4584-9c35-97cbf060cc89', null, 'ce27c124-e47b-490f-b8fe-3f37d5dbbef6', 'Un petit meaning', '{"fr", "us"}');"#, &[]).unwrap();
        client.execute(r#"INSERT INTO meanings(
                id_meaning, id_description, id_gesture, val, langs)
                VALUES ('02ca8fb9-c56e-4e45-b13e-98a6732f780a', null, 'ce27c124-e47b-490f-b8fe-3f37d5dbbef6', 'Un petit meaning', '{"fr", "us"}');"#, &[]).unwrap();
        client.execute(r#"INSERT INTO meanings(
                id_meaning, id_description, id_gesture, val, langs)
                VALUES ('4719b1d7-2810-4f7d-865d-03ee44cf0add', null, '16991982-1752-4aa0-bb22-db3fbceb3780', 'Un petit meaning', '{"fr", "us"}');"#, &[]).unwrap();
        client.execute(r#"INSERT INTO meanings(
                id_meaning, id_description, id_gesture, val, langs)
                VALUES ('e2c6eee0-49a7-49c4-9a0f-a9c6e6f668d8', '2ae70884-97bd-401d-8f43-d1778d4502d2', null, 'Un petit meaning', '{"fr", "us"}');"#, &[]).unwrap();
        client.execute(r#"INSERT INTO meanings(
                id_meaning, id_description, id_gesture, val, langs)
                VALUES ('45dca590-6bc4-4e4b-ad0c-0fe57a3a9643', '2ae70884-97bd-401d-8f43-d1778d4502d2', null, 'Un petit meaning', '{"fr", "us"}');"#, &[]).unwrap();

        client.execute(r#"INSERT INTO pictures(
                id_picture, id_gesture, langs, format)
                VALUES ('283e7b04-7c13-4154-aafe-8e55b6960fe3', 'ce27c124-e47b-490f-b8fe-3f37d5dbbef6', '{"fr", "us"}', 'png');"#, &[]).unwrap();
        client.execute(r#"INSERT INTO pictures(
                id_picture, id_gesture, langs, format)
                VALUES ('03b9bfc6-fa22-4ffb-9464-93c1be842ace', 'ce27c124-e47b-490f-b8fe-3f37d5dbbef6', '{"fr", "us"}', 'png');"#, &[]).unwrap();
        client.execute(r#"INSERT INTO pictures(
                id_picture, id_gesture, langs, format)
                VALUES ('6e1ee88d-fd97-488c-9aa8-6b66a3f3e714', '16991982-1752-4aa0-bb22-db3fbceb3780', '{"fr", "us"}', 'png');"#, &[]).unwrap();
    }

    pub fn reset_db() {
        use std::fs;
        let schema = fs::read_to_string("schema.sql").expect("Schema.sql not found!");
        let schema: &str = &schema;
        let mut client = connect();

        client.batch_execute(schema).unwrap();
    }

    pub fn connect() -> postgres::Client {
        let (host, port, dbname, user, password) = db_conf();

        postgres::Client::connect(
            &format!(
                "host={} port={} user={} password={} dbname={}",
                host, port, user, password, dbname
            ),
            NoTls,
        )
        .unwrap()
    }

    pub fn db_conf() -> (String, String, String, String, String) {
        dotenv::dotenv().ok();
        (
            var("PG_HOST").unwrap(),
            var("PG_PORT").unwrap(),
            var("PG_DB_NAME").unwrap(),
            var("PG_USER").unwrap(),
            var("PG_PWD").unwrap(),
        )
    }
}

#[allow(dead_code)]
pub mod check {

    pub fn select_picture() -> postgres::Row {
        let mut client = super::setup::connect();
        client.query_one("SELECT * FROM pictures", &[]).unwrap()
    }
}
