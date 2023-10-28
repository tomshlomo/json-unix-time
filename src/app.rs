use crate::datetime::ts_to_str;
use crate::json_crawl::{crawl_json, JsonPathPart};
use crate::tree::tree;
use crate::{datetime::year_to_ts, json_crawl::JsonPath};
use chrono::{Datelike, Utc};
use egui::{vec2, Response, ScrollArea, Ui};

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

enum SortBy {
    Time,
    Path,
}
pub struct TemplateApp {
    // Example stuff:
    min_year: i32,
    max_year: i32,
    json_body: String,
    fmt: String,
    anchor: i64,
    highlighted_path: Option<JsonPath>,
    sort_by: SortBy,
    ascend: bool,
    instruction_open: bool,
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
            anchor: 1692694500,
            json_body: r#"{
  "field1": 1692694500,
  "field2": "I am a string",
  "field3": [1692684500, 1692693500, 1692699500],
  "field4": {
    "subfield1": null,
    "subfield2": 1692694500
  }
}"#
            .to_owned(),
            fmt: "%Y-%m-%d %H:%M:%S".to_owned(),
            highlighted_path: None,
            sort_by: SortBy::Time,
            ascend: true,
            instruction_open: false,
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
        Self::default()
        // Default::default()
    }

    fn clickable_strong_label(text: String, ui: &mut Ui) -> bool {
        let label = ui.add(
            egui::Label::new(egui::RichText::strong(egui::RichText::from(text)))
                .sense(egui::Sense::click()),
        );
        label.clicked()
    }
    fn table_ui(
        x: &[(JsonPath, i64)],
        fmt: &str,
        anchor: &mut i64,
        sort_by: &mut SortBy,
        ascend: &mut bool,
        ui: &mut egui::Ui,
    ) {
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
        let arrow = if *ascend { " ↗" } else { " ↘" };
        let (time_arrow, path_arrow) = match sort_by {
            SortBy::Time => (arrow, ""),
            SortBy::Path => ("", arrow),
        };
        let mut time_clicked = false;
        let mut path_clicked = false;
        table
            .header(20.0, |mut header| {
                header.col(|ui| {
                    ui.strong("Row");
                });
                header.col(|ui| {
                    time_clicked |=
                        Self::clickable_strong_label(format!("Unix-time{}", time_arrow), ui);
                    // ui.strong(format!("Unix-time{}", time_arrow));
                });
                header.col(|ui| {
                    time_clicked |=
                        Self::clickable_strong_label(format!("Human Readable{}", time_arrow), ui);
                    // ui.strong(format!("Human readable{}", time_arrow));
                });
                header.col(|ui| {
                    time_clicked |=
                        Self::clickable_strong_label(format!("Relative{}", time_arrow), ui);
                    // ui.strong(format!("Relative{}", time_arrow));
                });
                header.col(|ui| {
                    path_clicked |=
                        Self::clickable_strong_label(format!("Path in JSON{}", path_arrow), ui);
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
        if time_clicked {
            *sort_by = SortBy::Time
        } else if path_clicked {
            *sort_by = SortBy::Path
        };
        if time_clicked | path_clicked {
            *ascend = !*ascend;
        };
    }

    fn ui_file_drag_and_drop(&mut self, ctx: &egui::Context) {
        use egui::*;
        use std::fmt::Write as _;

        // Preview hovering files:
        if !ctx.input(|i| i.raw.hovered_files.is_empty()) {
            let text = ctx.input(|i| {
                let mut text = "Dropping files:\n".to_owned();
                for file in &i.raw.hovered_files {
                    if let Some(path) = &file.path {
                        write!(text, "\n{}", path.display()).ok();
                    } else if !file.mime.is_empty() {
                        write!(text, "\n{}", file.mime).ok();
                    } else {
                        text += "\n???";
                    }
                }
                text
            });

            let painter =
                ctx.layer_painter(LayerId::new(Order::Foreground, Id::new("file_drop_target")));

            let screen_rect = ctx.screen_rect();
            painter.rect_filled(screen_rect, 0.0, Color32::from_black_alpha(192));
            painter.text(
                screen_rect.center(),
                Align2::CENTER_CENTER,
                text,
                TextStyle::Heading.resolve(&ctx.style()),
                Color32::WHITE,
            );
        }

        // Collect dropped files:
        ctx.input(|i| {
            dbg!(&i.raw.dropped_files);
            if !i.raw.dropped_files.is_empty() {
                dbg!(&i.raw.dropped_files);
                if let Some(bytes) = i.raw.dropped_files[0].bytes.as_deref() {
                    println!("some bytes");
                    if let Ok(file_content) = std::str::from_utf8(bytes) {
                        println!("valid utf8");
                        self.json_body = file_content.to_owned();
                    }
                }
                // let y = x.clone();
                // let v: &[u8] = &y;
                // // let v = x.into_iter();

                // // let c = String::from_utf8(x.into_iter().collect());
                // let z = std::str::from_utf8(v);
            }
        });

        // Show dropped files (if any):
    }

    fn show_instructions(ctx: &egui::Context, open: &mut bool) {
        egui::Window::new("instructions")
            .open(open)
            .resizable(true)
            .vscroll(false)
            .show(ctx, |ui| {
                ui.label("Paste or drop any JSON file in the left box. \
                Any numeric field that is a valid unix timestamp, will be displayed on the table on the right.\n\n\
                A numeric value is considered a valid unix timestamp if it is between the min and max years.\n\n\
                The \"Relative\" column displays the time relative to the anchor. \
                You can set the anchor manually, or by right clicking any timestamp on the table.\n\n\
                The table can be sorted either by time or path in Json.\n\n\
                Left click a table cell to copy its content.
                ")
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
            highlighted_path,
            sort_by,
            ascend,
            instruction_open,
        } = self;
        Self::show_instructions(ctx, instruction_open);

        let min_ts = year_to_ts(*min_year).unwrap();
        let max_ts = year_to_ts(*max_year + 1).unwrap();
        let predicate = |ts| (ts >= min_ts) && (ts <= max_ts);
        let parsed_json = serde_json::from_str(json_body);

        egui::SidePanel::left("left panel").show(ctx, |ui| {
            ui.heading("JSON-unix-time");
            ui.hyperlink("https://github.com/tomshlomo/json-unix-time");
            egui::warn_if_debug_build(ui);

            if ui.button("Instructions").clicked() {
                *instruction_open = true;
            }
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
            ScrollArea::vertical().show(ui, |ui| {
                egui::TextEdit::multiline(json_body)
                    .hint_text("Paste your JSON here!")
                    .desired_width(f32::INFINITY)
                    .show(ui);
            });
        });
        egui::CentralPanel::default().show(ctx, |ui| match parsed_json {
            Ok(parsed_json) => {
                let mut out = vec![];
                crawl_json(&parsed_json, JsonPath::new(), &predicate, &mut out);
                // out.sort_by(|a, b| );
                match sort_by {
                    SortBy::Time => out.sort_by_key(|(path, ts)| (*ts, path.0.clone())),
                    SortBy::Path => {
                        out.sort_by_key(|(path, _)| path.0.clone())
                        // let z = out.iter().map(|(path, _)| path);
                    }
                }
                if !*ascend {
                    out.reverse();
                }
                ScrollArea::horizontal().show(ui, |ui| {
                    Self::table_ui(&out, fmt, anchor, sort_by, ascend, ui);
                });
                // let path_to_open = Some(JsonPath(vec![
                //     JsonPathPart::Field("field4".to_owned()),
                //     JsonPathPart::Field("subfield2".to_owned()),
                // ]));
                // tree(
                //     &parsed_json,
                //     ui,
                //     "".to_owned(),
                //     JsonPath::new(),
                //     path_to_open.as_ref(),
                // );
            }
            Err(err) => {
                ui.label(err.to_string());
            }
        });
        self.ui_file_drag_and_drop(ctx);
    }
}
