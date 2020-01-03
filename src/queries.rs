use mysql as my;
use std::error::Error;

#[derive(Debug, PartialEq, Eq)]
struct User {
    username: String,
    password: String,
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
                         password TEXT not null
                     )",
        (),
    )
    .unwrap();
}

fn clear_tables() {
    let pool = create_pool();
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

fn main() {
    println!("Table dropped");
    println!("Yay!");
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
    let pool = create_pool();
    pool.prep_exec(r"DROP TABLE Users", ()).unwrap();
}

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
}
