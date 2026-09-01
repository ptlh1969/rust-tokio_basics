// RUN MULTIPLE FUTURES CONCURRENTLY USING JoinSet
use std::time;
use tokio::join;
use tokio::task::JoinSet;
use tokio::time::{Duration, sleep};
use sqlx::{Row, Column}; // Required for row data and column handling

async fn lazy_hi() {
    log::info!("Yaaaw!! 🥱");
    sleep(Duration::from_secs(3)).await;
    log::info!("lazy hi!");
}

pub async fn demo() -> Result<(), sqlx::Error> {
    let database_url = "mysql://root:KVNUFVgOJFnBrQNyeHJ@127.0.0.1:3306";

    println!("Attempting to connect to MariaDB...");

    // Create a connection pool targeting MariaDB/MySQL
    let pool_result = sqlx::mysql::MySqlPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await;

    // Extract the connection pool out of the block so we can use it for subsequent queries
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
            return Err(e); // Exit early if we cannot establish a connection
        }
    };

    // -------------------------------------------------------------------------
    // NEW IMPLEMENTATION: Query payment_system.users and loop to print name field
    // -------------------------------------------------------------------------
    println!("\nFetching user names from payment_system.users...");
    
    let users_rows = sqlx::query("SELECT name FROM payment_system.users")
        .fetch_all(&connection_pool)
        .await?;

    println!("--- User Names ---");
    for row in users_rows {
        // Extract the "name" column value as a String
        let user_name: String = row.get("name");
        println!("User: {}", user_name);
    }
    println!("-----------------\n");
    // -------------------------------------------------------------------------

    let mut numbers: Vec<i32> = Vec::new(); 
    
    // Create a vector with initial values using the vec! macro
    let mut fruits = vec!["apple", "banana", "orange", "grape", "cherry"]; 
    
    // Create a vector with a fixed size filled with a default value (e.g., five 0s)
    let _coordinates = vec![0; 5]; 

    // 2. Modifying a Vector (Requires 'mut')
    numbers.push(10); // Adds elements to the end
    numbers.push(20);
    numbers.push(30);
    
    let _last_num = numbers.pop(); // Removes and returns the last element (Some(30))

    // 3. Accessing Elements
    // Method A: Direct indexing (Panics if out of bounds!)
    let _first_fruit = fruits[0]; 
    
    // Method B: Using .get() (Safe, returns an Option type)
    match fruits.get(1) {
        Some(fruit) => println!("Second fruit is: {}", fruit),
        None => println!("No fruit at this index."),
    }

    // Updating a specific element via indexing
    fruits[0] = "avocado"; 

    // 4. Iterating over Elements
    // Iterate over immutable references (reading data)
    println!("Current fruits:");
    for fruit in &fruits {
        println!(" - {}", fruit);
    }

    // Iterate over mutable references (modifying data in-place)
    for num in &mut numbers {
        *num += 5; // Use dereference operator to alter the value
    }

    // 5. Utility Methods
    println!("Vector length: {}", numbers.len());     // Returns number of elements
    println!("Is numbers empty? {}", numbers.is_empty()); // R

    let start = time::Instant::now();
    let mut set = JoinSet::new();
    for _i in 1..=2 {
        let future = lazy_hi();
        set.spawn(future);
    }
    while let Some(result) = set.join_next().await {
        result.expect("task panicked");
    }

    let end = time::Instant::now();
    log::info!("Took {:?} secs", end - start);
    
    Ok(())
}
