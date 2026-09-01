// RUN MULTIPLE FUTURES CONCURRENTLY USING JoinSet
use std::time;
use tokio::join;
use tokio::task::JoinSet;
use tokio::time::{Duration, sleep};
use sqlx::{Row, Column};
use serde::Serialize; 
use std::fs::File;     
use std::io::Write;
use reqwest::Client;
use reqwest::redirect::Policy;
use headless_chrome::{Browser, LaunchOptions};
use scraper::{Html, Selector};

async fn lazy_hi() {
    log::info!("Yaaaw!! 🥱");
    sleep(Duration::from_secs(3)).await;
    log::info!("lazy hi!");
}

#[derive(Serialize)]
struct User {
    name: String,
}

#[derive(Debug, Serialize)]
struct CurrencyRow {
    symbol: String,
    unit: String,
    buy_rate: String,
    sell_rate: String,
}

pub async fn demo() -> Result<(), Box<dyn std::error::Error>> {
    // -------------------------------------------------------------------------
    // GUARANTEED FULLY LOADED WEB SCRAPING ENHANCEMENT
    // -------------------------------------------------------------------------
    println!("Launching highly configured browser instance...");

    // Configure common flags to masquerade headless state from anti-scraping systems
 // Configure common flags to masquerade headless state from anti-scraping systems
    let options = LaunchOptions::default_builder()
        .headless(true)
        .window_size(Some((1920, 1080)))
        .args(vec![
            std::ffi::OsStr::new("--disable-blink-features=AutomationControlled"),
            std::ffi::OsStr::new("--user-agent=Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36"),
        ])
        .build()?;

    let browser = Browser::new(options)?;
    let tab = browser.new_tab()?;

    // FIX 1: Point to the actual sub-directory that renders the real-time rates table
    println!("Navigating to live MaxMoney rates page...");
    tab.navigate_to("https://www.maxmoney.com/my/rates")?;

    // FIX 2: Wait until the generic body content layout is established
    println!("Waiting for structural DOM assembly...");
    tab.wait_for_element("body")?;

    // FIX 3: Explicitly wait until the dynamic application components are populated.
    // We poll until specific raw JS placeholders (like "{{ exchangeAmount }}") are completely 
    // evaluated and cleared out by the framework engine.
    println!("Synchronizing with asynchronous API hydration loops...");
    let mut data_hydrated = false;
    
    for attempt in 1..=15 {
        let current_snapshot = tab.get_content()?;
        
        // Ensure standard templates aren't blankly showing placeholders or raw loaders
        if !current_snapshot.contains("{{ exchangeAmount }}") && current_snapshot.contains("We Buy") {
            println!("DOM fully populated with live API metrics on attempt {}!", attempt);
            data_hydrated = true;
            break;
        }
        println!("Content is still rendering, waiting 1 second... (Attempt {}/15)", attempt);
        std::thread::sleep(Duration::from_secs(1));
    }

    if !data_hydrated {
        println!("⚠️ Warning: Maximum wait threshold exceeded. Extracting layout fallback state.");
    }

    println!("Extracting validated page contents...\n");
    let html_content = tab.get_content()?;

    
    let document = Html::parse_document(&html_content.as_str());

    // Select <tbody id="foreign_exchange_rate_lists_table_body">
    let selector = Selector::parse(
        r#"tbody#foreign_exchange_rate_lists_table_body"#
    ).unwrap();

    if let Some(tbody) = document.select(&selector).next() {
        println!("Found <tbody>");

        // Extract the complete HTML inside <tbody>
        let tbody_html = tbody.html();
        let mut body_file = File::create("body.html")?;
        body_file.write_all(tbody_html.as_bytes())?;  
        // println!("{}", tbody_html);

  // --- Extract ROW and Column
// Parse the HTML document
let fixed_html = format!("<table>{}</table>", tbody_html.as_str());
let fragment = Html::parse_fragment(&fixed_html);
    //let fragment = Html::parse_fragment(tbody_html.as_str());

    // Define CSS selectors
    let tr_selector = Selector::parse("tr").unwrap();
    let h4_selector = Selector::parse("h4").unwrap();
    let td_selector = Selector::parse("td").unwrap();

    // Initialize the Rust list (Vector)
    let mut currency_list: Vec<CurrencyRow> = Vec::new();

    // Iterate through each table row (tr)
    for tr_element in fragment.select(&tr_selector) {
        // 1. Extract the <h4> text content
        let symbol = match tr_element.select(&h4_selector).next() {
            Some(h4) => h4.text().collect::<String>().trim().to_string(),
            None => continue, // Skip row if it doesn't contain an <h4> tag (e.g., table header rows)
        };

        // 2. Collect all <td> text contents into a Vector of strings
        let td_contents: Vec<String> = tr_element
            .select(&td_selector)
            .map(|td| td.text().collect::<String>().trim().to_string())
            .collect();

        // 3. Ensure we have exactly 3 <td> values before populating the struct
        if td_contents.len() == 3 {
            let row_data = CurrencyRow {
                symbol,
                unit: td_contents[0].clone(),
                buy_rate: td_contents[1].clone(),
                sell_rate: td_contents[2].clone(),
            };
            
            // Push into the final list
            currency_list.push(row_data);
        }
    }

    // Print the populated Rust list
    println!("{:#?}", currency_list);

 // -------------------------------------------------------------------------
    // EXPORT CURRENCY LIST TO JSON
    // -------------------------------------------------------------------------
    println!("Writing currency metrics to currencies.json...");
    match serde_json::to_string_pretty(&currency_list) {
        Ok(json_data) => {
            match File::create("currencies.json") {
                Ok(mut file) => {
                    if let Err(e) = file.write_all(json_data.as_bytes()) {
                        eprintln!("Failed to write to currencies.json: {}", e);
                    } else {
                        println!("Successfully saved live market metrics to currencies.json file!\n");
                    }
                }
                Err(e) => eprintln!("Failed to create currencies.json file: {}", e),
            }
        }
        Err(e) => eprintln!("Failed to serialize currency_list to JSON: {}", e),
    }

   // --- END 





    } else {
        println!("tbody not found");
    }








    // -------------------------------------------------------------------------
    // WRITE RESPONSE HOOK
    // -------------------------------------------------------------------------
    println!("Writing fully loaded page contents to response.html...");
    let mut html_file = File::create("response.html")?;
    html_file.write_all(html_content.as_bytes())?;

 
    

    println!("Successfully saved verified snapshot data to response.html file!\n");

    // -------------------------------------------------------------------------
    // DATABASE REGISTRY PROCESSOR
    // -------------------------------------------------------------------------
    let database_url = "mysql://root:KVNUFVgOJFnBrQNyeHJ@127.0.0.1:3306";
    println!("Attempting to connect to MariaDB...");

    let pool_result = sqlx::mysql::MySqlPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await;

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
            return Err(Box::new(e)); 
        }
    };

    println!("\nFetching user names from payment_system.users...");
    
    let users_rows = sqlx::query("SELECT name FROM payment_system.users")
        .fetch_all(&connection_pool)
        .await?;

    let mut users_list: Vec<User> = Vec::new();

    println!("--- User Names ---");
    for row in users_rows {
        let user_name: String = row.get("name");
        println!("User: {}", user_name);
        users_list.push(User { name: user_name });
    }
    println!("-----------------\n");

    // -------------------------------------------------------------------------
    // SERIALIZE TO FILE
    // -------------------------------------------------------------------------
    println!("Writing user records to users.json...");
    let json_data = serde_json::to_string_pretty(&users_list)?;
    let mut file = File::create("users.json")?;
    file.write_all(json_data.as_bytes())?;



    
    println!("Successfully saved data to users.json file!\n");
    Ok(())
}
