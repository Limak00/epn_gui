use std::{fs::File, io::BufReader};

use egui::*;

use super::{Enviroment, GenerationStatistic};

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(default))]
pub struct Visualize {
    pub srodowisko: Vec<Vec<Pos2>>,
    pub kurs: Vec<Vec<Pos2>>,
    stroke: Stroke,
    stroke_kurs: Stroke,
    enviroment: Enviroment,
    statistics: Vec<GenerationStatistic>,
    pokolenie: usize,
    individual_start: usize,
    individual_end: usize,
}

impl Default for Visualize {
    fn default() -> Self {
        Self {
            individual_start: 1,
            individual_end: 1,
            pokolenie: 0,
            statistics: vec![],
            enviroment: Enviroment::default(),
            srodowisko: Default::default(),
            kurs: Default::default(),
            stroke: Stroke::new(4.0, Color32::from_rgb(124, 252, 0)),
            stroke_kurs: Stroke::new(2.0, Color32::from_rgb(0, 204, 229)),
        }
    }
}

impl Visualize {
    pub fn ui_control(&mut self, ui: &mut Ui) {
        ui.separator();
        ui.separator();
        if ui.button("wczytaj dane").clicked() {
            self.srodowisko.clear();
            let file = File::open("simulation_statistics.json").unwrap();
            let reader = BufReader::new(file);

            self.statistics = match serde_json::from_reader(reader) {
                Ok(x) => x,
                Err(_) => Vec::default(),
            };
            
            let file = File::open("env.json").unwrap();
            let reader = BufReader::new(file);
            self.enviroment = match serde_json::from_reader(reader) {
                Ok(x) => x,
                Err(_) => Enviroment::default(),
            };
            //self.enviroment = serde_json::from_reader(reader).unwrap();

            let height = self.enviroment.height as f32;
            let width = self.enviroment.width as f32;

            self.srodowisko = self
                .enviroment
                .static_obstacles
                .iter()
                .map(|x| {
                    x.iter()
                        .map(|x| (x.x / width, (x.y / height)).into())
                        .cycle()
                        .take(x.len() + 1)
                        .collect()
                })
                .collect();
        }
        ui.separator();

        if self.statistics.len() > 2 {
            ui.label("Wybierz pokolenie");
            ui.add(
                egui::Slider::new(&mut self.pokolenie, 0..=self.statistics.len() - 1)
                    .text("Pokolenie"),
            );
        }
        ui.separator();
        ui.label("wybierz ilosc najlepszych osobników");
        match self.statistics.first() {
            Some(x) => {
                ui.label("wybierz ilosc najlepszych osobników");
                ui.add(
                    egui::Slider::new(&mut self.individual_start, 1..=self.individual_end)
                        .text("poczatek"),
                );
                ui.add(
                    egui::Slider::new(
                        &mut self.individual_end,
                        self.individual_start..=x.population.len() - 1,
                    )
                    .text("koniec"),
                );
            }
            None => {}
        }
    }

    pub fn ui_content(&mut self, ui: &mut Ui) -> egui::Response {
        let (response, painter) =
            ui.allocate_painter(ui.available_size_before_wrap(), Sense::drag());

        let to_screen = emath::RectTransform::from_to(
            Rect::from_min_size(Pos2::ZERO, response.rect.square_proportions()),
            response.rect,
        );
        // let from_screen = to_screen.inverse();

        match self.statistics.first() {
            #[allow(unused)]
            Some(x) => {
                let height = self.enviroment.height as f32;
                let width = self.enviroment.width as f32;
                self.kurs = self.statistics[self.pokolenie]
                    .population
                    .iter()
                    .map(|x| -> Vec<Pos2> {
                        x.points
                            .iter()
                            .map(|x| (x.x / width, (x.y / height)).into())
                            .collect()
                    })
                    .skip(self.individual_start)
                    .take(self.individual_end.abs_diff(self.individual_start) + 1)
                    .collect();
            }
            None => {}
        }

        let mut shapes = vec![];
        if ui.visuals().dark_mode {
            self.stroke = Stroke::new(self.stroke.width, Color32::from_rgb(124, 252, 0));
            self.stroke_kurs =
                Stroke::new(self.stroke_kurs.width, Color32::from_rgb(255, 255, 255));
        } else {
            self.stroke = Stroke::new(
                self.stroke.width,
                Color32::from_rgb(255 - 124, 255 - 252, 255 - 0),
            );
            self.stroke_kurs = Stroke::new(self.stroke_kurs.width, Color32::from_rgb(0, 0, 0));
        };
        for line in &self.srodowisko {
            if line.len() >= 1 {
                let points: Vec<Pos2> = line.iter().map(|p| to_screen * *p).collect();
                shapes.push(egui::Shape::line(points, self.stroke));
            }
        }

        for (i, line) in self.kurs.iter().enumerate() {
            if line.len() >= 1 {
                let points: Vec<Pos2> = line.iter().map(|p| to_screen * *p).collect();
                let stroke = Stroke::new(
                    self.stroke_kurs.width,
                    self.stroke_kurs
                        .color
                        .linear_multiply(1. / (1. + (0.2 * i as f32))),
                );

                shapes.push(egui::Shape::line(points, stroke));
            }
        }

        //przeszkody
        for obstacle in &self.enviroment.dynaic_obstacles {
            let mut object: Vec<Pos2> = obstacle.safe_sphere.iter().map(|p| p.to_pos2()).collect();
            object.push(object.first().unwrap().clone());
            let points: Vec<Pos2> = object.iter().map(|p| to_screen * *p).collect();
            shapes.push(egui::Shape::circle_filled(
                obstacle.position.to_pos2(),
                0.05,
                Color32::from_rgb(255, 255, 255),
            ));
            shapes.push(egui::Shape::line(points, self.stroke_kurs));
        }

        painter.extend(shapes);
        response
    }
}

impl Visualize {
    pub fn show(&mut self, ctx: &Context) {
        egui::CentralPanel::default().show(ctx, |ui| self.ui(ui));
        egui::SidePanel::right("Ustawienia").show(ctx, |ui| {
            self.ui_control(ui);
        });
    }
}

impl Visualize {
    fn ui(&mut self, ui: &mut Ui) {
        //self.ui_control(ui);

        Frame::canvas(ui.style()).show(ui, |ui| {
            ui.set_max_width(ui.available_size().y);
            self.ui_content(ui);
        });
    }
}
