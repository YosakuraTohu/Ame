use ame_models::msg_rev::{fetch_latest_tiu, TIU};
use colored::*;
use futures_util::StreamExt;
use sha256::digest;
use sqlx::postgres::PgPoolOptions;
use tokio::{io::AsyncWriteExt, time};

static BASE_PATH: &str = "cache/";
static STATUS_FILE: &str = "cache/cache_status";
static REQ_LENGTH: i32 = 20;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut interval = time::interval(time::Duration::from_secs(60));
    let pool = PgPoolOptions::new()
        .max_connections(3)
        .connect("postgres://localhost/Services")
        .await
        .unwrap();

    tokio::fs::create_dir_all(BASE_PATH).await?;

    loop {
        interval.tick().await;
        task(&pool).await?;
    }
}

async fn task(pool: &sqlx::PgPool) -> Result<(), Box<dyn std::error::Error>> {
    let mut offset = get_last_status().await;
    let mut len = REQ_LENGTH;

    while len == REQ_LENGTH {
        let rows = fetch_latest_tiu(pool, REQ_LENGTH, offset).await?;
        len = rows.len() as i32;
        offset += len;
        store_cache(rows).await?;
    }

    write_status(offset).await?;

    Ok(())
}

async fn store_cache(r: Vec<TIU>) -> Result<(), Box<dyn std::error::Error>> {
    let list: Vec<(String, Vec<String>)> = r
        .into_iter()
        .map(|u| (cache_file_name(u.time.timestamp(), u.mid), u.urls))
        .collect();

    for (bfname, urls) in list {
        for (idx, url) in urls.iter().enumerate() {
            let fname = bfname.clone() + "_" + &idx.to_string();
            match download_file(fname.clone(), url.clone()).await {
                Ok(_) => println!("GET [{}] {}", fname.clone(), "OK".green()),
                Err(_) => println!("GET [{}] {}", fname.clone(), "ERR".red()),
            };
        }
    }
    Ok(())
}

async fn download_file(fname: String, url: String) -> Result<(), Box<dyn std::error::Error>> {
    let mut stream = reqwest::ClientBuilder::new()
        .user_agent("Mozilla/5.0 (compatible; MSIE 10.0; Windows NT 6.2)")
        .build()?
        .get(url)
        .send()
        .await?
        .bytes_stream();
    let mut out = tokio::fs::File::create(BASE_PATH.to_string() + &fname).await?;

    while let Some(item) = stream.next().await {
        out.write_all(&item?).await?;
    }

    Ok(())
}

fn cache_file_name(t: i64, i: i32) -> String {
    let input = t.to_string() + &i.to_string();
    digest(input)
}

async fn get_last_status() -> i32 {
    match tokio::fs::read_to_string(STATUS_FILE).await {
        Ok(s) => s.parse().unwrap_or_default(),
        Err(_) => 0,
    }
}

async fn write_status(s: i32) -> tokio::io::Result<()> {
    tokio::fs::write(STATUS_FILE, s.to_string()).await?;
    Ok(())
}
