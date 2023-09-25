use crate::json_crawl::crawl_json;
use crate::{datetime::year_to_ts, json_crawl::JsonPath};
use chrono::{Datelike, Utc};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    // Example stuff:
    min_year: i32,
    max_year: i32,
    json_body: String,
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
            json_body: r#"
            {
                "now_ts": 1678912200,
                "shift_plan":{
                    "actual_end_shift_ts": null,
                    "actual_start_shift_ts": 1678967840,
                    "algo_data": {
                            "shift_plan_cost": {
                        "total_shift_plan_cost": 53279.33500000001,
                        "violation_component": 0
                      }
                    },
                    "capacity_configurations": [
                      {
                        "seats": 5,
                        "system_ride": 0,
                        "wheelchairs": 0
                      }
                    ],
                    "driver_id": "30001",
                    "end_shift_task": null,
                    "jobs": {
                      "break_jobs": [],
                      "end_shift_job": {
                        "actual_ts": null,
                        "eta": null,
                        "planned_end_ts": null,
                        "planned_location": null,
                        "planned_ts": null,
                        "relative_planned_end_ts": null,
                        "relative_planned_ts": null
                      },
                      "pass_through_jobs": [],
                      "ride_jobs": [],
                      "start_shift_job": {
                        "actual_ts": 1678967840,
                        "eta": null,
                        "planned_end_ts": null,
                        "planned_location": {
                          "address": {
                            "number": null,
                            "street": null
                          },
                          "bearing": -2.2948000000000093,
                          "description": null,
                          "edge_id": -1217565107,
                          "lat": 35.32366473610731,
                          "lng": -119.05894420476312,
                          "place_id": null,
                          "place_of_business": null,
                          "position_on_edge": 0.9675
                        },
                        "planned_ts": 1678967840,
                        "relative_planned_end_ts": null,
                        "relative_planned_ts": null
                      },
                      "wait_jobs": []
                    },
                    "license_plate": "p00001",
                    "operational_data": {
                      "future_projected_location": {
                        "edge_data": {
                          "edge_id": 109189331,
                          "position_on_edge": 0
                        },
                        "location": {
                          "bearing": 0,
                          "lat": 35.319382,
                          "lng": -119.056679
                        },
                        "location_time": 1678969380
                        ,
                        "tasks_until_future_location": []
                      },
                      "last_observed_location": {
                        "bearing": 0,
                        "lat": 35.323632762486,
                        "lng": -119.05842188395677,
                        "location_time": 1678969300
                      },
                      "projected_location": {
                        "edge_data": {
                          "edge_id": -1217565107,
                          "position_on_edge": 0.9675
                        },
                        "location": {
                          "bearing": 357.7052,
                          "lat": 35.32366473610731,
                          "lng": -119.05894420476312
                        },
                        "location_time": 1678969300
                      },
                      "suspected_off_route": null
                    },
                    "requested_end_shift_location": {
                              "address": {
                                  "number": null,
                                  "street": null
                              },
                              "bearing": null,
                              "description": "D\u00e9p\u00f4t Keolis Pam 94",
                              "edge_id": null,
                              "lat": 35.388518,
                              "lng": -119.020748,
                              "place_of_business": null,
                              "position_on_edge": null
                          },
                    "requested_end_shift_ts": 1678971600,
                    "requested_start_shift_location": null,
                    "requested_start_shift_ts": 1678968000,
                    "ride_groups": null,
                    "service_tags": [],
                    "shift_plan_id": "bbfa5acf-c587-4b5c-96f8-26fb80b67300",
                    "shift_routes": null,
                    "shift_times_source": "AUTOMATIC",
                    "shift_trips": null,
                    "shift_type": "DYNAMIC",
                    "start_shift_task": null,
                    "stop_times": null,
                    "van_id": "30001",
                    "van_tags": []
                  },
                "update_all_cache": false
              }
            "#
            .to_owned(),
        }
    }
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for TemplateApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let Self {
            min_year,
            max_year,
            json_body,
        } = self;

        let min_ts = year_to_ts(*min_year).unwrap() as f64;
        let max_ts = year_to_ts(*max_year + 1).unwrap() as f64;
        let predicate = |ts| (ts >= min_ts) && (ts <= max_ts);
        let parsed_json = serde_json::from_str(json_body).unwrap();
        let mut out = vec![];
        crawl_json(parsed_json, JsonPath::new(), &predicate, &mut out);

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::warn_if_debug_build(ui);
            ui.heading("JSON-unix-time");
            ui.add(egui::github_link_file!(
                "https://github.com/tomshlomo/json-unix-time",
                "Source code."
            ));
            ui.horizontal(|ui| {
                ui.label("Min year");
                ui.add(egui::DragValue::new(min_year).speed(1.0));
                ui.label("Max year");
                ui.add(egui::DragValue::new(max_year).speed(1.0));
            });

            egui::TextEdit::multiline(json_body)
                .hint_text("Paste your JSON here!")
                .show(ui);
        });
    }
}
