use worker_sys::Scheduled as Sys_Scheduled;

#[derive(Debug)]
pub struct Scheduled {
    cron: String,
    scheduled_time: f64,
}

impl Scheduled {
    pub fn new(cron: String, scheduled_time: f64) -> Self {
        Self {
            cron,
            scheduled_time,
        }
    }

    pub fn cron(&self) -> String {
        self.cron.clone()
    }

    pub fn scheduled_time(&self) -> f64 {
        self.scheduled_time.clone()
    }
}

impl From<Sys_Scheduled> for Scheduled {
    fn from(event: Sys_Scheduled) -> Self {
        Self {
            cron: event.cron().to_string(),
            scheduled_time: event.scheduled_time().into(),
            // r#type: event.r#type().to_string(),
            // cron: String::from("asd"),
            // scheduled_time: 0,
        }
    }
}
