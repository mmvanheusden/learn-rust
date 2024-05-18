use std::error::Error;
use std::ops::Add;
use std::time::{Duration, SystemTime};

use thirtyfour::prelude::*;

/// This is quick bullshit code and it doesn't work well. Might improve later
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let caps = DesiredCapabilities::firefox();
    let driver = WebDriver::new("http://localhost:4444", caps).await?;
    // Navigate to https://wikipedia.org.
    driver.goto("https://cpstest.org/2-seconds.php").await?;

    let end = SystemTime::now().add(Duration::from_secs(2));
    let click_area = driver.find(By::Id("clickarea")).await?;
    
    // Spam click for 2 seconds.
    // Because it waits for it to fully finish it is very slow. Maybe if we just spawn a shit ton of threads we can make it go super speed.
    while SystemTime::now() < end {
        println!("a");
        click_area.click().await?;
        println!("b");
    }

    // Always explicitly close the browser.'
    driver.quit().await?;

    Ok(())
}
