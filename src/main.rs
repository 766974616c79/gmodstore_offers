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

        let url_element =
            tab.wait_for_xpath("/html/body/div[1]/main/div/div[2]/div[2]/div/div[3]/div[1]/div/a")?;

        let url = url_element.get_attribute_value("href")?.unwrap();
        let old_url = read_to_string(PATH)?;
        if old_url != url {
            let name_element = tab.wait_for_xpath(
                "/html/body/div[1]/main/div/div[2]/div[2]/div/div[3]/div[1]/div/div[1]",
            )?;

            let name = name_element.get_inner_text()?;
            Client::new()
                .post(WEBHOOK)
                .json(&Body {
                    content: format!("@everyone [**{name}**]({})", url),
                })
                .send()?;

            write(PATH, url)?;
        }

        sleep(Duration::from_secs(60));
    }
}
