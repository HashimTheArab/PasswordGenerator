use eframe::{epi, egui};
use egui::{CtxRef, CentralPanel, TopBottomPanel, Slider};
use epi::{Frame};
use settings::Settings;
use crate::settings;
use rand::Rng;
use winapi::um::winuser::{CF_TEXT, OpenClipboard, EmptyClipboard, SetClipboardData, CloseClipboard};
use winapi::um::winbase::{GlobalAlloc, GMEM_MOVEABLE, GlobalUnlock, GlobalLock};
use std::ptr::copy_nonoverlapping;

pub struct App {
    settings: Settings,
    password: String
}

impl App {
    pub fn default() -> Self {
        Self {
            settings: Settings::default(),
            password: String::new()
        }
    }

    pub fn generate(&self) -> String {
        let mut pass = String::new();
        if self.settings.chars.len() > 0 {
            let mut rng = rand::thread_rng();
            let bytes = self.settings.chars.as_bytes();
            for _ in 0..=self.settings.length {
                pass.push(bytes[rng.gen_range(0..self.settings.chars.len())] as char);
            }
        }
        pass
    }
}

impl epi::App for App {
    fn update(&mut self, ctx: &CtxRef, _frame: &mut Frame<'_>) {
        CentralPanel::default().show(&ctx, |ui| {
            ui.heading("Password Generator");
            ui.horizontal(|ui| {
                ui.label("Password Length");
                ui.add(Slider::new(&mut self.settings.length,0..=50));
            });
            let mut chars_to_push = String::new();
            let mut chars_to_remove: Vec<String> = Vec::new();
            for entry in self.settings.entries.iter_mut() {
                ui.horizontal(|ui| {
                    ui.label(&entry.name);
                    if ui.checkbox(&mut entry.toggled, format!("( {} )", entry.description)).changed() {
                        if entry.toggled {
                           chars_to_push += entry.added_chars.as_str().clone();
                        } else {
                            chars_to_remove.push(entry.added_chars.clone());
                        }
                    }
                });
            }
            self.settings.chars += chars_to_push.as_str().clone();
            for chars in chars_to_remove {
                if self.settings.chars.contains(&chars) {
                    self.settings.chars = self.settings.chars.replace(&chars, "");
                }
            }
            ui.horizontal(|ui| {
                ui.label(format!("Password: {}", self.password));
                if ui.button("Copy").clicked() {
                    unsafe {
                        let h_mem = GlobalAlloc(GMEM_MOVEABLE, self.password.len() + 1);
                        copy_nonoverlapping(self.password.as_ptr(), GlobalLock(h_mem) as _, self.password.len());
                        GlobalUnlock(h_mem);
                        OpenClipboard(0 as _);
                        EmptyClipboard();
                        SetClipboardData(CF_TEXT, h_mem);
                        CloseClipboard();
                    }
                }
                if ui.button("Generate").clicked() {
                    self.password = self.generate();
                }
            });
        });
    }

    fn name(&self) -> &str {
        "Password Generator"
    }

}