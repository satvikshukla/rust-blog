use my::Value;
use mysql as my;

#[derive(Debug, PartialEq, Eq)]
struct User {
    username: String,
    password: String,
}

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct Post {
    username: String,
    id: i32,
    body: String,
    timestamp: String,
}

fn create_pool() -> my::Pool {
    let mut builder = my::OptsBuilder::new();
    builder
        .db_name(Some("temp"))
        .user(Some("user"))
        .pass(Some("password"));
    let pool = my::Pool::new(builder).unwrap();
    return pool;
}

pub fn create_tables() {
    clear_tables();
    let pool = create_pool();

    pool.prep_exec(
        r"CREATE TABLE Users (
                         username VARCHAR(20) PRIMARY KEY,
                         password TEXT NOT NULL)",
        (),
    )
    .unwrap();
    // Should probably create index on username
    pool.prep_exec(
        r"CREATE TABLE Posts (
                         username VARCHAR(20),
                         id INT AUTO_INCREMENT PRIMARY KEY,
                         body TEXT ,
                         timestamp DATETIME,
                     FOREIGN KEY (username)
                     REFERENCES Users(username))",
        (),
    )
    .unwrap();

    // pool.prep_exec(r"CREATE TABLE Feed (
    //                      user VARCHAR(20),
    //                      following VARCHAR(20))", ()).unwrap();
}

fn clear_tables() {
    let pool = create_pool();
    pool.prep_exec(r"DROP TABLE IF EXISTS Posts", ()).unwrap();
    // pool.prep_exec(r"DROP TABLE IF EXISTS Feed", ()).unwrap();
    pool.prep_exec(r"DROP TABLE IF EXISTS Users", ()).unwrap();
}

pub fn create_user(username: &str, password: &str) {
    let pool = create_pool();
    pool.prep_exec(
        "INSERT INTO Users
         (username, password)
         VALUES
         (?, ?)",
        (&username, &password),
    )
    .unwrap();

    // pool.prep_exec(
    //     "INSERT INTO Feed
    //      (user, following)
    //      VALUES
    //      (?, ?)",
    //      (&username, &username)).unwrap();
}

pub fn user_exists(username: &str) -> bool {
    let pool = create_pool();
    let result = pool
        .prep_exec(
            "SELECT COUNT(*)
             FROM Users u
             WHERE u.username = ?",
            (&username,),
        )
        .unwrap();

    for row in result {
        let c = my::from_row::<i32>(row.unwrap());
        return c == 1;
    }
    false
}

pub fn check_credentials(username: &str) -> String {
    let pool = create_pool();
    let result = pool
        .prep_exec(
            "SELECT u.password
             FROM Users u
             WHERE u.username = ?",
            (&username,),
        )
        .unwrap();
    for row in result {
        let c = my::from_row::<String>(row.unwrap());
        return c;
    }
    return String::new();
}

pub fn create_post(username: &str, body: &str) {
    let pool = create_pool();
    pool.prep_exec(
        "INSERT INTO Posts
         (username, body, timestamp)
         VALUES
         (?, ?, CURRENT_TIMESTAMP)",
        (&username, &body),
    )
    .unwrap();
}

pub fn get_feed(username: &str) -> Vec<Post> {
    let pool = create_pool();
    // let result = pool.prep_exec(
    //     "SELECT *
    //      FROM Posts p, (SELECT following FROM Feed WHERE user = ?) f
    //      WHERE p.username = f.following
    //      ORDER BY p.timestamp DESC", (&username,)).unwrap();
    let feed: Vec<Post> = pool
        .prep_exec(
            "SELECT *
            FROM Posts p
            WHERE p.username = ?
            ORDER BY p.timestamp DESC",
            (&username,),
        )
        .map(|result| {
            result
                .map(|x| x.unwrap())
                .map(|row| {
                    let (username, id, body, timestamp) =
                        my::from_row::<(String, i32, String, Value)>(row);
                    Post {
                        username: username,
                        id: id,
                        body: body,
                        timestamp: timestamp.as_sql(true),
                    }
                })
                .collect()
        })
        .unwrap();
    feed
}

fn main() {
    setup();
    create_post("test", "this is the body of the post.");
    println!("{:?}", get_feed("test"));
    cleanup();
}

pub fn setup() {
    clear_tables();
    create_tables();
    let users = vec![User {
        username: String::from("test"),
        password: String::from("1234"),
    }];
    for u in users.iter() {
        create_user(&u.username, &u.password);
    }
}

pub fn cleanup() {
    clear_tables();
}

// Test using `cargo test -- --test-threads=1`
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn check_insert() {
        setup();
        let pool = create_pool();
        let selected_users: Vec<User> = pool
            .prep_exec("SELECT username, password FROM Users", ())
            .map(|result| {
                result
                    .map(|x| x.unwrap())
                    .map(|row| {
                        let (username, password) = my::from_row(row);
                        User {
                            username: username,
                            password: password,
                        }
                    })
                    .collect()
            })
            .unwrap();
        let users = vec![User {
            username: String::from("test"),
            password: String::from("1234"),
        }];
        assert_eq!(users, selected_users);
        cleanup();
    }

    #[test]
    fn check_exists() {
        setup();
        assert!(!user_exists("helloworld"));
        assert!(user_exists("test"));
        cleanup();
    }

    #[test]
    fn check_check_credentials() {
        setup();
        assert_eq!(check_credentials("tes2t"), "");
        assert_eq!(check_credentials("test"), "1234");
        cleanup();
    }

    #[test]
    fn check_create_post() {
        setup();
        create_post("test", "this is the body of the post.");
        cleanup();
    }
}
