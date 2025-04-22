use headless_chrome::Browser;
use reqwest::blocking::Client;
use serde::Serialize;
use std::{
    error::Error,
    fs::{read_to_string, write},
    thread::sleep,
    time::Duration,
};

const PATH: &str = "./old.txt";
const WEBHOOK: &str = "WEBHOOK";

#[derive(Serialize)]
struct Body {
    content: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    loop {
        let browser = Browser::default()?;
        let tab = browser.new_tab()?;
        tab.navigate_to("https://www.gmodstore.com/jobmarket/jobs/browse")?;

        let name = tab.wait_for_xpath(
            "/html/body/div[1]/main/div/div[2]/div[2]/div/div[3]/div[1]/div/div[1]",
        )?;

        let url =
            tab.wait_for_xpath("/html/body/div[1]/main/div/div[2]/div[2]/div/div[3]/div[1]/div/a")?;

        let old_name = read_to_string(PATH)?;
        let new_name = name.get_inner_text()?;
        if old_name != new_name {
            Client::new()
                .post(WEBHOOK)
                .json(&Body {
                    content: format!(
                        "@everyone [**{new_name}**]({})",
                        url.get_attribute_value("href")?.unwrap()
                    ),
                })
                .send()?;

            write(PATH, new_name)?;
        }

        sleep(Duration::from_secs(60));
    }
}
