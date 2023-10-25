use chrono::{Local, Timelike};
use leptos::*;
use wasm_bindgen::JsCast;

#[component]
fn Clock() -> impl IntoView {
    let (date, set_date) = create_signal(Local::now());

    set_interval_with_handle(
        move || {
            set_date.set(Local::now());
            let hr: web_sys::HtmlElement = document()
                .query_selector("#hr")
                .unwrap()
                .unwrap()
                .dyn_into()
                .unwrap();

            let mn: web_sys::HtmlElement = document()
                .query_selector("#mn")
                .unwrap()
                .unwrap()
                .dyn_into()
                .unwrap();
            let sc: web_sys::HtmlElement = document()
                .query_selector("#sc")
                .unwrap()
                .unwrap()
                .dyn_into()
                .unwrap();

            let hh = date.get().hour() * 30;
            let mm = date.get().minute() * 6;
            let ss = date.get().second() * 6;

            hr.style()
                .set_property("transform", &format!("rotateZ({}deg)", hh + (mm / 12)))
                .unwrap();
            mn.style()
                .set_property("transform", &format!("rotateZ({mm}deg)"))
                .unwrap();
            sc.style()
                .set_property("transform", &format!("rotateZ({ss}deg)"))
                .unwrap();
        },
        std::time::Duration::from_secs(1),
    )
    .unwrap();

    view! {
        <div>
            <div class="clock border-black dark:border-white">
                <div class="hour">
                    {
                        move || view! { <div id="hr" class="hr before:bg-black before:dark:bg-white"></div>}
                    }
                </div>
                <div class="min">
                    {
                        move || view! { <div id="mn" class="mn before:bg-black before:dark:bg-white"></div>}
                    }
                </div>
                <div class="sec">
                    {
                        move || view! { <div id="sc" class="sc" ></div>}
                    }
                </div>
            </div>

            <div id="digitalClock" class="text-black dark:text-white">
                {
                    move || view! {

                        <div>{format!("{:02}:{:02}:{:02}", date.get().hour(), date.get().minute(), date.get().second())}</div>
                    }
                }
                <div id="ampm"></div>
            </div>
        </div>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    _ = console_log::init_with_level(log::Level::Info);
    console_error_panic_hook::set_once();
    mount_to_body(|| view! {  <Clock/> })
}
