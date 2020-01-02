use actix_web::{middleware, web, App, HttpRequest, HttpServer};

async fn index(req: HttpRequest) -> &'static str {
    println!("REQ: {:?}", req);
    "Hello world!"
}

// #[actix_rt::main]
// async fn main() -> std::io::Result<()> {
//     std::env::set_var("RUST_LOG", "actix_web=info");
//     env_logger::init();

//     HttpServer::new(|| {
//         App::new()
//             .wrap(middleware::Logger::default())
//             // .service(web::resource("/index.html").to(|| async { "Hello world!" }))
//             .service(web::resource("/").to(index))
//     })
//     .bind("127.0.0.1:8080")?
//     .run()
//     .await
// }

use my::params;
use mysql as my;

#[derive(Debug, PartialEq, Eq)]
struct Payment {
    customer_id: i32,
    amount: i32,
    account_name: Option<String>,
}

fn main() {
    // See docs on the `OptsBuilder`'s methods for the list of options available via URL.
    let mut builder = my::OptsBuilder::new();
    builder
        .db_name(Some("temp"))
        .user(Some("user"))
        .pass(Some("password"));
    let pool = my::Pool::new(builder).unwrap();
    println!("1");

    // Let's create payment table.
    // Unwrap just to make sure no error happened.

    pool.prep_exec(r"DROP TABLE IF EXISTS payment", ()).unwrap();
    pool.prep_exec(
        r"CREATE TABLE payment (
                         customer_id int not null,
                         amount int not null,
                         account_name text
                     )",
        (),
    )
    .unwrap();

    let payments = vec![
        Payment {
            customer_id: 1,
            amount: 2,
            account_name: None,
        },
        Payment {
            customer_id: 3,
            amount: 4,
            account_name: Some("foo".into()),
        },
        Payment {
            customer_id: 5,
            amount: 6,
            account_name: None,
        },
        Payment {
            customer_id: 7,
            amount: 8,
            account_name: None,
        },
        Payment {
            customer_id: 9,
            amount: 10,
            account_name: Some("bar".into()),
        },
    ];
    println!("2");

    // Let's insert payments to the database
    // We will use into_iter() because we do not need to map Stmt to anything else.
    // Also we assume that no error happened in `prepare`.
    for mut stmt in pool
        .prepare(
            r"INSERT INTO payment
                                       (customer_id, amount, account_name)
                                   VALUES
                                       (:customer_id, :amount, :account_name)",
        )
        .into_iter()
    {
        for p in payments.iter() {
            // `execute` takes ownership of `params` so we pass account name by reference.
            // Unwrap each result just to make sure no errors happened.
            stmt.execute(params! {
                "customer_id" => p.customer_id,
                "amount" => p.amount,
                "account_name" => &p.account_name,
            })
            .unwrap();
        }
    }
    println!("3");

    // Let's select payments from database
    let selected_payments: Vec<Payment> = pool
        .prep_exec("SELECT customer_id, amount, account_name from payment", ())
        .map(|result| {
            // In this closure we will map `QueryResult` to `Vec<Payment>`
            // `QueryResult` is iterator over `MyResult<row, err>` so first call to `map`
            // will map each `MyResult` to contained `row` (no proper error handling)
            // and second call to `map` will map each `row` to `Payment`
            result
                .map(|x| x.unwrap())
                .map(|row| {
                    // ⚠️ Note that from_row will panic if you don't follow your schema
                    let (customer_id, amount, account_name) = my::from_row(row);
                    Payment {
                        customer_id: customer_id,
                        amount: amount,
                        account_name: account_name,
                    }
                })
                .collect() // Collect payments so now `QueryResult` is mapped to `Vec<Payment>`
        })
        .unwrap(); // Unwrap `Vec<Payment>`

    println!("4");
    // Now make sure that `payments` equals to `selected_payments`.
    // Mysql gives no guaranties on order of returned rows without `ORDER BY`
    // so assume we are lukky.
    assert_eq!(payments, selected_payments);

    pool.prep_exec(r"DROP TABLE payment", ()).unwrap();
    println!("Table dropped");
    println!("Yay!");
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::dev::Service;
    use actix_web::{http, test, web, App, Error};

    #[actix_rt::test]
    async fn test_index() -> Result<(), Error> {
        let app = App::new().route("/", web::get().to(index));
        let mut app = test::init_service(app).await;

        let req = test::TestRequest::get().uri("/").to_request();
        let resp = app.call(req).await.unwrap();

        assert_eq!(resp.status(), http::StatusCode::OK);

        let response_body = match resp.response().body().as_ref() {
            Some(actix_web::body::Body::Bytes(bytes)) => bytes,
            _ => panic!("Response error"),
        };

        assert_eq!(response_body, r##"Hello world!"##);

        Ok(())
    }
}
