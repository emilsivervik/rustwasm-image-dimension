use cfg_if::cfg_if;
use serde_json::json;
use worker::*;

mod formatter;
mod namnsdag;
mod slack;

cfg_if! {
    // https://github.com/rustwasm/console_error_panic_hook#readme
    if #[cfg(feature = "console_error_panic_hook")] {
        extern crate console_error_panic_hook;
        pub use self::console_error_panic_hook::set_once as set_panic_hook;
    } else {
        #[inline]
        pub fn set_panic_hook() {}
    }
}

fn log_event(event: &worker::Scheduled) {
    console_log!(
        "cron={}
scheduledTime={}",
        event.cron(),
        event.scheduled_time()
    );
}

#[event(scheduled)]
pub async fn main(event: Scheduled, env: worker::Env) -> worker::Result<()> {
    set_panic_hook();
    log_event(&event);

    let slack_url = env
        .secret("SLACK_URL")
        .map(|f| f.to_string())
        .expect("SLACK_HOOK env variable not set.");

    let (slack_message, err): (serde_json::Value, Option<reqwest_wasm::Error>) =
        match namnsdag::get_namnsdag().await {
            Ok(payload) => {
                let name_vector = namnsdag::get_names(&payload);
                let name_string = formatter::format_names_string(name_vector);

                let namn_formatted =
                    format!("Idag har {} namnsdag. Ping <@UR2FY3P5H>", name_string);

                (json!({ "text": namn_formatted }), None)
            }
            Err(err) => (json!({ "text": err.to_string() }), Some(err)),
        };

    if let Some(err) = err {
        console_log!("{}", err);
    }

    slack::send_to_slack(&slack_url, &slack_message).await;

    Ok(())
}
