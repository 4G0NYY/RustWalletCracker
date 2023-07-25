use rand::Rng;
use reqwest::StatusCode;
use rusqlite::{params, Connection, Result as SqliteResult};
use std::io::{Write};
use std::thread;
use std::time::{Instant};
use std::fs::File;
use std::io::{self, BufRead, BufReader};

const NUM_THREADS: usize = 10;
const NUM_PRIVATE_KEYS: usize = 1000;

fn getrpc() {
    let rpc_file = File::open("api.txt").expect("Failed to open the file");
    let rpc_reader = BufReader::new(rpc_file);

    let mut rpc = vec![];

    for line in rpc_reader.lines() {
        if let Ok(value) = line {
            if let Ok(num) = value.trim().parse::<u32>() {
                rpc.push(num);
            }
        }
    }
}

fn miner(_api: &str) {
    let rpc = vec![ 

    ];

    let mut rng = rand::thread_rng();
//    let mut db_conn = init_database().expect("Failed to initialize database.");

    for _ in 0..NUM_PRIVATE_KEYS {
        let private_key = generate_ethereum_private_key(&mut rng);
        let has_transactions = check_transactions(&rpc, &private_key);

        if has_transactions {
            write_to_file("hits.txt", &private_key);
        } else {
//            insert_to_database(&mut db_conn, &private_key);
        }
    }
}

fn generate_ethereum_private_key(rng: &mut impl Rng) -> String {
    let mut private_key = String::new();
    for _ in 0..64 {
        private_key.push(rng.gen_range('0'..='9'));
    }
    private_key
}

fn check_transactions(rpc_list: &[&str], private_key: &str) -> bool {
    for rpc_url in rpc_list {
        if let Ok(response) = reqwest::blocking::get(&format!(
            "{}/api/v1/accounts/{}",
            rpc_url, private_key
        )) {
            if response.status() == StatusCode::OK {
                let response_json: serde_json::Value = response.json().expect("JSON parsing failed.");
                if let Some(transactions) = response_json.get("transactions") {
                    if transactions.as_array().unwrap().len() > 0 {
                        return true;
                    }
                }
            }
        }
    }
    false
}

fn write_to_file(file_path: &str, private_key: &str) {
    let mut file = File::create(file_path).expect("Failed to create file.");
    writeln!(file, "{}", private_key).expect("Failed to write to file.");
}

fn init_database() -> SqliteResult<Connection> {
    let conn = Connection::open("ethereum_keys.db")?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS keys (
                  id INTEGER PRIMARY KEY,
                  private_key TEXT UNIQUE NOT NULL
                  )",
        params![],
    )?;
    Ok(conn)
//    let conn = Connection::close("ethereum_keys.db")?;
}

fn insert_to_database(conn: &mut Connection, private_key: &str) {
    conn.execute(
        "INSERT INTO keys (private_key) VALUES (?1)",
        params![private_key],
    )
    .expect("Failed to insert into the database.");
}

fn main() {
    println!("How many threads do you want to use (1 to 100)?");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");

    let num_threads: u32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input. Please enter a number between 1 and 100.");
            return;
        }
    };

    if num_threads < 1 || num_threads > 100 {
        println!("Invalid input. Please enter a number between 1 and 100.");
        return;
    }

    let _iteration = 0;
    let _start_time = Instant::now();

    let apis_file = File::open("api.txt").expect("Failed to open the file");
    let apis_reader = BufReader::new(apis_file);

    let mut apis = Vec::new();

    for line in apis_reader.lines() {
        if let Ok(api) = line {
            apis.push(api);
        }
    }

    if apis.is_empty() {
        println!("No APIs found in the file.");
        return;
    }

    let mut iteration = 0;
    let start_time = Instant::now();
    let num_apis = apis.len();
    let mut api_index = 0;

    loop {
        let current_api = &apis[api_index];

        let mut handles = vec![];

        for _ in 0..num_threads {
            let current_api_clone = current_api.to_string();
            let handle = thread::spawn(move || miner(&current_api_clone));
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        api_index = (api_index + 1) % num_apis;
        iteration += 1;
        let elapsed_time = start_time.elapsed().as_secs_f64();
        let iterations_per_second = iteration as f64 / elapsed_time;

        print!(
            "\rChecks: {}, API: {}, Checks per second: {:.2}",
            iteration, current_api, iterations_per_second
        );

        io::stdout().flush().unwrap();
    }
}