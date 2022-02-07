use std::sync::mpsc::{channel, Receiver, Sender};
use eframe::{egui, epi};
use eframe::egui::{Color32, InnerResponse, Response, ScrollArea, Ui};
use eframe::egui::plot::{Bar, BarChart, Legend, Plot, Value};
use crate::RT;

pub enum AppMsg {
    Count(f32),
    Data(String)
}

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "persistence", serde(default))] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    // Example stuff:
    label: String,

    // this how you opt-out of serialization of a member
    #[cfg_attr(feature = "persistence", serde(skip))]
    value: f32,
    rx: Receiver<AppMsg>,
    tx: Sender<AppMsg>,
}

impl Default for TemplateApp {
    fn default() -> Self {
        let (tx, rx) = channel();
        Self {
            label: "Hello World!".to_owned(),
            value: 2.7,
            rx,
            tx
        }
    }
}

impl epi::App for TemplateApp {
    fn name(&self) -> &str {
        "eframe template"
    }

    /// Called once before the first frame.
    fn setup(
        &mut self,
        _ctx: &egui::CtxRef,
        _frame: & epi::Frame,
        _storage: Option<&dyn epi::Storage>,
    ) {
        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        #[cfg(feature = "persistence")]
        if let Some(storage) = _storage {
            *self = epi::get_value(storage, epi::APP_KEY).unwrap_or_default()
        }
    }

    /// Called by the frame work to save state before shutdown.
    /// Note that you must enable the `persistence` feature for this to work.
    #[cfg(feature = "persistence")]
    fn save(&mut self, storage: &mut dyn epi::Storage) {
        epi::set_value(storage, epi::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::CtxRef, frame: &epi::Frame) {
        let Self { label, value , rx, tx} = self;

        if let Ok(msg) = rx.try_recv() {
            match msg {
                AppMsg::Count(val) => {
                    *value = val;
                }
                AppMsg::Data(data) => {
                    label.push_str(&data);
                }
            }
        }

        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            ui.heading("Side Panel");

            for i in 0..10 {
                let txt = format!("test: {}", i);
                if ui.button(&txt).clicked() {
                    println!("{}", txt);
                }
            }

            ui.horizontal(|ui| {
                ui.label("Write something: ");
                ui.text_edit_singleline(label);
            });


            ui.add(egui::Slider::new(value, 0.0..=100000.0).text("value"));
            if ui.button("Start").clicked() {
                let frame = frame.clone();
                let tx = tx.clone();
                let val = *value as i32;

                RT.spawn(async move {
                    // let resp = reqwest::get("https://httpbin.org/ip")
                    //     .await.unwrap()
                    //     .json::<std::collections::HashMap<String, String>>()
                    //     .await.unwrap();
                    // println!("{:#?}", resp);
                    let data = (0..val).into_iter().map(|i| format!("Data asdlfkj laskdf öaskdfölsadkfölsadfksl öadföasldf ölasldfk ölkasödlfk {}", i)).collect::<Vec<String>>().join("\n");
                    tx.send(AppMsg::Data(data));
                    frame.request_repaint();
                });
            }

            bar_stacked(ui);

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 0.0;
                    ui.label("powered by ");
                    ui.hyperlink_to("egui", "https://github.com/emilk/egui");
                    ui.label(" and ");
                    ui.hyperlink_to("eframe", "https://github.com/emilk/egui/tree/master/eframe");
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ScrollArea::vertical().max_width(f32::INFINITY)
                .stick_to_bottom()
                .show(ui, |ui|{
                    ui.add(
                        egui::TextEdit::multiline(label)
                            .frame(false)
                            .text_style(egui::TextStyle::Monospace)
                            .desired_width(f32::INFINITY)
                    );
            });
        });
    }
}

fn bar_gauss(ui: &mut Ui)  {
    let mut chart = BarChart::new(
        (-395..=395)
            .step_by(10)
            .map(|x| x as f64 * 0.01)
            .map(|x| {
                (
                    x,
                    (-x * x / 2.0).exp() / (2.0 * std::f64::consts::PI).sqrt(),
                )
            })
            // The 10 factor here is purely for a nice 1:1 aspect ratio
            .map(|(x, f)| Bar::new(x, f * 10.0).width(0.095))
            .collect(),
    )
        .color(Color32::LIGHT_BLUE)
        .name("Normal Distribution");


    let plot = Plot::new("Normal Distribution Demo")
        .legend(Legend::default())
        .data_aspect(1.0);

    let InnerResponse {
        response,
        inner: (screen_pos, pointer_coordinate, pointer_coordinate_drag_delta, bounds, hovered, chart),
    } = plot.show(ui, |plot_ui| {
        (
            plot_ui.screen_from_plot(Value::new(0.0, 0.0)),
            plot_ui.pointer_coordinate(),
            plot_ui.pointer_coordinate_drag_delta(),
            plot_ui.plot_bounds(),
            plot_ui.plot_hovered(),
            plot_ui.bar_chart(chart),
        )
    });

    if response.clicked() {
        println!("SP: {:?}", pointer_coordinate);
    }
}

fn bar_stacked(ui: &mut Ui)  {
    let mut chart1 = BarChart::new(vec![
        Bar::new(0.5, 1.0).name("Day 1"),
        Bar::new(1.5, 3.0).name("Day 2"),
        Bar::new(2.5, 1.0).name("Day 3"),
        Bar::new(3.5, 2.0).name("Day 4"),
        Bar::new(4.5, 4.0).name("Day 5"),
    ])
        .width(0.7)
        .name("Set 1");

    let mut chart2 = BarChart::new(vec![
        Bar::new(0.5, 1.0),
        Bar::new(1.5, 1.5),
        Bar::new(2.5, 0.1),
        Bar::new(3.5, 0.7),
        Bar::new(4.5, 0.8),
    ])
        .width(0.7)
        .name("Set 2")
        .stack_on(&[&chart1]);

    let mut chart3 = BarChart::new(vec![
        Bar::new(0.5, -0.5),
        Bar::new(1.5, 1.0),
        Bar::new(2.5, 0.5),
        Bar::new(3.5, -1.0),
        Bar::new(4.5, 0.3),
    ])
        .width(0.7)
        .name("Set 3")
        .stack_on(&[&chart1, &chart2]);

    let mut chart4 = BarChart::new(vec![
        Bar::new(0.5, 0.5),
        Bar::new(1.5, 1.0),
        Bar::new(2.5, 0.5),
        Bar::new(3.5, -0.5),
        Bar::new(4.5, -0.5),
    ])
        .width(0.7)
        .name("Set 4")
        .stack_on(&[&chart1, &chart2, &chart3]);

    let InnerResponse {  inner: (a), response }  = Plot::new("Stacked Bar Chart Demo")
        .legend(Legend::default())
        .data_aspect(1.0)
        .show(ui, |plot_ui| {
            plot_ui.bar_chart(chart1);
            plot_ui.bar_chart(chart2);
            plot_ui.bar_chart(chart3);
            plot_ui.bar_chart(chart4);
            plot_ui.pointer_coordinate()
        });

    if response.clicked() {
        println!("Clicked: {:?}", a);
    }

    // response.context_menu(|ui| {
    //     ui.menu_button("Plot", |ui| {
    //
    //         if ui.button("Open...").clicked() {
    //             ui.close_menu();
    //         }
    //     });
    // });

}

fn is_approx_integer(val: f64) -> bool {
    val.fract().abs() < 1e-6
}

fn is_approx_zero(val: f64) -> bool {
    val.abs() < 1e-6
}