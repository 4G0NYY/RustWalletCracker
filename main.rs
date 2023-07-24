use rand::Rng;
use reqwest::StatusCode;
use rusqlite::{params, Connection, Result as SqliteResult};
use std::fs::File;
use std::io::{self, Write};

const NUM_PRIVATE_KEYS: usize = 10;

fn main() {
    let rpc = vec![ 
        "https://blue-omniscient-liquid.discover.quiknode.pro/0ddba96281a6f3f0c499c936a2fbf6d854a2afc7/",
        "https://ancient-black-lambo.discover.quiknode.pro/70bd95a582d1ee953518d8e12fa98453919220ca/",
        "https://wandering-omniscient-mansion.discover.quiknode.pro/401aaf72015fda6d1645c0ac6a2353a28d1363d0/"
    ];

    let mut rng = rand::thread_rng();
    let mut db_conn = init_database().expect("Failed to initialize database.");

    for _ in 0..NUM_PRIVATE_KEYS {
        let private_key = generate_ethereum_private_key(&mut rng);
        let has_transactions = check_transactions(&rpc, &private_key);

        if has_transactions {
            write_to_file("hits.txt", &private_key);
        } else {
            insert_to_database(&mut db_conn, &private_key);
        }
    }
}

fn generate_ethereum_private_key<R: Rng>(rng: &mut R) -> String {
    let private_key: String = (0..64)
        .map(|_| rng.gen_range(0..=15))
        .map(|n| format!("{:x}", n))
        .collect();
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
}

fn insert_to_database(conn: &mut Connection, private_key: &str) {
    conn.execute(
        "INSERT INTO keys (private_key) VALUES (?1)",
        params![private_key],
    )
    .expect("Failed to insert into the database.");
}
