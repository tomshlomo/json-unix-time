use egui::{CollapsingHeader, Color32, RichText, Ui};
use serde_json::Value;

use crate::json_crawl::JsonPath;

pub fn tree(
    value: &Value,
    ui: &mut Ui,
    name: String,
    path: JsonPath,
    path_to_open: Option<&JsonPath>,
) {
    dbg!((
        &name,
        value,
        &path,
        path_to_open,
        path_to_open.is_some_and(|path_to_open| path.is_prefix_of(path_to_open)),
    ));
    match value {
        Value::Null => {
            ui.label(format!("{}: null", name));
        }
        Value::Bool(bool) => {
            ui.label(format!("{}: {}", name, bool));
        }
        Value::Number(num) => {
            let mut text = RichText::new(format!("{}: {}", name, num));
            if path_to_open.is_some_and(|path_to_open| path_to_open == &path) {
                text = text.background_color(Color32::from_rgb(0, 92, 128));
            };
            ui.label(text);
        }
        Value::String(s) => {
            ui.label(format!("{}: {}", name, s));
        }
        Value::Array(arr) => {
            let default_open =
                path_to_open.is_some_and(|path_to_open| path.is_prefix_of(path_to_open));
            CollapsingHeader::new(name)
                .default_open(default_open)
                .show(ui, |ui| {
                    for (i, sub_val) in arr.iter().enumerate() {
                        tree(
                            sub_val,
                            ui,
                            format!("{}", i),
                            path.append(crate::json_crawl::JsonPathPart::Index(i)),
                            path_to_open,
                        );
                    }
                });
        }
        Value::Object(obj) => {
            let default_open =
                path_to_open.is_some_and(|path_to_open| path.is_prefix_of(path_to_open));
            CollapsingHeader::new(name)
                .default_open(default_open)
                .show(ui, |ui| {
                    for (key, sub_val) in obj.into_iter() {
                        tree(
                            sub_val,
                            ui,
                            key.clone(),
                            path.append(crate::json_crawl::JsonPathPart::Field(key.clone())),
                            path_to_open,
                        );
                    }
                });
        }
    };
}

// #[derive(Clone, Default)]
// struct Tree(Vec<Tree>);

// impl Tree {
//     pub fn demo() -> Self {
//         Self(vec![
//             Tree(vec![Tree::default(); 4]),
//             Tree(vec![Tree(vec![Tree::default(); 2]); 3]),
//         ])
//     }

//     pub fn ui(&mut self, ui: &mut Ui) {
//         self.ui_impl(ui, 0, "root");
//     }
// }

// impl Tree {
//     fn ui_impl(&mut self, ui: &mut Ui, depth: usize, name: &str) {
//         CollapsingHeader::new(name)
//             .default_open(depth < 1)
//             .show(ui, |ui| self.children_ui(ui, depth));
//     }

//     fn children_ui(&mut self, ui: &mut Ui, depth: usize) {
//         if depth > 0
//             && ui
//                 .button(RichText::new("delete").color(ui.visuals().warn_fg_color))
//                 .clicked()
//         {
//             return;
//         }

//         self.0 = std::mem::take(self)
//             .0
//             .into_iter()
//             .enumerate()
//             .map(|(i, mut tree)| {
//                 if tree.ui_impl(ui, depth + 1, &format!("child #{i}")) == Action::Keep {
//                     Some(tree)
//                 } else {
//                     None
//                 }
//             })
//             .collect();

//         if ui.button("+").clicked() {
//             self.0.push(Tree::default());
//         }

//         Action::Keep
//     }
// }
