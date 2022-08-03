use egui::*;

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(default))]
pub struct Painting {
    /// in 0-1 normalized coordinates
    pub srodowisko: Vec<Vec<Pos2>>,
    pub kurs: Vec<Vec<Pos2>>,
    stroke: Stroke,
    stroke_kurs: Stroke,
}

impl Default for Painting {
    fn default() -> Self {
        Self {
            srodowisko: Default::default(),
            kurs: Default::default(),
            stroke: Stroke::new(1.0, Color32::from_rgb(255, 204, 229)),
            stroke_kurs: Stroke::new(1.0, Color32::from_rgb(0, 204, 229)),
        }
    }
}

impl Painting {
    pub fn ui_control(&mut self, ui: &mut egui::Ui) -> egui::Response {
        ui.horizontal(|ui| {
            egui::stroke_ui(ui, &mut self.stroke, "Stroke");
            ui.separator();
            if ui.button("Do zrobienia").clicked() {
                self.kurs.push(vec![]);
                self.kurs[0].push(egui::pos2(0.0, 0.0));
                self.kurs[0].push(egui::pos2(0.5, 0.5));
                self.kurs[0].push(egui::pos2(1.0, 1.0));
                self.kurs[0].push(egui::pos2(1.0, 0.0));
                self.kurs[0].push(egui::pos2(0.0, 1.0));
                self.kurs[0].push(egui::pos2(0.0, 0.0));
                self.kurs[0].push(egui::pos2(0.0, 1.0));


            }
        })
        .response
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

        let mut shapes = vec![];
        for line in &self.srodowisko {
            if line.len() >= 1 {
                let points: Vec<Pos2> = line.iter().map(|p| to_screen * *p).collect();
                shapes.push(egui::Shape::line(points, self.stroke));
            }
        }

        for line in &self.kurs {
            if line.len() >= 1 {
                let points: Vec<Pos2> = line.iter().map(|p| to_screen * *p).collect();
                shapes.push(egui::Shape::line(points, self.stroke_kurs));
            }
        }

        painter.extend(shapes);
        response
    }
}

impl Painting {
    pub fn show(&mut self, ctx: &Context) {
        egui::CentralPanel::default().show(ctx, |ui| self.ui(ui));
    }
}

impl Painting {
    fn ui(&mut self, ui: &mut Ui) {
        self.ui_control(ui);
        ui.label("Wyznacz punkty LPM, przejdz do następnego PPM");
        
        Frame::canvas(ui.style()).show(ui, |ui| {
          //  ui.set_min_size(vec2(100.0, 100.0));
            
           // ui.allocate_space(vec2(ui.available_height(),ui.available_height()));
            
         //   ui.set_max_size(vec2(ui.available_height(),ui.available_height()));

            self.ui_content(ui);
        });
    }
}
