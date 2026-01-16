use eframe::egui;
use std::collections::HashMap;
use std::fs;
use crate::state::GLOBAL_STATE;
use crate::actions::GameAction;
use crate::win_api;

pub struct DofusApp {
    status_msg: String,
    rebinding_index: Option<usize>,
    icon_textures: HashMap<String, egui::TextureHandle>,
    available_icon_names: Vec<String>,
    drag_texture: Option<egui::TextureHandle>,
    launch_texture: Option<egui::TextureHandle>,
    restore_texture: Option<egui::TextureHandle>,
    rotate_texture: Option<egui::TextureHandle>,
    delete_texture: Option<egui::TextureHandle>,
    mini_mode: bool,
    vertical_mini: bool,
}

impl Default for DofusApp {
    fn default() -> Self {
        Self {
            status_msg: "Listo.".to_owned(),
            rebinding_index: None,
            icon_textures: HashMap::new(),
            available_icon_names: Vec::new(),
            drag_texture: None,
            launch_texture: None,
            restore_texture: None,
            rotate_texture: None,
            delete_texture: None,
            mini_mode: false,
            vertical_mini: true,
        }
    }
}

impl DofusApp {
    //Carga de iconos
    fn load_icons(&mut self, ctx: &egui::Context) {
        self.icon_textures.clear();
        self.available_icon_names.clear();
        self.drag_texture = None;
        self.launch_texture = None;
        self.restore_texture = None;
        
        if let Ok(entries) = fs::read_dir("./icons") {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_file() {
                    if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                        let ext = ext.to_lowercase();
                        if ext == "png" || ext == "jpg" || ext == "jpeg" {
                            let filename = path.file_stem().unwrap().to_string_lossy().to_string();
                            
                            if let Ok(image_buffer) = image::open(&path) {
                                let image_buffer = image_buffer.to_rgba8();
                                let size = [image_buffer.width() as _, image_buffer.height() as _];
                                let pixels = image_buffer.as_flat_samples();
                                let color_image = egui::ColorImage::from_rgba_unmultiplied(size, pixels.as_slice());
                                let texture = ctx.load_texture(&filename, color_image, egui::TextureOptions::LINEAR);
                                //estos iconos no salen para asignar al pj
                                match filename.as_str() {
                                    "drag" => self.drag_texture = Some(texture),
                                    "launch" => self.launch_texture = Some(texture),
                                    "restore" => self.restore_texture = Some(texture),
                                    "rotate" => self.rotate_texture = Some(texture),
                                    "delete" => self.delete_texture = Some(texture),
                                    _ => {
                                        
                                        self.icon_textures.insert(filename.clone(), texture);
                                        self.available_icon_names.push(filename);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        self.available_icon_names.sort();
    }

    //Estilos
    fn configure_style(&self, ctx: &egui::Context) {
        let mut style = (*ctx.style()).clone();

        style.spacing.item_spacing = egui::vec2(8.0, 8.0);
        style.visuals.popup_shadow.color = egui::Color32::from_black_alpha(150);
        style.visuals.widgets.noninteractive.bg_fill = egui::Color32::from_rgb(30, 32, 40);

        if self.mini_mode {
            style.visuals.window_fill = egui::Color32::TRANSPARENT;
            style.visuals.panel_fill = egui::Color32::TRANSPARENT;
            style.visuals.window_shadow = egui::epaint::Shadow::NONE; 
            style.visuals.window_stroke = egui::Stroke::NONE;     
        } else {
            style.visuals.window_fill = egui::Color32::from_rgb(20, 22, 28); 
            style.visuals.panel_fill = egui::Color32::from_rgb(20, 22, 28);
        }
        
        ctx.set_style(style);
    }

    fn calculate_mini_size(&self, count: usize, is_vertical: bool) -> egui::Vec2 {        
        let icon_size = 28.0;
        let spacing = 4.0;
        let icons_space = (count as f32) * (icon_size + spacing);         
        
        let extras = 90.0; 

        if is_vertical {
            egui::vec2(60.0, icons_space + extras) 
        } else {
            egui::vec2(icons_space + extras, 52.0) 
        }
    }

    
    //mini modo
    fn render_mini_mode(&mut self, ctx: &egui::Context) {
        
        let count = GLOBAL_STATE.lock().unwrap().windows.len().max(1);
        let target_size = self.calculate_mini_size(count, self.vertical_mini);
        ctx.send_viewport_cmd(egui::ViewportCommand::InnerSize(target_size));

        // Estilos del panel
        let panel_frame = egui::Frame::none()
            .fill(egui::Color32::from_rgb(35, 37, 45)) 
            .rounding(egui::Rounding::same(20.0))     
            .inner_margin(egui::Margin::symmetric(10.0, 5.0))
            .outer_margin(egui::Margin::same(5.0)) 
            .stroke(egui::Stroke::new(1.0, egui::Color32::from_rgb(60, 65, 75)));

        egui::CentralPanel::default().frame(panel_frame).show(ctx, |ui| {
            
            if ui.input(|i| i.pointer.button_down(egui::PointerButton::Primary)) {
                ctx.send_viewport_cmd(egui::ViewportCommand::StartDrag);
            }

            let is_vertical = self.vertical_mini;

            let content = |ui: &mut egui::Ui| {
                ui.spacing_mut().item_spacing = egui::vec2(4.0, 4.0);

                //volver ventana principal
                let restore_btn = if let Some(tex) = &self.restore_texture {
                    let img = egui::Image::new(tex).fit_to_exact_size(egui::vec2(20.0, 20.0));
                    ui.add(egui::ImageButton::new(img).frame(false))
                } else {
                    ui.add(egui::Button::new("🗖").frame(false))
                };

                if restore_btn.on_hover_text("Volver").clicked() {
                    self.mini_mode = false;
                    ctx.send_viewport_cmd(egui::ViewportCommand::InnerSize(egui::vec2(450.0, 600.0)));
                    ctx.send_viewport_cmd(egui::ViewportCommand::Decorations(true));
                    ctx.send_viewport_cmd(egui::ViewportCommand::WindowLevel(egui::WindowLevel::Normal));
                    ctx.send_viewport_cmd(egui::ViewportCommand::Transparent(false));
                }

                ui.separator();

                //icnos
                let state = GLOBAL_STATE.lock().unwrap();
                let active_hwnd = win_api::get_foreground_window();

                for win in state.windows.iter() {
                    let tex = self.icon_textures.get(&win.icon).or_else(|| self.icon_textures.get("default"));
                    if let Some(texture) = tex {
                        let is_active = win.hwnd.0 == active_hwnd;
                        let btn_size = egui::vec2(28.0, 28.0);
                        let img = egui::Image::new(texture).fit_to_exact_size(btn_size);
                        let btn = egui::ImageButton::new(img).frame(false);
                        let resp = ui.add(btn);
                        
                        if is_active {
                            ui.painter().rect_stroke(resp.rect.expand(2.0), 50.0, egui::Stroke::new(1.5, egui::Color32::from_rgb(255, 204, 0)));
                        }
                        if resp.clicked() { win_api::switch_to_window(win.hwnd.0); }
                        resp.on_hover_text(&win.name);
                    }
                }

                 ui.separator();

                //rotar
                let rotate_resp = if let Some(tex) = &self.rotate_texture {
                    let img = egui::Image::new(tex).fit_to_exact_size(egui::vec2(16.0, 16.0));
                    ui.add(egui::ImageButton::new(img).frame(false))
                } else {
                    ui.add(egui::Button::new(egui::RichText::new("🔄").size(13.0)).frame(false))
                };

                if rotate_resp.hovered() {
                    ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
                    ui.painter().rect_filled(rotate_resp.rect.expand(2.0), 3.0, egui::Color32::from_white_alpha(30));
                }

                if rotate_resp.on_hover_text("Rotar").clicked() {
                    self.vertical_mini = !self.vertical_mini;
                    
                }
            };

            if is_vertical {
                ui.vertical_centered(content);
            } else {
                ui.horizontal_centered(content); 
            }
        });
        
        ctx.request_repaint_after(std::time::Duration::from_millis(100));
    }
    
}

impl eframe::App for DofusApp {

    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        if self.mini_mode {// Transparente en mini modo
            egui::Color32::TRANSPARENT.to_normalized_gamma_f32()
        } else {
            egui::Color32::from_rgb(20, 22, 28).to_normalized_gamma_f32()
        }
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.available_icon_names.is_empty() && !self.icon_textures.is_empty() {
        } else if self.available_icon_names.is_empty() {
             self.load_icons(ctx);
        }

        if self.rebinding_index.is_some() { ctx.request_repaint(); }

        if self.mini_mode {
            self.render_mini_mode(ctx);
        } 
        else
        {       
        self.configure_style(ctx);
        if self.rebinding_index.is_some() { ctx.request_repaint(); }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("by Exil");
            ui.separator();
            
                ui.horizontal(|ui| {                   

                    //Atajos
                    {
                        let state = GLOBAL_STATE.lock().unwrap();                        
                        ui.label(egui::RichText::new("Atajos:").strong().size(14.0));
                        
                        for (i, binding) in state.bindings.iter().enumerate() {
                            let label = match binding.action {
                                GameAction::NextWindow => "Sig:",
                                GameAction::PrevWindow => "Ant:",
                            };
                            
                            ui.label(egui::RichText::new(label).size(14.0));
                            
                            let btn_text = if self.rebinding_index == Some(i) { 
                                "...".to_string() 
                            } else { 
                                format!("[{}]", binding.name) 
                            };

                            if ui.add_enabled(self.rebinding_index.is_none(), egui::Button::new(btn_text).small()).clicked() {
                                self.rebinding_index = Some(i);
                                self.status_msg = format!("Configurando '{}'...", label);
                            }
                        }
                    }
                     
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        
                        //launch mini mode
                        let launch_resp = if let Some(tex) = &self.launch_texture {
                            let img = egui::Image::new(tex).fit_to_exact_size(egui::vec2(20.0, 20.0)); 
                            ui.add(egui::ImageButton::new(img).frame(false))
                        } else {
                            ui.button("Launch")
                        };

                        if launch_resp.hovered() {
                            ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);                            
                        }
                                               

                        if launch_resp.on_hover_text("Modo Mini").clicked() {
                            self.mini_mode = true;                            
                            ctx.send_viewport_cmd(egui::ViewportCommand::Decorations(false));
                            ctx.send_viewport_cmd(egui::ViewportCommand::WindowLevel(egui::WindowLevel::AlwaysOnTop));
                            ctx.send_viewport_cmd(egui::ViewportCommand::Transparent(true));
                        }

                        if ui.button(egui::RichText::new("🔄 Sincronizar").size(15.0)).clicked() {
                            win_api::scan_open_windows();
                            self.load_icons(ctx);
                            self.status_msg = "Sincronizado.".to_string();
                        } 
                    });
                });
                
                ui.add_space(5.0);
                let mut state = GLOBAL_STATE.lock().unwrap();
            
            ui.separator();

            //Lista
            let len = state.windows.len();
            if len == 0 {
                ui.centered_and_justified(|ui| {
                    ui.label("No se detectaron ventanas. Haz clic en 'Sincronizar' para buscar.");
                });
            } else {
                let mut source_idx = None;
                let mut target_idx = None;
                let mut delete_idx: Option<usize> = None;

                let mut pending_icon_save: Option<(String, String)> = None;

                egui::ScrollArea::vertical().show(ui, |ui| {
                        ui.set_width(ui.available_width());
                        
                        for i in 0..len {
                            let win = &mut state.windows[i];
                            let item_id = ui.make_persistent_id(win.hwnd.0);
                            let popup_id = item_id.with("popup");

                            egui::Frame::none()
                                .fill(egui::Color32::from_rgb(35, 37, 45))
                                .rounding(4.0)
                                .inner_margin(egui::Margin::symmetric(8.0, 4.0)) 
                                .show(ui, |ui| {
                                    let row_height = 36.0; 
                                    ui.set_height(row_height);
                                    
                                    ui.horizontal(|ui| {
                                        //drag handle
                                        ui.allocate_ui(egui::vec2(30.0, row_height), |ui| {
                                            ui.centered_and_justified(|ui| {
                                                let drag_resp = ui.dnd_drag_source(item_id, i, |ui| {
                                                    if let Some(tex) = &self.drag_texture {
                                                        let img = egui::Image::new(tex).fit_to_exact_size(egui::vec2(22.0, 22.0));
                                                        let btn = ui.add(egui::ImageButton::new(img).frame(false));
                                                        btn.on_hover_cursor(egui::CursorIcon::Grab);
                                                    } else {
                                                        let handle = ui.add(egui::Button::new("☩").frame(false));
                                                        handle.on_hover_cursor(egui::CursorIcon::Grab);
                                                    }
                                                });
                                                if let Some(payload) = drag_resp.response.dnd_hover_payload::<usize>() {
                                                    source_idx = Some(*payload);
                                                    target_idx = Some(i);
                                                }
                                            });
                                        });

                                        ui.add_space(50.0);

                                        let actions_width = 110.0;
                                        let available_width = ui.available_width() - actions_width;                                        
                                        let col_width = (available_width / 2.0) - 10.0; 

                                        //nombre pj
                                        ui.allocate_ui_with_layout(
                                            egui::vec2(col_width, row_height),
                                            egui::Layout::left_to_right(egui::Align::Center), 
                                            |ui| {
                                                ui.set_min_width(col_width);
                                                ui.add(egui::Label::new(
                                                    egui::RichText::new(&win.name).size(14.0).strong().color(egui::Color32::WHITE)
                                                ).truncate());
                                            }
                                        );

                                        //Clase pj
                                        ui.allocate_ui_with_layout(
                                            egui::vec2(col_width, row_height),
                                            egui::Layout::left_to_right(egui::Align::Center), 
                                            |ui| {
                                                ui.set_min_width(col_width);
                                                ui.add(egui::Label::new(
                                                    egui::RichText::new(&win.class).size(13.0).color(egui::Color32::from_rgb(200, 200, 200))
                                                ).truncate());
                                            }
                                        );

                                        //Acciones
                                        ui.allocate_ui_with_layout(
                                            egui::vec2(actions_width, row_height),
                                            egui::Layout::right_to_left(egui::Align::Center), 
                                            |ui| {

                                                //eliminar fila
                                                let del_btn = if let Some(tex) = &self.delete_texture {
                                                    let img = egui::Image::new(tex).fit_to_exact_size(egui::vec2(18.0, 18.0));
                                                    ui.add(egui::ImageButton::new(img).frame(false))
                                                } else {
                                                    ui.add(egui::Button::new("🗑").frame(false))
                                                };
                                                
                                                if del_btn.on_hover_text("Ocultar ventana").clicked() {
                                                    delete_idx = Some(i);
                                                }
                                                ui.add_space(20.0);

                                                //icono
                                                let tex = self.icon_textures.get(&win.icon).or_else(|| self.icon_textures.get("default"));
                                                let btn_size = egui::vec2(30.0, 30.0);
                                                
                                                let btn_resp = if let Some(t) = tex {
                                                    let img = egui::Image::new(t).fit_to_exact_size(btn_size);
                                                    ui.add(egui::ImageButton::new(img).frame(false))
                                                } else {
                                                    ui.add_sized(btn_size, egui::Button::new("📷").small())
                                                };
                                                
                                                if btn_resp.hovered() { ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand); }
                                                if btn_resp.clicked() { ui.memory_mut(|mem| mem.toggle_popup(popup_id)); }
                                                
                                                //popup selección icono
                                                egui::popup::popup_below_widget(ui, popup_id, &btn_resp, egui::PopupCloseBehavior::CloseOnClick, |ui| {
                                                    egui::ScrollArea::vertical().max_height(250.0).show(ui, |ui| {
                                                        egui::Grid::new(format!("grid_{}", i)).spacing(egui::vec2(6.0, 6.0)).show(ui, |ui| {
                                                            let mut col_count = 0;
                                                            for icon_name in &self.available_icon_names {
                                                                if let Some(t) = self.icon_textures.get(icon_name) {
                                                                    let img_gal = egui::Image::new(t).fit_to_exact_size(egui::vec2(36.0, 36.0));
                                                                    if ui.add(egui::ImageButton::new(img_gal).frame(false)).clicked() {
                                                                        win.icon = icon_name.to_string();
                                                                        pending_icon_save = Some((win.name.clone(), icon_name.to_string()));
                                                                        ui.memory_mut(|m| m.close_popup());
                                                                    }
                                                                }
                                                                col_count += 1;
                                                                if col_count >= 5 { ui.end_row(); col_count = 0; }
                                                            }
                                                        });
                                                    });
                                                });                                                
                                                 

                                                
                                            }
                                        );
                                    });
                                });
                            ui.add_space(3.0);
                        }
                    });

                    //'state' para guardar
                    if let Some((char_name, icon_name)) = pending_icon_save {
                        state.config.character_icons.insert(char_name, icon_name);
                        state.save_config();
                        self.status_msg = "Icono guardado.".to_string();
                    }

                    //cambios drag and drop
                    if let (Some(s), Some(t)) = (source_idx, target_idx) {
                        if s != t { let item = state.windows.remove(s); state.windows.insert(t, item); }
                    }

                    // Eliminar ventana
                    if let Some(idx) = delete_idx {
                        state.windows.remove(idx);
                        self.status_msg = "Ventana eliminada.".to_string();
                    }
            }

            ui.with_layout(egui::Layout::bottom_up(egui::Align::Min), |ui| {
                ui.separator();
                ui.label(egui::RichText::new(&self.status_msg).weak().small());
            });
        });

        if let Some(idx) = self.rebinding_index {
            if let Some(new_key) = win_api::detect_keypress() {
                let mut state = GLOBAL_STATE.lock().unwrap();
                state.bindings[idx].vk_code = new_key;
                state.bindings[idx].name = win_api::get_key_name(new_key);
                state.keys_need_update = true;
                self.status_msg = format!("Tecla actualizada: {}", state.bindings[idx].name);
                self.rebinding_index = None;
            }
        }
    }
    }
}