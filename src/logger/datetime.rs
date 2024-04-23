use chrono::{DateTime, Local};

pub struct Datetime;

impl Datetime {

    pub(crate) fn local(&self) -> DateTime<Local> {
        Local::now()
    } 

    pub fn date(&self) -> String {
        let local = self.local();
        local.format("%d-%m-%Y").to_string()
    }

    pub fn time(&self, seconds: bool) -> String {
        let local = self.local();

        match seconds {
            true => local.format("%H-%M-%S").to_string(),
            false => local.format("%H-%M").to_string()
        }
    }

    pub fn formatted_datetime(&self) -> String {
        let date = self.date();
        let time = self.time(true);

        format!("[{} | {}]", date.replace("-", "/"), time.replace("-", ":"))
    }
}
