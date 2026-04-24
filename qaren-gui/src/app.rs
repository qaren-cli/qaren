//! Core application struct and eframe::App implementation.
//!
//! `QarenApp` holds the entire GUI state and implements the `eframe::App`
//! trait to render the UI each frame. All qaren-core interactions happen
//! through this module.

use std::path::PathBuf;

use eframe::App;
use egui;

use qaren_core::{ConfigFile, DiffResult, LiteralDiffResult};

use crate::settings::AppSettings;
use crate::theme;

/// Main application state.
pub struct QarenApp {
    /// Path to source (left) file
    pub file1_path: Option<PathBuf>,
    /// Path to target (right) file
    pub file2_path: Option<PathBuf>,
    /// Raw content of source file
    pub file1_content: Option<String>,
    /// Raw content of target file
    pub file2_content: Option<String>,

    /// Parsed source config
    pub config1: Option<ConfigFile>,
    /// Parsed target config
    pub config2: Option<ConfigFile>,

    /// Semantic diff result
    pub diff_result: Option<DiffResult>,
    /// Literal diff result
    pub literal_diff_result: Option<LiteralDiffResult>,

    /// Application settings (persisted)
    pub settings: AppSettings,

    /// Current error message to display
    pub error_message: Option<String>,
    /// Current success/info message
    pub info_message: Option<String>,
}

impl QarenApp {
    /// Create a new application instance, restoring persisted settings.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Restore settings from eframe storage if available
        let settings: AppSettings = cc
            .storage
            .and_then(|s| eframe::get_value(s, "qaren_settings"))
            .unwrap_or_default();

        // Apply theme on startup
        theme::apply_theme(&cc.egui_ctx, settings.dark_mode);

        Self {
            file1_path: None,
            file2_path: None,
            file1_content: None,
            file2_content: None,
            config1: None,
            config2: None,
            diff_result: None,
            literal_diff_result: None,
            settings,
            error_message: None,
            info_message: None,
        }
    }

    /// Load a file from disk, returning its content or setting an error.
    fn load_file(&mut self, path: PathBuf, is_file1: bool) {
        match std::fs::read_to_string(&path) {
            Ok(content) => {
                if is_file1 {
                    self.file1_content = Some(content);
                    self.file1_path = Some(path);
                } else {
                    self.file2_content = Some(content);
                    self.file2_path = Some(path);
                }
                self.error_message = None;
                // Clear previous results when files change
                self.diff_result = None;
                self.literal_diff_result = None;
                self.config1 = None;
                self.config2 = None;
            }
            Err(e) => {
                self.error_message = Some(format!("Failed to read {}: {}", path.display(), e));
            }
        }
    }

    /// Open a native file dialog and load the selected file.
    fn open_file_dialog(&mut self, is_file1: bool) {
        let dialog = rfd::FileDialog::new()
            .set_title(if is_file1 { "Open Source File" } else { "Open Target File" })
            .add_filter("Config files", &["env", "ini", "cfg", "conf", "properties", "yaml", "yml", "toml", "json"])
            .add_filter("All files", &["*"]);

        if let Some(path) = dialog.pick_file() {
            self.load_file(path, is_file1);
        }
    }

    /// Run the comparison using qaren-core.
    fn run_comparison(&mut self) {
        let (content1, content2) = match (&self.file1_content, &self.file2_content) {
            (Some(c1), Some(c2)) => (c1.clone(), c2.clone()),
            _ => {
                self.error_message = Some("Please load both files before comparing.".to_string());
                return;
            }
        };

        let delim1 = self.settings.resolve_delimiter(&content1);
        let delim2 = self.settings.resolve_delimiter(&content2);
        let path1 = self.file1_path.clone().unwrap_or_default();
        let path2 = self.file2_path.clone().unwrap_or_default();

        let opts1 = self.settings.to_parse_options(delim1);
        let opts2 = self.settings.to_parse_options(delim2);
        let diff_opts = self.settings.to_diff_options();

        // Parse both files
        match qaren_core::parse_content(&content1, &path1, &opts1) {
            Ok(cfg) => self.config1 = Some(cfg),
            Err(e) => {
                self.error_message = Some(format!("Parse error (source): {}", e));
                return;
            }
        }
        match qaren_core::parse_content(&content2, &path2, &opts2) {
            Ok(cfg) => self.config2 = Some(cfg),
            Err(e) => {
                self.error_message = Some(format!("Parse error (target): {}", e));
                return;
            }
        }

        // Run semantic diff
        if let (Some(cfg1), Some(cfg2)) = (&self.config1, &self.config2) {
            self.diff_result = Some(qaren_core::semantic_diff(cfg1, cfg2, &diff_opts));
        }

        // Run literal diff
        self.literal_diff_result = Some(qaren_core::literal_diff(&content1, &content2, &diff_opts));

        self.error_message = None;
        self.info_message = Some("Comparison complete.".to_string());
    }

    /// Render the top toolbar.
    fn render_toolbar(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 8.0;

            // File buttons
            if ui.button("📂 Source File").clicked() {
                self.open_file_dialog(true);
            }
            if let Some(ref path) = self.file1_path {
                let name = path.file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("unknown");
                ui.label(egui::RichText::new(name).color(theme::ACTION_BLUE).monospace());
            }

            ui.separator();

            if ui.button("📂 Target File").clicked() {
                self.open_file_dialog(false);
            }
            if let Some(ref path) = self.file2_path {
                let name = path.file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("unknown");
                ui.label(egui::RichText::new(name).color(theme::ACTION_BLUE).monospace());
            }

            ui.separator();

            // Compare button
            let both_loaded = self.file1_content.is_some() && self.file2_content.is_some();
            let compare_btn = egui::Button::new(
                egui::RichText::new("⚡ Compare").color(egui::Color32::WHITE),
            ).fill(if both_loaded { theme::ACTION_BLUE } else { egui::Color32::from_rgb(156, 163, 175) });

            if ui.add_enabled(both_loaded, compare_btn).clicked() {
                self.run_comparison();
            }

            ui.separator();

            // Mode switcher
            ui.label("Mode:");
            let mode = &mut self.settings.comparison_mode;
            ui.selectable_value(mode, crate::settings::ComparisonMode::SemanticKv, "Semantic KV");
            ui.selectable_value(mode, crate::settings::ComparisonMode::LiteralDiff, "Literal Diff");

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                // Dark mode toggle
                let mode_icon = if self.settings.dark_mode { "☀" } else { "🌙" };
                if ui.button(mode_icon).clicked() {
                    self.settings.dark_mode = !self.settings.dark_mode;
                    theme::apply_theme(ui.ctx(), self.settings.dark_mode);
                }

                // Secret toggle
                let eye_icon = if self.settings.show_secrets { "👁" } else { "👁‍🗨" };
                if ui.button(eye_icon).on_hover_text(
                    if self.settings.show_secrets { "Hide secrets" } else { "Show secrets" }
                ).clicked() {
                    self.settings.show_secrets = !self.settings.show_secrets;
                }
            });
        });
    }

    /// Render the main content area (empty state or diff results).
    fn render_content(&mut self, ui: &mut egui::Ui) {
        let colors = theme::ThemeColors::for_mode(self.settings.dark_mode);

        // Error banner
        if let Some(ref err) = self.error_message {
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new("⚠").color(theme::ERROR));
                ui.label(egui::RichText::new(err.as_str()).color(theme::ERROR));
            });
            ui.separator();
        }

        // Empty state — no files loaded yet
        if self.file1_content.is_none() && self.file2_content.is_none() {
            self.render_empty_state(ui, &colors);
            return;
        }

        // Show diff results if available
        if self.diff_result.is_some() || self.literal_diff_result.is_some() {
            self.render_diff_results(ui, &colors);
        } else {
            // Files loaded but not compared yet
            ui.vertical_centered(|ui| {
                ui.add_space(100.0);
                ui.label(
                    egui::RichText::new("Files loaded. Click ⚡ Compare to see differences.")
                        .size(16.0)
                        .color(colors.text_secondary),
                );
            });
        }
    }

    /// Render the empty/welcome state with drop zones.
    fn render_empty_state(&self, ui: &mut egui::Ui, colors: &theme::ThemeColors) {
        ui.vertical_centered(|ui| {
            ui.add_space(ui.available_height() / 4.0);

            ui.label(
                egui::RichText::new("قارن")
                    .size(48.0)
                    .color(theme::ACTION_BLUE)
                    .strong(),
            );
            ui.add_space(8.0);
            ui.label(
                egui::RichText::new("Configuration Comparator")
                    .size(20.0)
                    .color(colors.text_secondary),
            );
            ui.add_space(32.0);

            ui.label(
                egui::RichText::new("Drop two files here, or use the toolbar to open them.")
                    .size(14.0)
                    .color(colors.text_secondary),
            );
            ui.add_space(16.0);

            // Drop zone hint
            let frame = egui::Frame::none()
                .stroke(egui::Stroke::new(2.0, colors.border))
                .rounding(egui::Rounding::same(8.0))
                .inner_margin(egui::Margin::same(24.0));

            frame.show(ui, |ui| {
                ui.label(
                    egui::RichText::new("📁  Drag & drop .env, .ini, .yaml, or any config file")
                        .size(14.0)
                        .color(colors.text_secondary),
                );
            });
        });
    }

    /// Render the diff results in a split pane view.
    fn render_diff_results(&mut self, ui: &mut egui::Ui, colors: &theme::ThemeColors) {
        use crate::settings::ComparisonMode;

        match self.settings.comparison_mode {
            ComparisonMode::SemanticKv => self.render_semantic_diff(ui, colors),
            ComparisonMode::LiteralDiff => self.render_literal_diff(ui, colors),
        }
    }

    /// Render semantic KV diff results.
    fn render_semantic_diff(&self, ui: &mut egui::Ui, colors: &theme::ThemeColors) {
        let diff = match &self.diff_result {
            Some(d) => d,
            None => return,
        };

        // Summary bar
        self.render_summary_bar(ui, diff, colors);
        ui.separator();

        // Scrollable diff table
        egui::ScrollArea::vertical().show(ui, |ui| {
            egui::Grid::new("semantic_diff_grid")
                .num_columns(4)
                .spacing([8.0, 4.0])
                .striped(true)
                .min_col_width(80.0)
                .show(ui, |ui| {
                    // Header
                    ui.label(egui::RichText::new("Status").strong());
                    ui.label(egui::RichText::new("Key").strong().monospace());
                    ui.label(egui::RichText::new("Source Value").strong().monospace());
                    ui.label(egui::RichText::new("Target Value").strong().monospace());
                    ui.end_row();

                    let show = self.settings.show_secrets;

                    // Modified keys
                    for m in &diff.modified {
                        let v1 = crate::masking::mask_value(&m.key, &m.value_file1, show);
                        let v2 = crate::masking::mask_value(&m.key, &m.value_file2, show);

                        ui.label(egui::RichText::new("Modified").color(colors.modified_text).background_color(colors.modified_bg));
                        ui.label(egui::RichText::new(&m.key).monospace().color(colors.text));
                        ui.label(egui::RichText::new(v1).monospace().color(colors.modified_text).background_color(colors.modified_bg));
                        ui.label(egui::RichText::new(v2).monospace().color(colors.modified_text).background_color(colors.modified_bg));
                        ui.end_row();
                    }

                    // Missing in target (deleted)
                    for kv in &diff.missing_in_file2 {
                        let v = crate::masking::mask_value(&kv.key, &kv.value, show);
                        ui.label(egui::RichText::new("Only in Source").color(colors.deleted_text).background_color(colors.deleted_bg));
                        ui.label(egui::RichText::new(&kv.key).monospace().color(colors.text));
                        ui.label(egui::RichText::new(v).monospace().color(colors.deleted_text).background_color(colors.deleted_bg));
                        ui.label(egui::RichText::new("—").color(colors.text_secondary));
                        ui.end_row();
                    }

                    // Missing in source (added)
                    for kv in &diff.missing_in_file1 {
                        let v = crate::masking::mask_value(&kv.key, &kv.value, show);
                        ui.label(egui::RichText::new("Only in Target").color(colors.added_text).background_color(colors.added_bg));
                        ui.label(egui::RichText::new(&kv.key).monospace().color(colors.text));
                        ui.label(egui::RichText::new("—").color(colors.text_secondary));
                        ui.label(egui::RichText::new(v).monospace().color(colors.added_text).background_color(colors.added_bg));
                        ui.end_row();
                    }

                    // Identical keys
                    for key in &diff.identical {
                        let val = self.config1.as_ref()
                            .and_then(|c| c.pairs.get(key))
                            .map(|(v, _)| v.as_str())
                            .unwrap_or("");
                        let v = crate::masking::mask_value(key, val, show);
                        ui.label(egui::RichText::new("Identical").color(colors.identical_text));
                        ui.label(egui::RichText::new(key).monospace().color(colors.identical_text));
                        ui.label(egui::RichText::new(v).monospace().color(colors.identical_text));
                        ui.label(egui::RichText::new(v).monospace().color(colors.identical_text));
                        ui.end_row();
                    }
                });
        });
    }

    /// Render literal line-by-line diff.
    fn render_literal_diff(&self, ui: &mut egui::Ui, colors: &theme::ThemeColors) {
        let diff = match &self.literal_diff_result {
            Some(d) => d,
            None => return,
        };

        // Summary
        ui.horizontal(|ui| {
            ui.label(egui::RichText::new(format!("+ {} additions", diff.additions.len())).color(colors.added_text));
            ui.label(egui::RichText::new(format!("− {} deletions", diff.deletions.len())).color(colors.deleted_text));
        });
        ui.separator();

        egui::ScrollArea::vertical().show(ui, |ui| {
            // Deletions
            for line in &diff.deletions {
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new(format!("{:4}", line.line_number)).monospace().color(colors.text_secondary));
                    ui.label(egui::RichText::new(format!("- {}", line.content)).monospace().color(colors.deleted_text).background_color(colors.deleted_bg));
                });
            }
            // Additions
            for line in &diff.additions {
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new(format!("{:4}", line.line_number)).monospace().color(colors.text_secondary));
                    ui.label(egui::RichText::new(format!("+ {}", line.content)).monospace().color(colors.added_text).background_color(colors.added_bg));
                });
            }
        });
    }

    /// Render the summary bar showing diff counts.
    fn render_summary_bar(&self, ui: &mut egui::Ui, diff: &DiffResult, colors: &theme::ThemeColors) {
        ui.horizontal(|ui| {
            if diff.is_identical() {
                ui.label(egui::RichText::new("✓ Files are identical").color(theme::SUCCESS).strong());
            } else {
                ui.label(egui::RichText::new(format!("Modified: {}", diff.modified.len())).color(colors.modified_text).strong());
                ui.separator();
                ui.label(egui::RichText::new(format!("Only in Source: {}", diff.missing_in_file2.len())).color(colors.deleted_text).strong());
                ui.separator();
                ui.label(egui::RichText::new(format!("Only in Target: {}", diff.missing_in_file1.len())).color(colors.added_text).strong());
                ui.separator();
                ui.label(egui::RichText::new(format!("Identical: {}", diff.identical.len())).color(colors.identical_text));
            }
        });
    }

    /// Handle drag-and-drop file events.
    fn handle_dropped_files(&mut self, ctx: &egui::Context) {
        let dropped: Vec<egui::DroppedFile> = ctx.input(|i| i.raw.dropped_files.clone());
        for (idx, file) in dropped.iter().enumerate() {
            if let Some(ref path) = file.path {
                let is_file1 = if self.file1_content.is_none() {
                    true
                } else if self.file2_content.is_none() {
                    false
                } else {
                    idx == 0
                };
                self.load_file(path.clone(), is_file1);
            }
        }
    }
}

impl App for QarenApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Handle drag-and-drop
        self.handle_dropped_files(ctx);

        // Top toolbar
        egui::TopBottomPanel::top("toolbar").show(ctx, |ui| {
            ui.add_space(4.0);
            self.render_toolbar(ui);
            ui.add_space(4.0);
        });

        // Main content
        egui::CentralPanel::default().show(ctx, |ui| {
            self.render_content(ui);
        });
    }

    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, "qaren_settings", &self.settings);
    }
}
