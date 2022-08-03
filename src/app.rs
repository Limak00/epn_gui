
use egui::Vec2;
use serde::{Serialize, Deserialize};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
use self::painting::Painting;
pub mod painting;

use self::podglad::Visualize;
pub mod podglad;

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct Epn_Gui {
    // Example stuff:
    label: String,
    #[serde(skip)]
    state: State,
    // this how you opt-out of serialization of a member
    #[serde(skip)]
    value: f32,
    #[serde(skip)]
    painting: Painting,
    #[serde(skip)]
    visualize: Visualize,
}

pub struct State {
    ustawienia_algorytmu: bool,
    rysowanie: bool,
    pokolenia: bool,
    tri: bool,
    selected_anchor: String,
}

impl Default for Epn_Gui {
    fn default() -> Self {
        Self {
            // Example stuff:
            state: State {
                ustawienia_algorytmu: (true),
                rysowanie: (true),
                pokolenia: (false),
                tri: (false),
                selected_anchor: ("Rysowanie").to_string(),
            },
            label: "Hello World!".to_owned(),
            value: 2.7,
            painting: Painting::default(),
            visualize: Visualize::default(),
        }
    }
}

impl Epn_Gui {
    /// Called once before the first frame.

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

impl eframe::App for Epn_Gui {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }
    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.

    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let Self {
            state,
            label,
            value,
            painting,
            visualize,
        } = self;
        // The top panel is often a good place for a menu bar:

        // Examples of how to create different panels and windows.
        // Pick whichever suits you.
        // Tip: a good default choice is to just keep the `CentralPanel`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            ui.horizontal_wrapped(|ui| {
                egui::widgets::global_dark_light_mode_switch(ui);
                ui.toggle_value(&mut state.ustawienia_algorytmu, "Ustawienia algorytmu");

                ui.separator();

                ui.with_layout(egui::Layout::left_to_right(), |ui| {
                    let mut selected_anchor = state.selected_anchor.clone();

                    for (name, anchor) in [
                        ("Rysowanie", &mut state.rysowanie),
                        ("tri", &mut state.tri),
                        ("wizualizacja", &mut state.pokolenia),
                    ] {
                        if ui.selectable_label(selected_anchor == name, name).clicked() {
                            selected_anchor = name.to_owned();
                            *anchor = true;
                        }
                        if selected_anchor !=name{
                            *anchor=false;
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

        if state.ustawienia_algorytmu {
            egui::SidePanel::left("ustawienia_algorytmu").show(ctx, |ui| {
                ui.heading("Ustawienia_algorytmu");

                ui.horizontal(|ui| {
                    ui.label("Write something: ");
                    ui.text_edit_singleline(label);
                });

                ui.add(egui::Slider::new(value, 0.0..=10.0).text("value"));
                if ui.button("Increment").clicked() {
                    *value += 1.0;
                }

                ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                    ui.horizontal(|ui| {
                        ui.spacing_mut().item_spacing.x = 0.0;
                        ui.label("powered by ");
                        ui.hyperlink_to("egui", "https://github.com/emilk/egui");
                        ui.label(" and ");
                        ui.hyperlink_to(
                            "eframe",
                            "https://github.com/emilk/egui/tree/master/eframe",
                        );
                    });
                });
            });
        }

        if state.rysowanie {
            painting.show(ctx, &mut true);
            
            state.ustawienia_algorytmu=true;
        }
        if state.pokolenia {
            visualize.show(ctx, &mut true);
            state.ustawienia_algorytmu=false;
            
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
    pub operator_names: Vec<String>,
}
#[derive(Clone, Debug, Deserialize, Serialize, Default)]
pub struct Enviroment {
    pub width: f64,
    pub height: f64,
    pub starting_point: Vec2,
    pub ending_point: Vec2,
    pub static_obstacles: Vec<Vec<Vec2>>,
}