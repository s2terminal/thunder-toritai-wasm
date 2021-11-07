#![recursion_limit = "128"]

use wasm_bindgen::prelude::*;
use yew::prelude::*;

use std::time::Duration;

use yew::services::{ConsoleService, IntervalService, Task};
use yew::{html, Callback, Component, ComponentLink, Html, ShouldRender};

#[wasm_bindgen]
extern "C" {
    fn getUnixtimeJS() -> u32;
}

struct Unixtime {
    number: u32
}
fn get_unixtime(add_sec: u32) -> Unixtime {
    Unixtime { number: getUnixtimeJS() + add_sec }
}
struct UnixtimeInterval {
    number: u32
}
impl UnixtimeInterval {
    fn to_string(&self) -> String {
        format!("{:>02}:{:>02}", self.number / 60, self.number % 60)
    }
    fn gt(&self, given: &UnixtimeInterval) -> bool {
        self.number > given.number
    }
}
impl PartialEq for UnixtimeInterval {
    fn eq(&self, other: &Self) -> bool {
        self.number == other.number
    }
}
impl Eq for UnixtimeInterval {}
fn new_interval(start: &Unixtime, end: &Unixtime) -> UnixtimeInterval {
    UnixtimeInterval { number: end.number - start.number }
}
fn from_m_s(m: u32, s: u32) -> UnixtimeInterval {
    UnixtimeInterval { number: m * 60 + s }
}

fn init_timeline() -> Vec<(UnixtimeInterval, String)> {
    let mut timeline: Vec<(UnixtimeInterval, String)> = Vec::new();
    timeline.clear();
    // TODO: 別のところで定義
    timeline.push((from_m_s(9, 40), String::from("中央エビ")));
    timeline.push((from_m_s(8, 50), String::from("中央ハチ")));
    timeline.push((from_m_s(7, 0), String::from("カメロトム")));
    timeline.push((from_m_s(5, 0), String::from("カメロトム（最短）")));
    timeline.push((from_m_s(3, 0), String::from("カメロトム（最短）")));
    timeline.push((from_m_s(2, 0), String::from("サンダー")));
    timeline
}

pub struct Model {
    link: ComponentLink<Self>,
    callback_tick: Callback<()>,
    job: Option<Box<dyn Task>>,
    end_at: Unixtime,
    left_time: UnixtimeInterval,
    timeline: Vec<(UnixtimeInterval, String)>,
}

pub enum Msg {
    StartInterval,
    Cancel,
    Tick,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Model {
            link: link.clone(),
            callback_tick: link.callback(|_| Msg::Tick),
            job: None,
            end_at: get_unixtime(0),
            left_time: UnixtimeInterval { number: 600 },
            timeline: init_timeline(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::StartInterval => {
                self.end_at = get_unixtime(600);
                {
                    let handle =
                        IntervalService::spawn(Duration::from_secs(1), self.callback_tick.clone());
                    self.job = Some(Box::new(handle));
                }
                ConsoleService::clear();
                ConsoleService::log("Interval started!");
            }
            Msg::Cancel => {
                self.job.take();
                ConsoleService::warn("Canceled!");
                ConsoleService::assert(self.job.is_none(), "Job still exists!");
            }
            Msg::Tick => {
                let now = get_unixtime(0);
                let left_time = new_interval(&now, &self.end_at);
                self.left_time = left_time;
            }
        }
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let has_job = self.job.is_some();
        html! {
            <div>
                <h1>{"タイマー"}</h1>
                <h2>{self.left_time.to_string()}</h2>
                <button onclick=self.link.callback(|_| Msg::StartInterval) disabled=has_job>{ "Start!" }</button>
                <button onclick=self.link.callback(|_| Msg::Cancel) disabled=!has_job>{ "Stop!" }</button>
                <table>
                    <thead><th>{"時間"}</th><th>{"イベント"}</th><th>{"状況"}</th></thead>
                    <tbody>
                        { for self.timeline.iter().map(|ts| html! {
                            <tr>
                                <th>{ts.0.to_string()}</th><td>{&ts.1}</td>
                                <td>
                                { match ts.0.gt(&self.left_time) {
                                    true => String::from("done"),
                                    false => String::from(""),
                                }}
                                </td>
                            </tr>
                        }) }
                    </tbody>
                </table>
                <hr/>
                <ul>
                    <li>{"リポジトリ: "}<a target="_blank" href="https://github.com/s2terminal/thunder-toritai-wasm">{"https://github.com/s2terminal/thunder-toritai-wasm"}</a></li>
                    <li>{"参考: "}<a target="_blank" href="https://unite-db.com/maps">{"Pokemon Unite Remoat Stadium Interactive Map • Unite-DB"}</a></li>
                </ul>
            </div>
        }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<Model>::new().mount_to_body();
}
