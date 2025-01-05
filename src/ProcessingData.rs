use std::time::Duration;
use thirtyfour::error::WebDriverResult;
use thirtyfour::{By, WebDriver};
use thirtyfour::prelude::ElementQueryable;
use tokio::time::{sleep, Instant};
use plotters::prelude::*;

pub async fn run_loop(driver: WebDriver) -> WebDriverResult<Vec<f64>> {
    let start_time = Instant::now(); // Засекаем время начала
    let timeout = Duration::from_secs(600); // 5 минут

    let mut value = String::new();
    let mut values: Vec<f64> = Vec::new();

    loop {
        if start_time.elapsed() >= timeout {
            println!("Время вышло. Выход из цикла.");
            break;
        }

        let curr_div = driver.query(By::Id("history-item-0")).first().await?;
        let curr_value = curr_div.text().await?;

        if curr_value == value {
            sleep(Duration::from_secs(8)).await;
        } else {
            value = curr_value.clone();

            if let Some(ind) = value.find("x") {
                let without_x = &value[..ind];
                let int_value: f64 = without_x.parse().unwrap();
                values.push(int_value);
                println!("{}", int_value);
            } else {
                println!("Error");
            }
        }

        sleep(Duration::from_micros(200)).await;
    }

    Ok(values)
}

pub fn find_mediana_average(values: &Vec<f64>) ->(f64, f64) {
    let mut sort_values =values.clone();
    sort_values.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));

    let mut mediana:f64=0.0;
    if sort_values.len() %2==1 {
        mediana=sort_values[sort_values.len() / 2-1];
    }
    else{
        mediana=(sort_values[sort_values.len() / 2-1]+sort_values[sort_values.len() / 2])/2.0;
    }
    let mut average:f64=0.0;
    let mut sum:f64=0.0;
    for i in 0..values.len(){
        sum+=values[i];
    };
    average=sum/(values.len()as f64);
    (average,mediana)
}

pub fn find_low(values: &Vec<f64>) ->(i64, Vec<i64>) {
    let mut straight_data:Vec<i64> = Vec::new();
    let mut sort_values =values.clone();
    sort_values.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    let mut low: i64=0;
    let mut straight:i64=1;
    let mut check=true;
    for i in 0..sort_values.len() {
        if sort_values[i] < 1.2 && check{
            low += 1;
            straight+=1;
        }
        else if sort_values[i] < 1.2{
            low+=1;
            check=true;
        }
        else{
            check=false;
            straight_data.push(straight);
            straight=0;
        }

    }
    (low,straight_data)
}

pub fn graph(values: &Vec<f64>) ->Result<(), Box<dyn std::error::Error>> {
    let y_data: Vec<f64>=values.clone();
    let len=y_data.len() as i32;
    let x_data: Vec<i32> = (0..len).collect();
    let x_data_f64: Vec<f64> = x_data.iter().map(|y| *y as f64).collect();

    // Создаем область для рисования
    let root = BitMapBackend::new("plot.png", (640, 480)).into_drawing_area();
    root.fill(&WHITE)?;

    // Создаем график
    let mut chart = ChartBuilder::on(&root)
        .caption("-------", ("sans-serif", 50).into_font())
        .margin(5)
        .x_label_area_size(50)
        .y_label_area_size(50)
        .build_cartesian_2d(0.0..50.0, 0.0..200.0)?; // Диапазоны осей X и Y

    // Настраиваем сетку
    chart.configure_mesh().draw()?;

    // Рисуем график
    chart.draw_series(LineSeries::new(
        x_data_f64.iter().zip(y_data.iter()).map(|(x, y)| (*x, *y)),
        &RED,
    ))?;

    // Добавляем точки на график
    chart.draw_series(
        x_data_f64.iter().zip(y_data.iter()).map(|(x, y)| {
            Circle::new((*x, *y), 5, RED.filled())
        }),
    )?;

    Ok(())
}