use std::{fs::File, io::BufReader};

use egui::*;

use super::{Enviroment, GenerationStatistic};

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(default))]
pub struct Visualizedynamic {
    file_statistics: String,
    file_srodowisko: String,

    pub srodowisko: Vec<Vec<Pos2>>,
    pub kurs: Vec<Vec<Pos2>>,
    stroke: Stroke,
    stroke_kurs: Stroke,
    enviroment: Enviroment,
    statistics: Vec<GenerationStatistic>,
    pokolenie: usize,
    individual: usize,
    percent_of_path: usize,
    point: usize,
}

impl Default for Visualizedynamic {
    fn default() -> Self {
        Self {
            file_statistics: "simulation_statistics".to_owned(),
            file_srodowisko: "env".to_owned(),
            individual: 1,
            point: 1,
            pokolenie: 0,
            percent_of_path: 0,
            statistics: vec![],
            enviroment: Enviroment::default(),
            srodowisko: Default::default(),
            kurs: Default::default(),
            stroke: Stroke::new(4.0, Color32::from_rgb(124, 252, 0)),
            stroke_kurs: Stroke::new(2.0, Color32::from_rgb(0, 204, 229)),
        }
    }
}

impl Visualizedynamic {
    pub fn ui_control(&mut self, ui: &mut Ui) {
        ui.separator();
        ui.separator();
        ui.label("Plik srodowiska: ");
        ui.text_edit_singleline(&mut self.file_srodowisko);
        ui.label("Plik statystyk: ");
        ui.text_edit_singleline(&mut self.file_statistics);

        if ui.button("wczytaj dane").clicked() {
            self.srodowisko.clear();
            self.statistics.clear();
            let file = File::open(self.file_statistics.to_owned() + ".json").unwrap();
            let reader = BufReader::new(file);

            self.statistics = match serde_json::from_reader(reader) {
                Ok(x) => x,
                Err(_) => Vec::default(),
            };

            let file = File::open(self.file_srodowisko.to_owned() + ".json").unwrap();
            let reader = BufReader::new(file);
            self.enviroment = match serde_json::from_reader(reader) {
                Ok(x) => x,
                Err(_) => Enviroment::default(),
            };

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
        ui.label("wybierz ilosc najlepszych osobnika");
        match self.statistics.first() {
            Some(x) => {
                ui.label("wybierz ilosc  osobnika");
                ui.add(
                    egui::Slider::new(&mut self.individual, 1..=x.population.len() - 1)
                        .text("Osobnik"),
                );
                ui.label("punkt osobnika");
                ui.add(
                    egui::Slider::new(
                        &mut self.point,
                        1..=self.statistics[self.pokolenie].population[self.individual]
                            .points
                            .len(),
                    )
                    .text("punkt"),
                );
                ui.label("procentowy progres");
                ui.add(egui::Slider::new(&mut self.percent_of_path, 0..=100).text("%"));
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

        //TODO

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
                    .skip(self.individual)
                    .take(1)
                    .collect();
            }
            None => {}
        }

        let mut shapes = vec![];
        // kolory w zaleznosci od motywu
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
        // pbiekty statyczne
        for line in &self.srodowisko {
            if line.len() >= 1 {
                let points: Vec<Pos2> = line.iter().map(|p| to_screen * *p).collect();
                shapes.push(egui::Shape::line(points, self.stroke));
            }
        }
        //kurs
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

        // pukty pocztkowe i koncowe
        shapes.push(egui::Shape::circle_filled(
            to_screen
                * vec2(
                    self.enviroment.starting_point.x / self.enviroment.width as f32,
                    self.enviroment.starting_point.y / self.enviroment.height as f32,
                )
                .to_pos2(),
            3.0,
            Color32::from_rgb(0, 255, 255),
        ));
        shapes.push(egui::Shape::circle_filled(
            to_screen
                * vec2(
                    self.enviroment.ending_point.x / self.enviroment.width as f32,
                    self.enviroment.ending_point.y / self.enviroment.height as f32,
                )
                .to_pos2(),
            3.0,
            Color32::from_rgb(255, 0, 255),
        ));
        shapes.push(egui::Shape::text(
            &ui.fonts(),
            to_screen
                * ((vec2(
                    self.enviroment.starting_point.x / self.enviroment.width as f32 + 0.01,
                    self.enviroment.starting_point.y / self.enviroment.height as f32,
                ))
                .to_pos2()),
            Align2::LEFT_TOP,
            format!(
                "P. Poczatkowy\nX:{:.3} Y:{:.3}",
                self.enviroment.starting_point.x, self.enviroment.starting_point.y
            ),
            FontId::new(12., FontFamily::Monospace),
            Color32::from_rgb(0, 255, 255),
        ));
        shapes.push(egui::Shape::text(
            &ui.fonts(),
            to_screen
                * ((vec2(
                    self.enviroment.ending_point.x / self.enviroment.width as f32 + 0.01,
                    self.enviroment.ending_point.y / self.enviroment.height as f32,
                ))
                .to_pos2()),
            Align2::LEFT_TOP,
            format!(
                "P. Koncowy\nX:{:.3} Y:{:.3}",
                self.enviroment.ending_point.x, self.enviroment.ending_point.y
            ),
            FontId::new(12., FontFamily::Monospace),
            Color32::from_rgb(255, 0, 255),
        ));

        //dynamiczne przeszkody

        for obstacle in &self.enviroment.dynamic_obstacles {
            let mut object: Vec<Pos2> = obstacle
                .safe_sphere
                .iter()
                .map(|p| {
                    vec2(
                        p.x / self.enviroment.width as f32,
                        p.y / self.enviroment.height as f32,
                    )
                    .to_pos2()
                })
                .collect();
            object.push(object.first().unwrap().clone());
            let points: Vec<Pos2> = object.iter().map(|p| to_screen * *p).collect();
            shapes.push(egui::Shape::circle_filled(
                to_screen
                    * vec2(
                        obstacle.position.x / self.enviroment.width as f32,
                        obstacle.position.y / self.enviroment.height as f32,
                    )
                    .to_pos2(),
                1.0,
                Color32::from_rgb(255, 255, 255),
            ));
            //pozycja dynamicznych przeszkod
            shapes.push(egui::Shape::text(
                &ui.fonts(),
                to_screen
                    * ((vec2(
                        obstacle.position.x / self.enviroment.width as f32,
                        obstacle.position.y / self.enviroment.height as f32 + 0.02,
                    ))
                    .to_pos2()),
                Align2::CENTER_BOTTOM,
                format!("X:{:.3} Y:{:.3}", obstacle.position.x, obstacle.position.y),
                FontId::new(8., FontFamily::Monospace),
                Color32::from_rgb(0, 255, 255),
            ));

            shapes.push(egui::Shape::line(points, self.stroke_kurs));
        }

        painter.extend(shapes);
        response
    }
}

impl Visualizedynamic {
    pub fn show(&mut self, ctx: &Context) {
        egui::CentralPanel::default().show(ctx, |ui| self.ui(ui));
        egui::SidePanel::right("Ustawienia").show(ctx, |ui| {
            self.ui_control(ui);
        });
    }
}

impl Visualizedynamic {
    fn ui(&mut self, ui: &mut Ui) {
        //self.ui_control(ui);

        Frame::canvas(ui.style()).show(ui, |ui| {
            ui.set_max_width(ui.available_size().y);
            self.ui_content(ui);
        });
    }
}
