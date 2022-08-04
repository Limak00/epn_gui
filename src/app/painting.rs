use std::fmt::format;
use std::fs::File;
use std::io::BufWriter;
use std::sync::Arc;

use super::DynaicObstacle;
use super::Enviroment;

use eframe::epaint::CircleShape;
use eframe::epaint::TextShape;
use egui::*;
#[derive(PartialEq, Clone, Debug)]
enum Enum {
    Start,
    End,
    Dym,
    Stat,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(default))]
pub struct Painting {
    /// in 0-1 normalized coordinates
    pub srodowisko: Vec<Vec<Pos2>>,
    pub kurs: Vec<Vec<Pos2>>,
    stroke: Stroke,
    stroke_kurs: Stroke,
    enviroment: Enviroment,
    menu: Menu,
    my_enum: Enum,
}
#[derive(Clone, Debug, Default)]
pub struct Menu {
    individual_start: Vec2,
    individual_end: Vec2,
    obstacle: Vec<DynaicObstacle>,
    obstacle_course: f32,
    obstacle_pos: Vec2,
    obstacle_speed: f32,
}

impl Default for Painting {
    fn default() -> Self {
        Self {
            my_enum: Enum::Stat,
            menu: Menu::default(),
            enviroment: Enviroment::default(),
            srodowisko: Default::default(),
            kurs: Default::default(),
            stroke: Stroke::new(1.0, Color32::from_rgb(255, 204, 229)),
            stroke_kurs: Stroke::new(1.0, Color32::from_rgb(0, 204, 229)),
        }
    }
}

impl Painting {
    pub fn ui_control(&mut self, ui: &mut egui::Ui) {
        ui.separator();
        ui.separator();
        if ui.button("zapisz do pliku").clicked() {
            self.enviroment.static_obstacles = self
                .srodowisko
                .iter()
                .map(|x| {
                    x.iter()
                        .map(|x| vec2(x.to_vec2().x * 1000., x.to_vec2().y * 1000.))
                        .collect()
                })
                .collect();

                self.enviroment.height=1000.0;
                self.enviroment.width=1000.0;
            self.enviroment
                .dynaic_obstacles
                .clone_from(&self.menu.obstacle);

            self.enviroment
                .dynaic_obstacles
                .iter_mut()
                .for_each(|x| x.position = vec2(x.position.x * 1000., x.position.y * 1000.));

            self.enviroment.dynaic_obstacles.iter_mut().for_each(|x| {
                x.safe_sphere
                    .iter_mut()
                    .for_each(|x| *x = vec2(x.x * 1000.0, x.y * 1000.))
            });

            self.enviroment.starting_point = vec2(
                self.menu.individual_start.x * 1000.,
                self.menu.individual_start.y * 1000.,
            );
            self.enviroment.ending_point = vec2(
                self.menu.individual_end.x * 1000.,
                self.menu.individual_end.y * 1000.,
            );

            let enviroment_file = File::options()
                .write(true)
                .create(true)
                .truncate(true)
                .open("env2.json")
                .unwrap();
            let writer = BufWriter::new(enviroment_file);
            serde_json::to_writer(writer, &self.enviroment).unwrap();
        }

        ui.separator();
        ui.label("wybierz punkt początkowy");
        if ui
            .add(egui::RadioButton::new(
                self.my_enum == Enum::Start,
                "Dodawanie punktu starowego",
            ))
            .clicked()
        {
            self.my_enum = Enum::Start
        }
        if self.my_enum == Enum::Start {
            ui.add(
                egui::Slider::new(&mut self.menu.individual_start.x, 0.0..=1.0).text("Pozycja X"),
            );
            ui.add(
                egui::Slider::new(&mut self.menu.individual_start.y, 0.0..=1.0).text("Pozycja Y"),
            );
            if ui.button("ustaw punkt początkowy").clicked() {
                self.enviroment.starting_point = self.menu.individual_start;
            }
        }
        ui.separator();
        ui.label("wybierz punkt koncowy");
        if ui
            .add(egui::RadioButton::new(
                self.my_enum == Enum::End,
                "Dodawanie punktu koncowego",
            ))
            .clicked()
        {
            self.my_enum = Enum::End
        }
        if self.my_enum == Enum::End {
            ui.add(egui::Slider::new(&mut self.menu.individual_end.x, 0.0..=1.0).text("Pozycja X"));
            ui.add(egui::Slider::new(&mut self.menu.individual_end.y, 0.0..=1.0).text("Pozycja Y"));
            if ui.button("wybierz punkt koncowy").clicked() {
                self.enviroment.ending_point = self.menu.individual_end;
            }
        }

        ui.separator();
        ui.label("obiekty dynamiczne");
        if ui
            .add(egui::RadioButton::new(
                self.my_enum == Enum::Dym,
                "Dodawanie obiektu dynamicznengo",
            ))
            .clicked()
        {
            self.my_enum = Enum::Dym;
        }

        if self.my_enum == Enum::Dym {
            ui.add(egui::Slider::new(&mut self.menu.obstacle_course, 0.0..=359.0).text("Kurs"));
            ui.add(egui::Slider::new(&mut self.menu.obstacle_speed, 0.0..=10.0).text("Predkosc"));
            ui.add(egui::Slider::new(&mut self.menu.obstacle_pos.x, 0.0..=1.0).text("Pozycja X"));
            ui.add(egui::Slider::new(&mut self.menu.obstacle_pos.y, 0.0..=1.0).text("Pozycja Y"));

            if ui.button("dodaj obiekt dynamiczny").clicked() {
                self.menu.obstacle.push(DynaicObstacle::new(
                    self.menu.obstacle_pos,
                    self.menu.obstacle_course,
                    self.menu.obstacle_speed,
                ));
                // self.enviroment
                //     .dynaic_obstacles
                //     .push(DynaicObstacle::new(vec2(0.5, 0.5), 0.0, 0.1));
                // match self.enviroment.dynaic_obstacles.first() {
                //     Some(x) => {
                //         self.srodowisko
                //             .push(x.safe_sphere.iter().map(|x| x.to_pos2()).collect());
                //     }
                //     None => {}
                // }
            }
        }

        ui.separator();
        ui.label("obiekty Statyczne");
        if ui
            .add(egui::RadioButton::new(
                self.my_enum == Enum::Stat,
                "Dodawanie obiektu Statycznego",
            ))
            .clicked()
        {
            self.my_enum = Enum::Stat;
        }
        if self.my_enum == Enum::Stat {
            ui.label(" LPM Lewy przycisk myszy aby zaznaczyczac kolejne punkty");
            ui.label(" RPM prawy przycisk myszy aby zakonczyc rysowanie figury");
        }
    }

    pub fn ui_content(&mut self, ui: &mut Ui) -> egui::Response {
        let (mut response, painter) =
            ui.allocate_painter(ui.available_size_before_wrap(), Sense::drag());

        let to_screen = emath::RectTransform::from_to(
            Rect::from_min_size(Pos2::ZERO, response.rect.square_proportions()),
            response.rect,
        );
        let from_screen = to_screen.inverse();
        if self.srodowisko.is_empty() {
            self.srodowisko.push(vec![]);
        }

        //inputy

        if self.my_enum == Enum::Start {
            if response.clicked_by(PointerButton::Primary) {
                if let Some(pointer_pos) = response.interact_pointer_pos() {
                    let canvas_pos = from_screen * pointer_pos;
                    self.menu.individual_start = canvas_pos.to_vec2();
                }
            }
        }
        if self.my_enum == Enum::End {
            if response.clicked_by(PointerButton::Primary) {
                if let Some(pointer_pos) = response.interact_pointer_pos() {
                    let canvas_pos = from_screen * pointer_pos;
                    self.menu.individual_end = canvas_pos.to_vec2();
                }
            }
        }
        if self.my_enum == Enum::Dym {
            if response.clicked_by(PointerButton::Primary) {
                if let Some(pointer_pos) = response.interact_pointer_pos() {
                    let canvas_pos = from_screen * pointer_pos;
                    self.menu.obstacle_pos = canvas_pos.to_vec2();
                }
            }
        }
        if self.my_enum == Enum::Stat {
            let current_line = self.srodowisko.last_mut().unwrap();
            if response.clicked_by(PointerButton::Primary) {
                if let Some(pointer_pos) = response.interact_pointer_pos() {
                    let canvas_pos = from_screen * pointer_pos;
                    if current_line.last() != Some(&canvas_pos) {
                        current_line.push(canvas_pos);
                        response.mark_changed();
                    }
                }
            }
            if response.clicked_by(PointerButton::Secondary) {
                match current_line.first() {
                    Some(x) => {
                        current_line.push(*x);
                        self.srodowisko.push(vec![]);
                    }
                    None => {}
                };
            }
        }

        //rysowaonie

        let mut shapes = vec![];

        shapes.push(egui::Shape::circle_filled(
            to_screen * self.menu.individual_start.to_pos2(),
            3.0,
            Color32::from_rgb(0, 255, 255),
        ));
        shapes.push(egui::Shape::circle_filled(
            to_screen * self.menu.individual_end.to_pos2(),
            3.0,
            Color32::from_rgb(255, 0, 255),
        ));
        shapes.push(egui::Shape::text(
            &ui.fonts(),
            to_screen
                * ((vec2(
                    self.menu.individual_start.x + 0.01,
                    self.menu.individual_start.y,
                ))
                .to_pos2()),
            Align2::LEFT_TOP,
            format!(
                "P. Poczatkowy\nX:{:.3} Y:{:.3}",
                self.menu.individual_start.x, self.menu.individual_start.y
            ),
            FontId::new(12., FontFamily::Monospace),
            Color32::from_rgb(0, 255, 255),
        ));

        shapes.push(egui::Shape::text(
            &ui.fonts(),
            to_screen
                * ((vec2(
                    self.menu.individual_end.x + 0.01,
                    self.menu.individual_end.y,
                ))
                .to_pos2()),
            Align2::LEFT_TOP,
            format!(
                "P. Koncowy\nX:{:.3} Y:{:.3}",
                self.menu.individual_end.x, self.menu.individual_end.y
            ),
            FontId::new(12., FontFamily::Monospace),
            Color32::from_rgb(255, 0, 255),
        ));

        for line in &self.srodowisko {
            if line.len() >= 1 {
                let points: Vec<Pos2> = line.iter().map(|p| to_screen * *p).collect();
                shapes.push(egui::Shape::line(points, self.stroke));
            }
        }
        for obstacle in &self.menu.obstacle {
            let mut object: Vec<Pos2> = obstacle.safe_sphere.iter().map(|p| p.to_pos2()).collect();
            object.push(object.first().unwrap().clone());
            let points: Vec<Pos2> = object.iter().map(|p| to_screen * *p).collect();
            shapes.push(egui::Shape::circle_filled(
                to_screen * obstacle.position.to_pos2(),
                1.0,
                Color32::from_rgb(255, 255, 255),
            ));
            shapes.push(egui::Shape::text(
                &ui.fonts(),
                to_screen * ((vec2(obstacle.position.x, obstacle.position.y + 0.02)).to_pos2()),
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

impl Painting {
    pub fn show(&mut self, ctx: &Context) {
        egui::CentralPanel::default().show(ctx, |ui| self.ui(ui));
        egui::SidePanel::right("Menu").show(ctx, |ui| {
            self.ui_control(ui);
        });
    }
}

impl Painting {
    fn ui(&mut self, ui: &mut Ui) {
        Frame::canvas(ui.style()).show(ui, |ui| {
            ui.set_max_width(ui.available_size().y);
            self.ui_content(ui);
        });
    }
}
