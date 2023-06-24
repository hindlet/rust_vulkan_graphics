use std::{ops::RangeInclusive, fmt::Debug};
use egui::{self, Color32, Context, FontFamily, FontDefinitions, FontData, epaint::Shadow, Stroke, CentralPanel, ScrollArea, Checkbox, Slider, Label};
use egui_winit_vulkano::{Gui, GuiConfig};
use vulkano_util::window::VulkanoWindows;
use winit::{window::WindowId, event_loop::EventLoop};


pub struct GuiWindowData {
    pub title: String,
    pub checkboxes: Vec<(String, bool)>,
    pub f32_sliders: Vec<(String, f32, RangeInclusive<f32>)>,
    pub i32_sliders: Vec<(String, i32, RangeInclusive<i32>)>,
    pub gui: Gui
}

impl Debug for GuiWindowData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Gui Window")
            .field("title", &self.title)
            .field("checkboxes", &self.checkboxes)
            .field("float sliders", &self.f32_sliders)
            .field("int sliders", &self.i32_sliders)
            .finish()
    }
}


/// this just sets the gui style to my preffered
fn set_gui_style(
    ctx: &Context
) {
    let mut style: egui::Style = (*ctx.style()).clone();

    style.visuals.override_text_color = Some(Color32::from_rgb(250, 250, 250));

    style.visuals.widgets.inactive.bg_stroke = Stroke {
        width: 0.5,
        color: Color32::from_rgb(0, 0, 0)
    };

    style.visuals.button_frame = true;

    style.visuals.collapsing_header_frame = true;

    style.visuals.window_shadow = Shadow::NONE;

    style.visuals.window_fill = Color32::from_rgb(150, 150, 150);
    

    ctx.set_style(style);

    let font_droidsansmono = include_bytes!("../assets/DroidSansMono.ttf");
    let mut font = FontDefinitions::default();

    font.font_data.insert(
        "Droid Sans Mono".to_string(),
        FontData::from_static(font_droidsansmono),
    );

    font.families
        .insert(FontFamily::Proportional, vec!["Droid Sans Mono".to_string()]);

    ctx.set_fonts(font); 
}

pub fn create_gui_window(
    title: String,
    checkboxes: Vec<(String, bool)>,
    f32_sliders: Vec<(String, f32, RangeInclusive<f32>)>,
    i32_sliders: Vec<(String, i32, RangeInclusive<i32>)>,

    windows: &mut VulkanoWindows,
    window_id: WindowId,
    event_loop: &EventLoop<()>,
) -> GuiWindowData {
    let renderer = windows.get_renderer_mut(window_id).unwrap();
    let gui = Gui::new(event_loop, renderer.surface(), renderer.graphics_queue(), GuiConfig::default());
    set_gui_style(&gui.context());
    GuiWindowData {
        title,
        checkboxes,
        f32_sliders,
        i32_sliders,
        gui
    }
}


pub fn draw_gui_window(
    data: &mut GuiWindowData
) {
    let gui = &mut data.gui;
    gui.immediate_ui(|gui| {
        let ctx = gui.context();
        CentralPanel::default().show(&ctx, |ui| {
            ui.vertical_centered(|ui| {
                sized_text(ui, &data.title, 32.0);
            });
            ui.separator();
            ScrollArea::vertical().id_source("settings").show(
                ui,
                |ui| {
                    ui.vertical_centered(|ui| {
                        sized_text(ui, "Checkboxes", 16.0);
                    });
                    for checkbox_data in data.checkboxes.iter_mut() {
                        ui.add(Checkbox::new(&mut checkbox_data.1, &checkbox_data.0));
                    }
                    ui.vertical_centered(|ui| {
                        sized_text(ui, "Sliders", 16.0);
                    });
                    for slider_data in data.f32_sliders.iter_mut() {
                        ui.add(Label::new(slider_data.0.clone()));
                        ui.add(Slider::new(&mut slider_data.1, slider_data.2.clone()));
                    }
                    for slider_data in data.i32_sliders.iter_mut() {
                        ui.add(Label::new(slider_data.0.clone()));
                        ui.add(Slider::new(&mut slider_data.1, slider_data.2.clone()));
                    }
                }
            )
        });
    });
}

fn sized_text(ui: &mut egui::Ui, text: impl Into<String>, size: f32) {
    ui.label(egui::RichText::new(text).size(size));
}
