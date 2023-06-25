use std::{ops::RangeInclusive, fmt::Debug};
use egui::{self, Color32, Context, FontFamily, FontDefinitions, FontData, epaint::Shadow, Stroke, CentralPanel, ScrollArea, Checkbox, Slider, Label, TextEdit, DragValue};
use egui_winit_vulkano::{Gui, GuiConfig};
use vulkano::sync::GpuFuture;
use vulkano_util::{window::VulkanoWindows, renderer::VulkanoWindowRenderer};
use winit::{window::WindowId, event_loop::EventLoop, event::WindowEvent};


pub struct GuiWindowData {
    pub title: String,
    pub checkboxes: Vec<(String, bool)>,
    pub f32_sliders: Vec<(String, f32, RangeInclusive<f32>)>,
    pub i32_sliders: Vec<(String, i32, RangeInclusive<i32>)>,
    pub f32_boxes: Vec<(String, f32)>,
    pub i32_boxes: Vec<(String, i32)>,
    pub u32_boxes: Vec<(String, u32)>,
    pub string_boxes: Vec<(String, String)>,

    has_checkboxes: bool,
    has_sliders: bool,
    has_boxes: bool,

    gui: Gui
}

impl Debug for GuiWindowData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f,
            "Gui Window - {}: \n- Checkboxes: {:?} \n- Float sliders: {:?} \n- Int Sliders: {:?} \n- Float Boxes: {:?} \n- Int Boxes: {:?} \n- Uint Boxes: {:?} \n- String Boxes: {:?}",
            self.title, self.checkboxes, self.f32_sliders, self.i32_sliders, self.f32_boxes, self.i32_boxes, self.u32_boxes, self.string_boxes
        )
        // f.debug_struct("Gui Window")
        //     .field("title", &self.title)
        //     .field("checkboxes", &self.checkboxes)
        //     .field("float sliders", &self.f32_sliders)
        //     .field("int sliders", &self.i32_sliders)
        //     .field("float boxes", &self.f32_boxes)
        //     .field("int boxes", &self.i32_boxes)
        //     .field("uint boxes", &self.u32_boxes)
        //     .field("string boxes", &self.string_boxes)
        //     .finish()
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
    f32_boxes: Vec<(String, f32)>,
    i32_boxes: Vec<(String, i32)>,
    u32_boxes: Vec<(String, u32)>,
    string_boxes: Vec<(String, String)>,

    windows: &mut VulkanoWindows,
    window_id: WindowId,
    event_loop: &EventLoop<()>,
) -> GuiWindowData {
    let renderer = windows.get_renderer_mut(window_id).unwrap();
    let gui = Gui::new(event_loop, renderer.surface(), renderer.graphics_queue(), GuiConfig::default());
    set_gui_style(&gui.context());
    let has_checkboxes = !(checkboxes.len() == 0);
    let has_sliders = !(f32_sliders.len() == 0 && i32_sliders.len() == 0);
    let has_boxes = !(f32_boxes.len() == 0 && i32_boxes.len() == 0 && u32_boxes.len() == 0 && string_boxes.len() == 0);

    GuiWindowData {
        title,
        checkboxes,
        f32_sliders,
        i32_sliders,
        f32_boxes,
        i32_boxes,
        u32_boxes,
        string_boxes,

        has_checkboxes,
        has_sliders,
        has_boxes,

        gui
    }
}

pub fn update_gui_window(
    window: &mut GuiWindowData,
    event: &WindowEvent
) -> bool {
    window.gui.update(event)
}

pub fn draw_gui_on_image(
    window: &mut GuiWindowData,
    before_future: Box<dyn GpuFuture>,
    renderer: &mut VulkanoWindowRenderer,
) -> Box<dyn GpuFuture>{
    window.gui.draw_on_image(before_future, renderer.swapchain_image_view())
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
                    if data.has_checkboxes {
                        ui.vertical_centered(|ui| {
                            sized_text(ui, "Checkboxes", 16.0);
                        });
                        for checkbox_data in data.checkboxes.iter_mut() {
                            ui.add(Checkbox::new(&mut checkbox_data.1, &checkbox_data.0));
                        }
                        ui.separator();
                    }
                    if data.has_sliders {
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
                        ui.separator();
                    }
                    if data.has_boxes {
                        ui.vertical_centered(|ui| {
                            sized_text(ui, "Data Boxes", 16.0);
                        });
                        for f32_data in data.f32_boxes.iter_mut() {
                            ui.add(Label::new(f32_data.0.clone()));
                            ui.add(DragValue::new(&mut f32_data.1));
                        }
                        for i32_data in data.i32_boxes.iter_mut() {
                            ui.add(Label::new(i32_data.0.clone()));
                            ui.add(DragValue::new(&mut i32_data.1));
                        }
                        for u32_data in data.u32_boxes.iter_mut() {
                            ui.add(Label::new(u32_data.0.clone()));
                            ui.add(DragValue::new(&mut u32_data.1));
                        }
                        for string_data in data.string_boxes.iter_mut() {
                            ui.add(Label::new(string_data.0.clone()));
                            ui.add(TextEdit::singleline(&mut string_data.1));
                        }
                    }
                }
            )
        });
    });
}

fn sized_text(ui: &mut egui::Ui, text: impl Into<String>, size: f32) {
    ui.label(egui::RichText::new(text).size(size));
}
