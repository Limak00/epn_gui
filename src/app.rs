use std::f32::consts::PI;

use egui::{vec2, Vec2};
use serde::{Deserialize, Serialize};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
use self::painting::Painting;
pub mod painting;

use self::podglad::Visualize;
pub mod podglad;

use self::dynamic_viev::Visualizedynamic;
pub mod dynamic_viev;

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct EpnGui {
    // Example stuff:
    //label: String,
    //#[serde(skip)]
    //value: f32,
    #[serde(skip)]
    state: State,
    // this how you opt-out of serialization of a member
    #[serde(skip)]
    painting: Painting,
    #[serde(skip)]
    visualize: Visualize,
    #[serde(skip)]
    visualizedynamic: Visualizedynamic,
}

pub struct State {
    //ustawienia_algorytmu: bool,
    rysowanie: bool,
    pokolenia: bool,
    tri: bool,
    selected_anchor: String,
}

impl Default for EpnGui {
    fn default() -> Self {
        Self {
            // Example stuff:
            state: State {
                //ustawienia_algorytmu: (true),
                rysowanie: (true),
                pokolenia: (false),
                tri: (false),
                selected_anchor: ("Rysowanie").to_string(),
            },
            //label: "Hello World!".to_owned(),
            //value: 2.7,
            painting: Painting::default(),
            visualize: Visualize::default(),
            visualizedynamic: Visualizedynamic::default(),
        }
    }
}

impl EpnGui {
    /// Called once before the first frame.
    #[allow(unused)]
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customized the look at feel of egui using
        // `cc.egui_ctx.set_visuals`ctx.set_fonts`.
        // Load previous app state (if any). and `cc.egui_
        // Note that you must enable the `persistence` feature for this to work.
        cc.egui_ctx.set_visuals(egui::Visuals::dark());
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for EpnGui {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }
    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.

    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let Self {
            state,

            //label,
            //value,
            painting,
            visualize,
            visualizedynamic,
        } = self;
        // The top panel is often a good place for a menu bar:

        // Examples of how to create different panels and windows.
        // PIck whichever suits you.
        // Tip: a good default choice is to just keep the `CentralPanel`.
        // For insPIration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            ui.horizontal_wrapped(|ui| {
                egui::widgets::global_dark_light_mode_switch(ui);
                //ui.toggle_value(&mut state.ustawienia_algorytmu, "Ustawienia algorytmu");

                ui.separator();

                ui.with_layout(egui::Layout::left_to_right(), |ui| {
                    let mut selected_anchor = state.selected_anchor.clone();

                    for (name, anchor) in [
                        ("Rysowanie", &mut state.rysowanie),
                        ("wizualizacja", &mut state.pokolenia),
                        ("wizualizacja dynamiczna", &mut state.tri),
                    ] {
                        if ui.selectable_label(selected_anchor == name, name).clicked() {
                            selected_anchor = name.to_owned();
                            *anchor = true;
                        }
                        if selected_anchor != name {
                            *anchor = false;
                        }
                    }
                    state.selected_anchor = selected_anchor;
                });

                ui.separator();

                if ui.button("Quit").clicked() {
                    frame.quit();
                }
            });
        });

        //  if state.ustawienia_algorytmu {
        //      egui::SidePanel::left("ustawienia_algorytmu").show(ctx, |ui| {
        //          ui.heading("Ustawienia_algorytmu");
        //
        //          ui.horizontal(|ui| {
        //              ui.label("Write something: ");
        //              ui.text_edit_singleline(label);
        //          });
        //
        //          ui.add(egui::Slider::new(value, 0.0..=10.0).text("value"));
        //          if ui.button("Increment").clicked() {
        //              *value += 1.0;
        //          }
        //
        //      });
        //  }
        //
        if state.rysowanie {
            painting.show(ctx);
        }
        if state.pokolenia {
            visualize.show(ctx);
        }
        if state.tri {
            visualizedynamic.show(ctx);
        }
    }

    fn on_exit_event(&mut self) -> bool {
        true
    }

    fn on_exit(&mut self, _gl: &eframe::glow::Context) {}

    fn auto_save_interval(&self) -> std::time::Duration {
        std::time::Duration::from_secs(30)
    }

    fn max_size_points(&self) -> egui::Vec2 {
        egui::Vec2::INFINITY
    }

    fn clear_color(&self, _visuals: &egui::Visuals) -> egui::Rgba {
        // NOTE: a bright gray makes the shadows of the windows look weird.
        // We use a bit of transparency so that if the user switches on the
        // `transparent()` option they get immediate results.
        egui::Color32::from_rgba_unmultiplied(12, 12, 12, 180).into()

        // _visuals.window_fill() would also be a natural choice
    }

    fn persist_native_window(&self) -> bool {
        true
    }

    fn persist_egui_memory(&self) -> bool {
        true
    }

    fn warm_up_enabled(&self) -> bool {
        false
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Individual {
    pub fitness: f64,
    pub feasible: bool,
    pub points: Vec<Vec2>,
    pub evaluated: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize, Default)]
pub struct GenerationStatistic {
    pub generation: usize,
    pub population: Vec<Individual>,
    pub mutation_operators_weights: Vec<f64>,
    pub mutation_operators_uses: Vec<usize>,
    pub crossover_operators_uses: usize,
}
#[derive(Clone, Debug, Deserialize, Serialize, Default)]
pub struct Enviroment {
    pub width: f64,
    pub height: f64,
    pub starting_point: Vec2,
    pub ending_point: Vec2,
    pub static_obstacles: Vec<Vec<Vec2>>,
    pub dynamic_obstacles: Vec<DynaicObstacle>,
}
#[derive(Clone, Debug, Deserialize, Serialize, Default)]
pub struct DynaicObstacle {
    pub position: Vec2,
    pub course: f32,
    pub speed: f32,
    pub safe_sphere: Vec<Vec2>,
}

impl DynaicObstacle {
    pub fn new(position: Vec2, course: f32, speed: f32) -> Self {
        let mut safe_sphere = vec![];
        let mut xx: f32;
        let mut yy: f32;
        let x = position.x * 100.0;
        let y = position.y * 100.0;
        let d: f32 = 2.0;
        let co: f32 = course * PI / 180.0;
        safe_sphere.clear();

        xx = x + d * (-co).sin();
        yy = y + d * (co + PI).cos();
        safe_sphere.push(vec2(xx, yy));

        xx = x + d * (co + PI / 2.0).sin();
        yy = y + d * (co + PI / 2.0).cos();
        safe_sphere.push(vec2(xx, yy));

        xx = x + (4.47214) * (co + 26.565 * PI / 180.0).sin();
        yy = y + (4.47214) * (co + 26.565 * PI / 180.0).cos();
        safe_sphere.push(vec2(xx, yy));

        xx = x + 3.0 * d * (co).sin();
        yy = y + 3.0 * d * (co).cos();
        safe_sphere.push(vec2(xx, yy));

        xx = x + (5.65685) * (co + 315.0 * PI / 180.0).sin();
        yy = y + (5.65685) * (co + 315.0 * PI / 180.0).cos();
        safe_sphere.push(vec2(xx, yy));

        xx = x + d * d * (co - PI / 2.0).sin();
        yy = y + d * d * (-co + PI / 2.0).cos();
        safe_sphere.push(vec2(xx, yy));

        safe_sphere = safe_sphere
            .iter()
            .map(|x| vec2(x.x / 100.0, x.y / 100.0))
            .collect();

        Self {
            position,
            course,
            speed,
            safe_sphere,
        }
    }
}
