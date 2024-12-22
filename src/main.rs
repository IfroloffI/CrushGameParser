use thirtyfour::prelude::*;
use thirtyfour::{WebDriver};
use serde_json;
use tokio::time::{sleep, Duration};
use sqlx::{PgPool, Error};

async fn dlya_slona(curr_value: String, bets_total: String, pool: &PgPool) -> Result<(), Error> {
    let query = "INSERT INTO game_data (curr_value, bets_total) VALUES ($1, $2)";
    
    sqlx::query(query)
        .bind(curr_value)
        .bind(bets_total)
        .execute(pool)
        .await?;

    Ok(())
}


#[tokio::main]
async fn main() -> WebDriverResult<()> {
    let localhost = "61823";
    let caps = DesiredCapabilities::chrome();
    let driver = WebDriver::new(&format!("http://localhost:{}", localhost), caps).await?;

    let database_url = "postgres://postgres:123456@localhost/test_analysis_001";
    let pool = PgPool::connect(database_url).await.map_err(|e| {
        WebDriverError::from(std::io::Error::new(std::io::ErrorKind::Other, format!("Ошибка подключения к базе данных: {}", e)))
    })?;

    driver.goto("https://1play.gamedev-tech.cc/lucky/onewin/?exitUrl=https%253A%252F%252F1wwzta.top%252Fcasino&language=ru&b=demo").await?;

    let mut previous_value = String::new();
    let mut bets_total;
    let mut bets_total_div_span;
    //let mut players_online_value_span;
    //let mut players_online;

    loop {
        let curr_div = driver.query(By::Id("history-item-0")).first().await?;
        let curr_value = curr_div.text().await?;
    
        if curr_value != previous_value {
            bets_total_div_span = driver.query(By::Id("bets-total")).first().await?;
            bets_total = bets_total_div_span.text().await?;
            //players_online_value_span = driver.query(By::Id("players-online-value")).first().await?;
            //players_online = players_online_value_span.text().await?;
            //class="sc-ecPEgm jhxPcI" bets
            //let bet =driver.find_all(By::Id("all-bets-item-price")).await?;
            //for elem in bet{
            //    println!("{}", elem.text().await?);
            //}
            //println!("Онлайн челиков: {:?}", players_online);

            sleep(Duration::from_micros(200)).await;

            if let Err(e) = dlya_slona(curr_value.clone(), bets_total.clone(), &pool).await {
                eprintln!("Ошибка при записи в БД: {:?}", e);
            }
            
            previous_value = curr_value;
        }
    
        sleep(Duration::from_micros(200)).await;
    }
    driver.quit().await?;

    Ok(())
}
