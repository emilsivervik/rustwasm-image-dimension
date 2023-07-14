use std::result::Result as StdResult;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen (extends = ::js_sys::Object , js_name = Scheduled)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub type Scheduled;
    //   #[wasm_bindgen(structural, method, getter, js_name=type)]
    //    pub fn r#type(this: &Scheduled) -> String;

    #[wasm_bindgen(structural, method, getter, js_class = "Scheduled", js_name=cron)]
    pub fn cron(this: &Scheduled) -> String;

    #[wasm_bindgen(structural, method, getter, js_class = "Scheduled", js_name=scheduledTime)]
    pub fn scheduled_time(this: &Scheduled) -> i32;
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen (extends = ::js_sys::Object , js_name = ScheduledContext)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ScheduledContext` dictionary."]
    pub type ScheduledContext;

    #[wasm_bindgen(catch, method, js_class = "ScheduledContext", js_name = waitUntil)]
    pub fn wait_until(
        this: &ScheduledContext,
        p: &::js_sys::Promise,
    ) -> StdResult<(), wasm_bindgen::JsValue>;
}
