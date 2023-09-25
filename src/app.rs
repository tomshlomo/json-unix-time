use crate::datetime::ts_to_str;
use crate::json_crawl::crawl_json;
use crate::{datetime::year_to_ts, json_crawl::JsonPath};
use chrono::{Datelike, Utc};
use egui::{Response, Ui};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
// #[derive(serde::Deserialize, serde::Serialize)]
// #[serde(default)] // if we add new fields, give them default values when deserializing old state

fn add_copiable_label(text: String, ui: &mut Ui, with_hover_text: bool) -> Response {
    let mut label = ui.add(egui::Label::new(&text).sense(egui::Sense::click()));
    if with_hover_text {
        label = label.on_hover_text("Click to copy");
    }
    if label.clicked_by(egui::PointerButton::Primary) {
        ui.output_mut(|po| {
            po.copied_text = text;
        });
    }
    label
}

pub struct TemplateApp {
    // Example stuff:
    min_year: i32,
    max_year: i32,
    json_body: String,
    fmt: String,
    anchor: i64,
}
impl Default for TemplateApp {
    fn default() -> Self {
        let current_datetime = Utc::now();

        // Extract the year from the current date and time
        let current_year = current_datetime.year();

        Self {
            // Example stuff:
            min_year: current_year - 2,
            max_year: current_year + 3,
            anchor: 1678968000,
            json_body: r#"{
  "field1": 1678968000,
  "field2": "I am a string",
  "field3": [1678969000, 1678978000, 1678868000],
  "field4": {
    "subfield1": null,
    "subfield2": 1678968000
  }
}"#
            .to_owned(),
            fmt: "%Y-%m-%d %H:%M:%S".to_owned(),
        }
    }
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        // if let Some(storage) = cc.storage {
        //     return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        // }

        Default::default()
    }

    fn table_ui(x: &[(JsonPath, i64)], fmt: &str, anchor: &mut i64, ui: &mut egui::Ui) {
        use egui_extras::{Column, TableBuilder};

        let table = TableBuilder::new(ui)
            .striped(true)
            .resizable(true)
            .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
            .column(Column::auto())
            .column(Column::auto())
            .column(Column::auto())
            .column(Column::auto())
            .column(Column::remainder())
            .min_scrolled_height(0.0);

        table
            .header(20.0, |mut header| {
                header.col(|ui| {
                    ui.strong("Row");
                });
                header.col(|ui| {
                    ui.strong("Unix-time");
                });
                header.col(|ui| {
                    ui.strong("Human Readable");
                });
                header.col(|ui| {
                    ui.strong("Relative");
                });
                header.col(|ui| {
                    ui.strong("JSON Path");
                });
            })
            .body(|mut body| {
                for (row_index, (path, ts)) in x.iter().enumerate() {
                    let row_height = 18.0;
                    body.row(row_height, |mut row| {
                        row.col(|ui| {
                            ui.label(row_index.to_string());
                        });
                        row.col(|ui| {
                            let response = add_copiable_label(ts.to_string(), ui, false)
                                .on_hover_text(
                                    "Left click to copy.\nRight click to set as anchor.".to_owned(),
                                );
                            if response.clicked_by(egui::PointerButton::Secondary) {
                                *anchor = *ts
                            }
                            // ui.label(ts.to_string());
                        });
                        row.col(|ui| {
                            add_copiable_label(
                                ts_to_str(*ts, fmt).unwrap_or("N/A".to_owned()),
                                ui,
                                true,
                            );
                        });
                        row.col(|ui| {
                            let diff = ts - *anchor;
                            let abs_diff = diff.abs();
                            let hours = abs_diff / 3600;
                            let minutes = (abs_diff % 3600) / 60;
                            let seconds = abs_diff % 60;
                            let sign = if diff < 0 { "-" } else { "+" };
                            add_copiable_label(
                                format!("{}{:02}:{:02}:{:02}", sign, hours, minutes, seconds),
                                ui,
                                true,
                            );
                            // let dur = chrono::Duration::seconds(ts - anchor);
                            // ui.label(dur.to_string());
                        });
                        row.col(|ui| {
                            ui.style_mut().wrap = Some(false);
                            add_copiable_label(format!("{}", path), ui, true);
                        });
                    });
                }
            });
    }
}

impl eframe::App for TemplateApp {
    /// Called by the frame work to save state before shutdown.
    // fn save(&mut self, storage: &mut dyn eframe::Storage) {
    //     eframe::set_value(storage, eframe::APP_KEY, self);
    // }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let Self {
            min_year,
            max_year,
            json_body,
            fmt,
            anchor,
        } = self;

        let min_ts = year_to_ts(*min_year).unwrap();
        let max_ts = year_to_ts(*max_year + 1).unwrap();
        let predicate = |ts| (ts >= min_ts) && (ts <= max_ts);
        let parsed_json = serde_json::from_str(json_body);

        egui::SidePanel::left("left panel").show(ctx, |ui| {
            ui.heading("JSON-unix-time");
            ui.hyperlink("https://github.com/tomshlomo/json-unix-time");
            egui::warn_if_debug_build(ui);
            ui.separator();
            ui.horizontal(|ui| {
                ui.label("Min year:");
                ui.add(egui::DragValue::new(min_year).speed(1.0));
                ui.label("Max year:");
                ui.add(egui::DragValue::new(max_year).speed(1.0));
            });
            ui.horizontal(|ui| {
                ui.label("Datetime format:");
                ui.text_edit_singleline(fmt);
            });
            ui.horizontal(|ui| {
                ui.label("Anchor ts:");
                ui.add(egui::DragValue::new(anchor).speed(1.0));
                ui.label(ts_to_str(*anchor, fmt).unwrap_or("N/A".to_owned()));
            });
            ui.separator();
            egui::TextEdit::multiline(json_body)
                .hint_text("Paste your JSON here!")
                .show(ui);
        });
        egui::CentralPanel::default().show(ctx, |ui| match parsed_json {
            Ok(parsed_json) => {
                let mut out = vec![];
                crawl_json(parsed_json, JsonPath::new(), &predicate, &mut out);
                out.sort_by_key(|(_, ts)| *ts);
                Self::table_ui(&out, fmt, anchor, ui);
            }
            Err(err) => {
                ui.label(err.to_string());
            }
        });
    }
}
