use chrono::{Local, Timelike};
use leptos::*;
use wasm_bindgen::JsCast;

#[component]
fn Clock(cx: Scope) -> impl IntoView {
    let (date, set_date) = create_signal(cx, Local::now());

    set_interval_with_handle(
        move || {
            set_date(Local::now());
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

            let hh = date().hour() * 30;
            let mm = date().minute() * 6;
            let ss = date().second() * 6;

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

    view! { cx,
        <div>
            <div class="clock border-black dark:border-white">
                <div class="hour">
                    {
                        move || view! {cx, <div id="hr" class="hr before:bg-black before:dark:bg-white"></div>}
                    }
                </div>
                <div class="min">
                    {
                        move || view! {cx, <div id="mn" class="mn before:bg-black before:dark:bg-white"></div>}
                    }
                </div>
                <div class="sec">
                    {
                        move || view! {cx, <div id="sc" class="sc" ></div>}
                    }
                </div>
            </div>

            <div id="digitalClock" class="text-black dark:text-white">
                {
                    move || view! {cx,

                        <div>{format!("{:02}:{:02}:{:02}", date().hour(), date().minute(), date().second())}</div>
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
    mount_to_body(|cx| view! { cx,  <Clock/> })
}
