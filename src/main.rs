use serde::{Deserialize, Serialize};
use surrealdb::engine::remote::ws::Ws;
use surrealdb::opt::auth::Root;
use surrealdb::sql::Thing;
use surrealdb::Surreal;
use std::io::{self, BufRead};

#[derive(Debug, Serialize)]
struct Name<'a> {
    first: &'a str,
    last: &'a str,
}

#[derive(Debug, Serialize)]
struct Person<'a> {
    title: &'a str,
    name: Name<'a>,
    marketing: bool,
}

#[derive(Debug, Serialize)]
struct Responsibility {
    marketing: bool,
}

#[derive(Debug, Deserialize)]
struct Record {
    #[allow(dead_code)]
    id: Thing,
}

#[tokio::main]
async fn main() -> surrealdb::Result<()> {
    dbg!(std::env::args());
    let domain_and_port=std::env::args().nth(1).expect("Need the domain & Port... Aborting!");
    let username=std::env::args().nth(2).expect("Need the Username... Aborting!");
    let password=std::env::args().nth(3).expect("Need the Password... Aborting!");
    let namespace=std::env::args().nth(4).expect("Need the Namespace... Aborting!");
    let database=std::env::args().nth(5).expect("Need the Database... Aborting!");

    println!("Connecting to: {} {} {} {} {}", domain_and_port,username,password,namespace,database);

    // Connect to the server
    let db = Surreal::new::<Ws>(domain_and_port.trim()).await?;

    // Signin as a namespace, database, or root user
    db.signin(Root {
        username: username.trim(),
        password: password.trim(),
    })
    .await?;

    // Select a specific namespace / database
    db.use_ns(namespace.trim()).use_db(database.trim()).await?;

    let mut input_line = String::new();
    let stdin = io::stdin();
    let mut input_handler=stdin.lock();
    let exit_str=String::from("exit");
    while exit_str.ne(input_line.trim()){
        input_line = String::new();
        input_handler.read_line(&mut input_line);
        
        let mut result=db.query(input_line.trim()).await?;
        dbg!(result);
        
    }

    Ok(())
}