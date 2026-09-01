// RUN MULTIPLE FUTURES CONCURRENTLY USING JoinSet
use std::time;
use tokio::join;
use tokio::task::JoinSet;
use tokio::time::{Duration, sleep};
use sqlx::{Row, Column};
use serde::Serialize; // Required to convert Rust structures into JSON
use std::fs::File;     // Required to create and write files
use std::io::Write;

async fn lazy_hi() {
    log::info!("Yaaaw!! 🥱");
    sleep(Duration::from_secs(3)).await;
    log::info!("lazy hi!");
}

// Define a structured layout to map your target JSON format
#[derive(Serialize)]
struct User {
    name: String,
}

pub async fn demo() -> Result<(), Box<dyn std::error::Error>> {
    let database_url = "mysql://root:KVNUFVgOJFnBrQNyeHJ@127.0.0.1:3306";
    println!("Attempting to connect to MariaDB...");

    // Create a connection pool targeting MariaDB/MySQL
    let pool_result = sqlx::mysql::MySqlPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await;

    // Extract the connection pool out of the block so we can use it 
    // for subsequent queries
    let connection_pool = match pool_result {
        Ok(pool) => {
            println!("status connected");
            let row = sqlx::query("SELECT VERSION()")
                .fetch_one(&pool)
                .await?;
            
            let server_version: String = sqlx::Row::get(&row, 0);
            println!("Database Server Version: {}", server_version);
            pool
        }
        Err(e) => {
            println!("Status: connection failed");
            eprintln!("Error details: {}", e);
            return Err(Box::new(e)); // Return boxed error to support multiple error variants
        }
    };

    println!("\nFetching user names from payment_system.users...");
    
    let users_rows = sqlx::query("SELECT name FROM payment_system.users")
        .fetch_all(&connection_pool)
        .await?;

    // Create an empty vector to store our structured user records
    let mut users_list: Vec<User> = Vec::new();

    println!("--- User Names ---");
    for row in users_rows {
        let user_name: String = row.get("name");
        println!("User: {}", user_name);
        
        // Push instances into our container list
        users_list.push(User { name: user_name });
    }
    println!("-----------------\n");

    // -------------------------------------------------------------------------
    // NEW IMPLEMENTATION: Serialize vector to JSON and save to a file
    // -------------------------------------------------------------------------
    println!("Writing user records to users.json...");
    
    // Convert the vector of structs into a pretty-printed JSON string
    let json_data = serde_json::to_string_pretty(&users_list)?;

    // Open a file handle and dump the text contents into it
    let mut file = File::create("users.json")?;
    file.write_all(json_data.as_bytes())?;

    println!("Successfully saved data to users.json file!\n");
    Ok(())
}
