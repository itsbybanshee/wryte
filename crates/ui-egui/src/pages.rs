//! The Wryte layout: one unified toolbar of icon-over-label buttons in place of the ribbon,
//! and a Format sidebar on the right (Style, Layout and More tabs) that formats the text at the
//! caret. Every control runs a command by id, like the ribbon. Icons are drawn here in code.

use egui::{Align2, Color32, CornerRadius, Rect, Sense, Stroke, Ui, pos2, vec2};
use serde_json::{Value, json};

use crate::theme::{Tokens, medium, regular, semibold};
use crate::widgets::{enabled, run_from_ribbon};
use crate::{WordApp, icons};

/// Height of the unified toolbar.
pub const TOOLBAR_H: f32 = 60.0;
/// Width of the Format sidebar.
pub const INSPECTOR_W: f32 = 264.0;

/// The paragraph styles the sidebar offers, by style id and shown name.
const PARA_STYLES: [(&str, &str); 9] = [
    ("Title", "Title"),
    ("Subtitle", "Subtitle"),
    ("Heading1", "Heading 1"),
    ("Heading2", "Heading 2"),
    ("Heading3", "Heading 3"),
    ("Normal", "Body"),
    ("Quote", "Quote"),
    ("IntenseQuote", "Intense Quote"),
    ("ListParagraph", "List Paragraph"),
];

/// The toolbar: View, Zoom, Add Page | Insert, Table, Chart, Text, Shape, Media, Comment | Share,
/// Format, Document.
pub fn toolbar(app: &mut WordApp, ui: &mut Ui) {
    let t = Tokens::get(ui.ctx());
    egui::Panel::top("pages_toolbar")
        .exact_size(TOOLBAR_H)
        // The canvas colour, with no line under it: the toolbar and the page area are one surface.
        .frame(egui::Frame::NONE.fill(t.canvas).inner_margin(egui::Margin { left: 4, right: 10, top: 0, bottom: 0 }))
        .show(ui, |ui| {
            // The toolbar is the title bar too: its background drags the window.
            crate::caption::drag_area(ui, ui.max_rect(), "pages_drag");
            // Windows: minimize, maximize and close at the right end, always enabled.
            if app.custom_caption {
                // Inset from the window's right edge as far as the Share button is from its left.
                let full = ui.max_rect();
                let right = ui.ctx().content_rect().max.x - crate::caption::INSET;
                let r = Rect::from_min_max(pos2(right - crate::caption::WIDTH, full.min.y), pos2(right, full.max.y));
                // A child that takes no room from the row of tools.
                let mut child = ui.new_child(egui::UiBuilder::new().max_rect(r).layout(egui::Layout::left_to_right(egui::Align::Center)));
                crate::caption::buttons(&mut child);
            }
            ui.horizontal_centered(|ui| {
                ui.spacing_mut().item_spacing = vec2(2.0, 0.0);
                if app.integrated_titlebar && !app.custom_caption {
                    ui.add_space((app.mac_titlebar.content_left - 10.0).max(0.0));
                }
                ui.add_enabled_ui(app.has_document(), |ui| {
                    ui.spacing_mut().item_spacing = vec2(2.0, 0.0);
                    if tool(ui, app, "share", "Share", false, false).clicked() {
                        let _ = app.run("ui.backstage", json!({"value": true, "page": "export"}));
                    }
                    let fmt_on = app.ui.pages_inspector && app.ui.pages_tab != "document";
                    if tool(ui, app, "pg.format", "Format", false, fmt_on).clicked() {
                        toggle_inspector(app, "style");
                    }
                    let doc_on = app.ui.pages_inspector && app.ui.pages_tab == "document";
                    if tool(ui, app, "pg.document", "Document", false, doc_on).clicked() {
                        toggle_inspector(app, "document");
                    }
                });
                ui.add_space(12.0);
                title_block(app, ui);
                ui.add_space(10.0);
                // The tools start where the page does (the page the caret is on, last frame).
                let page_left = app.canvas.page_rects.get(app.session.page_hint).or(app.canvas.page_rects.first()).map(|r| r.min.x);
                if let Some(x) = page_left.filter(|x| x.is_finite()) {
                    let gap = x - ui.cursor().min.x;
                    if gap > 0.0 {
                        ui.add_space(gap);
                    }
                }
                if !app.has_document() {
                    ui.disable();
                }
                let view = tool(ui, app, "pg.view", "View", true, false);
                egui::Popup::menu(&view).show(|ui| view_menu(ui, app));
                let pct = zoom_percent(app);
                let zoom = zoom_tool(ui, &format!("{pct}%"));
                egui::Popup::menu(&zoom).show(|ui| {
                    for z in [50, 75, 100, 125, 150, 200, 300, 400] {
                        if ui.selectable_label(pct == z, format!("{z}%")).clicked() {
                            run_from_ribbon(app, "view.zoom", json!({"value": z}));
                            ui.close();
                        }
                    }
                    ui.separator();
                    item(ui, app, "Fit Width", "view.pageWidth", json!({}));
                    item(ui, app, "Fit Page", "view.onePage", json!({}));
                });
                if tool(ui, app, "pg.addPage", "Add Page", false, false).clicked() {
                    run_from_ribbon(app, "insert.blankPage", json!({}));
                }
                ui.add_space(18.0);
                let insert = tool(ui, app, "pg.insert", "Insert", true, false);
                egui::Popup::menu(&insert).show(|ui| {
                    item(ui, app, "Page Break", "insert.pageBreak", json!({}));
                    item(ui, app, "Section Break", "layout.break", json!({"kind": "nextPage"}));
                    item(ui, app, "Column Break", "layout.break", json!({"kind": "column"}));
                    ui.separator();
                    item(ui, app, "Page Number", "insert.pageNumber", json!({"position": "current"}));
                    item(ui, app, "Date & Time", "insert.dateTime", json!({}));
                    item(ui, app, "Footnote", "references.footnote", json!({}));
                    item(ui, app, "Table of Contents", "references.toc", json!({}));
                    ui.separator();
                    item(ui, app, "Link…", "ui.dialog", json!({"name": "link"}));
                    item(ui, app, "Bookmark…", "ui.dialog", json!({"name": "bookmark"}));
                    item(ui, app, "Equation", "insert.equation", json!({}));
                    item(ui, app, "Symbol…", "insert.symbol", json!({}));
                });
                let table = tool(ui, app, "table", "Table", true, false);
                egui::Popup::menu(&table).show(|ui| {
                    crate::dialogs::table_grid_picker(ui, app);
                    ui.separator();
                    item(ui, app, "Insert Table…", "ui.dialog", json!({"name": "insertTable"}));
                });
                if tool(ui, app, "chart", "Chart", false, false).clicked() {
                    run_from_ribbon(app, "insert.chart", json!({}));
                }
                if tool(ui, app, "textBox", "Text", false, false).clicked() {
                    run_from_ribbon(app, "insert.textBox", json!({}));
                }
                let shape = tool(ui, app, "shapes", "Shape", true, false);
                egui::Popup::menu(&shape).show(|ui| {
                    for (l, k) in [
                        ("Rectangle", "rectangle"),
                        ("Rounded Rectangle", "roundedRectangle"),
                        ("Oval", "ellipse"),
                        ("Triangle", "triangle"),
                        ("Diamond", "diamond"),
                        ("Line", "line"),
                        ("Arrow", "arrow"),
                        ("Star", "star"),
                    ] {
                        item(ui, app, l, "insert.shape", json!({"kind": k}));
                    }
                });
                let media = tool(ui, app, "picture", "Media", true, false);
                egui::Popup::menu(&media).show(|ui| {
                    item(ui, app, "Photo…", "insert.picture", json!({}));
                    item(ui, app, "Icon…", "insert.icon", json!({}));
                });
                if tool(ui, app, "newComment", "Comment", false, false).clicked() {
                    run_from_ribbon(app, "review.newComment", json!({}));
                }
            });
        });
}

/// The document's name and whether it is saved, in two lines; a click opens the File page.
fn title_block(app: &mut WordApp, ui: &mut Ui) {
    let t = Tokens::get(ui.ctx());
    let name = if app.has_document() { app.title_stem() } else { "Wryte".to_string() };
    let state = if !app.has_document() {
        ""
    } else if app.session.dirty {
        tl!("Edited")
    } else if app.session.path.is_some() {
        tl!("Saved")
    } else {
        tl!("Not saved")
    };
    let width = 150.0;
    let (r, resp) = ui.allocate_exact_size(vec2(width, TOOLBAR_H - 12.0), Sense::click());
    if resp.hovered() {
        ui.painter().rect_filled(r, CornerRadius::same(8), t.hover);
    }
    let shown = crate::theme::fit_text(ui.painter(), &name, &semibold(13.0), 0.0, t.text, width - 16.0);
    ui.painter().text(pos2(r.min.x + 8.0, r.center().y - 7.0), Align2::LEFT_CENTER, shown, semibold(13.0), t.text);
    ui.painter().text(pos2(r.min.x + 8.0, r.center().y + 9.0), Align2::LEFT_CENTER, state, regular(11.0), t.text_dim);
    resp.widget_info(|| egui::WidgetInfo::labeled(egui::Role::Button, true, &name));
    if resp.on_hover_text(tl!("File")).clicked() {
        let _ = app.run("ui.backstage", json!({"value": true}));
    }
}

/// Format and Document share the sidebar: a click on the one showing closes it.
fn toggle_inspector(app: &mut WordApp, page: &str) {
    let showing = app.ui.pages_inspector && (app.ui.pages_tab == "document") == (page == "document");
    if showing {
        app.ui.pages_inspector = false;
    } else {
        app.ui.pages_inspector = true;
        app.ui.pages_tab = page.into();
    }
}

fn view_menu(ui: &mut Ui, app: &mut WordApp) {
    let v = app.session.view.clone();
    check(ui, app, "Show Page Thumbnails", v.nav_pane, "view.navigationPane", json!({"value": !v.nav_pane}));
    check(ui, app, "Show Rulers", v.ruler, "view.ruler", json!({"value": !v.ruler}));
    check(ui, app, "Show Invisibles", v.marks, "view.marks", json!({}));
    ui.separator();
    item(ui, app, "Two Pages", "view.multiplePages", json!({}));
    item(ui, app, "Focus", "view.focus", json!({}));
    item(ui, app, "Read Mode", "view.readMode", json!({}));
}

fn zoom_percent(app: &WordApp) -> i32 {
    let z = app.session.view.zoom;
    let z = if z.is_finite() { z } else { 1.0 };
    // The view keeps zoom as a factor; tolerate a percentage too.
    let pct = if z <= 10.0 { z * 100.0 } else { z };
    pct.round().clamp(1.0, 5000.0) as i32
}

/// A menu entry that runs a command, greyed out when the command can't run.
fn item(ui: &mut Ui, app: &mut WordApp, label: &str, id: &str, params: Value) {
    if ui.add_enabled(enabled(app, id), egui::Button::new(tl!(label))).clicked() {
        run_from_ribbon(app, id, params);
        ui.close();
    }
}

fn check(ui: &mut Ui, app: &mut WordApp, label: &str, on: bool, id: &str, params: Value) {
    let mut v = on;
    if ui.add_enabled(enabled(app, id), egui::Checkbox::new(&mut v, tl!(label))).clicked() {
        run_from_ribbon(app, id, params);
        ui.close();
    }
}

/// A toolbar button: an 18 pt icon over a small label, in a rounded hover well.
fn tool(ui: &mut Ui, app: &WordApp, icon: &str, label: &str, menu: bool, checked: bool) -> egui::Response {
    let t = Tokens::get(ui.ctx());
    let label = tl!(label);
    let lw = ui.ctx().fonts_mut(|f| f.layout_no_wrap(label.to_string(), regular(10.5), t.text).size().x);
    let w = (lw + 14.0).max(48.0);
    let (r, resp) = ui.allocate_exact_size(vec2(w, TOOLBAR_H - 8.0), Sense::click());
    let on = ui.is_enabled() && (!icon.starts_with("pg.") || app.has_document());
    let well = Rect::from_center_size(pos2(r.center().x, r.min.y + 17.0), vec2(36.0, 26.0));
    if checked {
        ui.painter().rect_filled(well, CornerRadius::same(6), t.checked);
    } else if resp.is_pointer_button_down_on() {
        ui.painter().rect_filled(well, CornerRadius::same(6), t.pressed);
    } else if resp.hovered() {
        ui.painter().rect_filled(well, CornerRadius::same(6), t.hover);
    }
    let c = if !on {
        t.text_disabled
    } else if checked {
        t.accent_text
    } else {
        t.icon
    };
    let ic = Rect::from_center_size(well.center(), vec2(20.0, 20.0));
    paint_icon(ui, ic, icon, c, if on { t.accent } else { c });
    let text = if menu { format!("{label} ▾") } else { label.to_string() };
    ui.painter().text(pos2(r.center().x, r.min.y + 38.0), Align2::CENTER_CENTER, text, regular(10.5), if on { t.text_dim } else { t.text_disabled });
    resp.widget_info(|| egui::WidgetInfo::labeled(egui::Role::Button, on, label));
    resp.on_hover_text(label)
}

/// The Zoom control: the percentage in a rounded box, labelled underneath.
fn zoom_tool(ui: &mut Ui, value: &str) -> egui::Response {
    let t = Tokens::get(ui.ctx());
    let (r, resp) = ui.allocate_exact_size(vec2(62.0, TOOLBAR_H - 8.0), Sense::click());
    let field = Rect::from_center_size(pos2(r.center().x, r.min.y + 17.0), vec2(56.0, 22.0));
    ui.painter().rect_filled(field, CornerRadius::same(6), if resp.hovered() { t.hover } else { t.input });
    ui.painter().rect_stroke(field, CornerRadius::same(6), Stroke::new(1.0, t.border_strong), egui::StrokeKind::Inside);
    ui.painter().text(field.center() - vec2(5.0, 0.0), Align2::CENTER_CENTER, value, regular(11.5), t.text);
    icons::paint(ui.painter(), Rect::from_center_size(pos2(field.max.x - 9.0, field.center().y), vec2(8.0, 8.0)), "dropdown", t.text_dim, t.text_dim);
    let label = tl!("Zoom");
    ui.painter().text(pos2(r.center().x, r.min.y + 38.0), Align2::CENTER_CENTER, label, regular(10.5), t.text_dim);
    resp.widget_info(|| egui::WidgetInfo::labeled(egui::Role::Button, true, label));
    resp
}

/// The toolbar's own icons (`pg.*`), or the shared ones.
fn paint_icon(ui: &Ui, r: Rect, name: &str, c: Color32, accent: Color32) {
    let p = ui.painter();
    let s = Stroke::new(1.4, c);
    let round = CornerRadius::same(3);
    match name {
        "pg.view" => {
            let b = r.shrink2(vec2(1.0, 3.0));
            p.rect_stroke(b, round, s, egui::StrokeKind::Inside);
            p.line_segment([pos2(b.min.x + 6.0, b.min.y), pos2(b.min.x + 6.0, b.max.y)], s);
        }
        "pg.addPage" => {
            let b = Rect::from_center_size(r.center(), vec2(13.0, 17.0));
            p.rect_stroke(b, round, s, egui::StrokeKind::Inside);
            let m = b.center();
            p.line_segment([m - vec2(3.5, 0.0), m + vec2(3.5, 0.0)], s);
            p.line_segment([m - vec2(0.0, 3.5), m + vec2(0.0, 3.5)], s);
        }
        "pg.insert" => {
            for i in 0..3 {
                let y = r.min.y + 5.0 + i as f32 * 5.0;
                p.line_segment([pos2(r.min.x + 2.0, y), pos2(r.max.x - 8.0, y)], s);
            }
            let m = pos2(r.max.x - 3.5, r.max.y - 4.0);
            p.line_segment([m - vec2(3.5, 0.0), m + vec2(3.5, 0.0)], Stroke::new(1.6, accent));
            p.line_segment([m - vec2(0.0, 3.5), m + vec2(0.0, 3.5)], Stroke::new(1.6, accent));
        }
        "pg.format" => {
            // A paintbrush: the handle, the ferrule and the bristles.
            let a = pos2(r.max.x - 3.0, r.min.y + 3.0);
            let b = pos2(r.center().x + 1.0, r.center().y - 1.0);
            p.line_segment([a, b], Stroke::new(2.0, c));
            let tip = [b + vec2(-1.5, -1.5), b + vec2(1.5, 1.5), pos2(r.min.x + 3.0, r.max.y - 3.0)];
            p.add(egui::Shape::convex_polygon(tip.to_vec(), accent, Stroke::NONE));
        }
        "pg.document" => {
            let b = Rect::from_center_size(r.center(), vec2(14.0, 18.0));
            let fold = 5.0;
            let pts = vec![
                b.left_top(),
                pos2(b.max.x - fold, b.min.y),
                pos2(b.max.x, b.min.y + fold),
                b.right_bottom(),
                b.left_bottom(),
                b.left_top(),
            ];
            p.add(egui::Shape::line(pts, s));
            p.line_segment([pos2(b.max.x - fold, b.min.y), pos2(b.max.x - fold, b.min.y + fold)], s);
            p.line_segment([pos2(b.max.x - fold, b.min.y + fold), pos2(b.max.x, b.min.y + fold)], s);
        }
        _ => icons::paint(p, r, name, c, accent),
    }
}

/// The Format sidebar on the right, when it is open.
pub fn inspector(app: &mut WordApp, ui: &mut Ui) {
    if !app.ui.pages_inspector || !app.has_document() {
        return;
    }
    let t = Tokens::get(ui.ctx());
    egui::Panel::right("pages_inspector")
        .exact_size(INSPECTOR_W)
        .resizable(false)
        .frame(egui::Frame::NONE.fill(t.panel).inner_margin(egui::Margin::symmetric(14, 10)).stroke(Stroke::new(1.0, t.border)))
        .show(ui, |ui| {
            ui.spacing_mut().item_spacing = vec2(6.0, 6.0);
            if app.ui.pages_tab == "document" {
                header(ui, &t, "Document");
                document_page(app, ui, &t);
                return;
            }
            let pages = [("style", "Style"), ("layout", "Layout"), ("more", "More")];
            header(ui, &t, "Text");
            let mut tab = app.ui.pages_tab.clone();
            if !pages.iter().any(|(k, _)| *k == tab) {
                tab = "style".into();
            }
            if let Some(k) = segmented(ui, &t, &pages.map(|(k, l)| (k, tl!(l))), &tab) {
                tab = k.to_string();
            }
            app.ui.pages_tab = tab.clone();
            ui.add_space(4.0);
            egui::ScrollArea::vertical().auto_shrink([false, false]).show(ui, |ui| match tab.as_str() {
                "layout" => layout_page(app, ui, &t),
                "more" => more_page(app, ui, &t),
                _ => style_page(app, ui, &t),
            });
        });
}

fn header(ui: &mut Ui, t: &Tokens, title: &str) {
    ui.vertical_centered(|ui| {
        ui.label(egui::RichText::new(tl!(title)).font(semibold(13.0)).color(t.text));
    });
}

fn section(ui: &mut Ui, t: &Tokens, title: &str) {
    ui.add_space(6.0);
    let (r, _) = ui.allocate_exact_size(vec2(ui.available_width(), 1.0), Sense::hover());
    ui.painter().line_segment([r.left_center(), r.right_center()], Stroke::new(1.0, t.border));
    ui.add_space(2.0);
    ui.label(egui::RichText::new(tl!(title)).font(semibold(12.0)).color(t.text));
}

/// A segmented control: the chosen key, when another segment was clicked.
fn segmented<'a>(ui: &mut Ui, t: &Tokens, items: &[(&'a str, &str)], current: &str) -> Option<&'a str> {
    let n = items.len().max(1) as f32;
    let (r, _) = ui.allocate_exact_size(vec2(ui.available_width(), 24.0), Sense::hover());
    ui.painter().rect_filled(r, CornerRadius::same(8), t.hover);
    let w = r.width() / n;
    let mut picked = None;
    for (i, (k, label)) in items.iter().enumerate() {
        let seg = Rect::from_min_size(pos2(r.min.x + w * i as f32, r.min.y), vec2(w, r.height()));
        let resp = ui.interact(seg, ui.scope_id().with(("seg", *k)), Sense::click());
        let on = *k == current;
        if on {
            let knob = seg.shrink(2.0);
            ui.painter().rect_filled(knob.translate(vec2(0.0, 0.5)), CornerRadius::same(6), Color32::from_black_alpha(24));
            ui.painter().rect_filled(knob, CornerRadius::same(6), t.input);
        }
        ui.painter().text(seg.center(), Align2::CENTER_CENTER, *label, if on { medium(11.5) } else { regular(11.5) }, t.text);
        if resp.clicked() && !on {
            picked = Some(*k);
        }
    }
    picked
}

/// A row of square icon toggles, like Pages' B / I / U / S and alignment rows.
fn icon_toggle(ui: &mut Ui, app: &mut WordApp, icon: &str, tip: &str, id: &str, on: bool, w: f32) {
    let t = Tokens::get(ui.ctx());
    let (r, resp) = ui.allocate_exact_size(vec2(w, 24.0), Sense::click());
    let fill = if on {
        t.checked
    } else if resp.hovered() {
        t.hover
    } else {
        t.input
    };
    ui.painter().rect_filled(r, CornerRadius::same(6), fill);
    ui.painter().rect_stroke(r, CornerRadius::same(6), Stroke::new(1.0, t.border_strong), egui::StrokeKind::Inside);
    let c = if on { t.accent_text } else { t.icon };
    icons::paint(ui.painter(), Rect::from_center_size(r.center(), vec2(16.0, 16.0)), icon, c, t.accent);
    let resp = resp.on_hover_text(tl!(tip));
    if resp.clicked() && enabled(app, id) {
        run_from_ribbon(app, id, json!({}));
    }
}

fn style_page(app: &mut WordApp, ui: &mut Ui, t: &Tokens) {
    let st = app.session.run("format.state", &json!({})).unwrap_or_default();
    let flag = |k: &str| st.get(k).and_then(Value::as_bool).unwrap_or(false);
    // The paragraph style, as a large drop-down button at the top.
    let info = wryte_engine::cmd::inspector::inspect(&app.session);
    let style_id = info["paragraph"]["style"].as_str().unwrap_or("Normal").to_string();
    let style_name = PARA_STYLES
        .iter()
        .find(|(id, _)| *id == style_id)
        .map(|(_, n)| tl!(*n).to_string())
        .unwrap_or_else(|| info["paragraph"]["styleName"].as_str().unwrap_or("Body").to_string());
    let (r, resp) = ui.allocate_exact_size(vec2(ui.available_width(), 40.0), Sense::click());
    ui.painter().rect_filled(r, CornerRadius::same(8), if resp.hovered() { t.hover } else { t.input });
    ui.painter().rect_stroke(r, CornerRadius::same(8), Stroke::new(1.0, t.border_strong), egui::StrokeKind::Inside);
    ui.painter().text(pos2(r.min.x + 12.0, r.center().y), Align2::LEFT_CENTER, &style_name, semibold(14.0), t.text);
    icons::paint(ui.painter(), Rect::from_center_size(pos2(r.max.x - 14.0, r.center().y), vec2(10.0, 10.0)), "dropdown", t.text_dim, t.text_dim);
    egui::Popup::menu(&resp).show(|ui| {
        ui.set_min_width(INSPECTOR_W - 40.0);
        ui.label(egui::RichText::new(tl!("Paragraph Styles")).font(semibold(11.0)).color(t.text_dim));
        for (id, name) in PARA_STYLES {
            if ui.selectable_label(style_id == id, tl!(name)).clicked() {
                run_from_ribbon(app, "para.style", json!({"style": id}));
                ui.close();
            }
        }
        ui.separator();
        if ui.button(tl!("Update Style to Match")).clicked() {
            run_from_ribbon(app, "styles.updateToMatch", json!({"style": style_id}));
            ui.close();
        }
    });

    section(ui, t, "Font");
    let font = st.get("font").and_then(Value::as_str).unwrap_or("").to_string();
    let font = app.previews.font_shown(&font);
    let fams = app.previews.families();
    let recent: Vec<String> = app.ui.recent_fonts.iter().map(|f| app.previews.font_shown(f)).collect();
    let prev = app.previews.font_preview_fn();
    let full = ui.available_width();
    if let Some(f) = crate::widgets::font_combo_with_recent_hint(
        ui,
        crate::widgets::FontComboOptions { id: "pages_font", width: full, current: &font, items: &fams, recent: &recent, preview: Some(&*prev), tip: tl!("Font") },
    ) {
        if f == crate::widgets::ADD_FONT {
            let _ = app.run("font.add", json!({}));
        } else if f == crate::widgets::LOAD_SYSTEM_FONTS {
            let _ = app.run("font.loadSystem", json!({}));
        } else {
            let f = app.previews.font_stored(&f);
            run_from_ribbon(app, "format.font", json!({"name": f}));
        }
    }
    ui.horizontal(|ui| {
        let size = st.get("size").and_then(Value::as_f64).map(|s| if s.fract() == 0.0 { format!("{s:.0} pt") } else { format!("{s} pt") }).unwrap_or_default();
        let sizes: Vec<String> =
            wryte_engine::cmd::format::SIZES.iter().map(|s| if s.fract() == 0.0 { format!("{s:.0}") } else { s.to_string() }).collect();
        if let Some(v) = crate::widgets::combo_hint(ui, "pages_size", 74.0, &size, &sizes, None, tl!("Font Size"))
            && let Ok(x) = v.trim().trim_end_matches("pt").trim().parse::<f64>()
        {
            run_from_ribbon(app, "format.size", json!({"size": x}));
        }
        ui.add_space(4.0);
        icon_toggle(ui, app, "shrink", "Decrease Font Size", "format.shrinkFont", false, 30.0);
        icon_toggle(ui, app, "grow", "Increase Font Size", "format.growFont", false, 30.0);
    });
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 5.0;
        let w = ((ui.available_width() - 42.0 - 4.0 * 5.0) / 4.0).max(24.0);
        icon_toggle(ui, app, "bold", "Bold", "format.bold", flag("bold"), w);
        icon_toggle(ui, app, "italic", "Italic", "format.italic", flag("italic"), w);
        icon_toggle(ui, app, "underline", "Underline", "format.underline", flag("underline"), w);
        icon_toggle(ui, app, "strike", "Strikethrough", "format.strikethrough", flag("strike"), w);
        ui.add_space(3.0);
        let fc = app.canvas.last_font_color.clone();
        let sw = wryte_doc::Rgb::parse(&fc).map(crate::theme::c32).unwrap_or(t.text);
        let (r, resp) = ui.allocate_exact_size(vec2(34.0, 24.0), Sense::click());
        ui.painter().rect_filled(r, CornerRadius::same(6), if resp.hovered() { t.hover } else { t.input });
        ui.painter().rect_stroke(r, CornerRadius::same(6), Stroke::new(1.0, t.border_strong), egui::StrokeKind::Inside);
        ui.painter().circle_filled(r.center(), 7.0, sw);
        ui.painter().circle_stroke(r.center(), 7.0, Stroke::new(1.0, t.border_strong));
        let resp = resp.on_hover_text(tl!("Text Color"));
        egui::Popup::menu(&resp).show(|ui| {
            if ui.button(tl!("Automatic")).clicked() {
                run_from_ribbon(app, "format.color", json!({"color": "auto"}));
                ui.close();
            }
            let theme = app.session.doc.settings.theme_colors.clone();
            crate::colors::menu(ui, app, &theme, crate::colors::Target::cmd("format.color").remember(crate::colors::Remember::FontColor));
        });
    });

    section(ui, t, "Alignment");
    let rp = app.session.doc.para_at(&app.session.sel.focus).map(|p| app.session.doc.styles.resolve_para(&p.props));
    let align = rp.as_ref().map(|r| r.align.visual(r.bidi)).unwrap_or_default();
    ui.horizontal(|ui| {
        use wryte_doc::Align as A;
        ui.spacing_mut().item_spacing.x = 5.0;
        let w = ((ui.available_width() - 60.0 - 10.0 - 6.0 * 5.0) / 4.0).max(24.0);
        icon_toggle(ui, app, "alignLeft", "Align Left", "para.alignLeft", align == A::Left, w);
        icon_toggle(ui, app, "alignCenter", "Center", "para.alignCenter", align == A::Center, w);
        icon_toggle(ui, app, "alignRight", "Align Right", "para.alignRight", align == A::Right, w);
        icon_toggle(ui, app, "justify", "Justify", "para.justify", align == A::Justify, w);
        ui.add_space(5.0);
        icon_toggle(ui, app, "outdent", "Decrease Indent", "para.outdent", false, 30.0);
        icon_toggle(ui, app, "indent", "Increase Indent", "para.indent", false, 30.0);
    });

    section(ui, t, "Spacing");
    ui.horizontal(|ui| {
        for v in [1.0, 1.15, 1.5, 2.0] {
            let label = if v == 1.0 { tl!("Single").to_string() } else { format!("{v}") };
            if ui.button(label).clicked() {
                run_from_ribbon(app, "para.lineSpacing", json!({"value": v}));
            }
        }
    });
    if ui.link(tl!("Spacing options…")).clicked() {
        run_from_ribbon(app, "para.dialog", json!({}));
    }

    section(ui, t, "Bullets & Lists");
    let lists = wryte_engine::cmd::para::list_buttons(&app.session.doc, rp.as_ref().and_then(|r| r.numbering));
    let shown = if lists.bullets {
        tl!("Bullet")
    } else if lists.numbering {
        tl!("Numbered")
    } else if lists.multilevel {
        tl!("Harvard")
    } else {
        tl!("None")
    };
    egui::ComboBox::from_id_salt("pages_lists").width(ui.available_width() - 8.0).selected_text(shown).show_ui(ui, |ui| {
        if ui.selectable_label(!lists.bullets && !lists.numbering && !lists.multilevel, tl!("None")).clicked() {
            run_from_ribbon(app, "para.bullets", json!({"off": true}));
        }
        if ui.selectable_label(lists.bullets, tl!("Bullet")).clicked() {
            run_from_ribbon(app, "para.bullets", json!({"kind": "•"}));
        }
        if ui.selectable_label(false, tl!("Dash")).clicked() {
            run_from_ribbon(app, "para.bullets", json!({"kind": "–"}));
        }
        if ui.selectable_label(lists.numbering, tl!("Numbered")).clicked() {
            run_from_ribbon(app, "para.numbering", json!({"kind": "numbered"}));
        }
        if ui.selectable_label(lists.multilevel, tl!("Harvard")).clicked() {
            run_from_ribbon(app, "para.multilevel", json!({"kind": "outline"}));
        }
        if ui.selectable_label(false, tl!("Legal")).clicked() {
            run_from_ribbon(app, "para.multilevel", json!({"kind": "legal"}));
        }
    });
}

fn layout_page(app: &mut WordApp, ui: &mut Ui, t: &Tokens) {
    section(ui, t, "Columns");
    ui.horizontal(|ui| {
        for n in 1..=3u64 {
            if ui.button(n.to_string()).clicked() {
                run_from_ribbon(app, "layout.columns", json!({"count": n}));
            }
        }
    });
    section(ui, t, "Indents");
    ui.horizontal(|ui| {
        icon_toggle(ui, app, "outdent", "Decrease Indent", "para.outdent", false, 34.0);
        icon_toggle(ui, app, "indent", "Increase Indent", "para.indent", false, 34.0);
    });
    if ui.link(tl!("Indents & spacing…")).clicked() {
        run_from_ribbon(app, "para.dialog", json!({}));
    }
    section(ui, t, "Tabs");
    if ui.button(tl!("Tabs…")).clicked() {
        run_from_ribbon(app, "ui.dialog", json!({"name": "tabs"}));
    }
    section(ui, t, "Borders & Rules");
    ui.horizontal(|ui| {
        for (l, k) in [("Bottom", "bottom"), ("Box", "outside"), ("None", "none")] {
            if ui.button(tl!(l)).clicked() {
                run_from_ribbon(app, "para.borders", json!({"kind": k}));
            }
        }
    });
}

fn more_page(app: &mut WordApp, ui: &mut Ui, t: &Tokens) {
    section(ui, t, "Pagination & Break");
    for (label, id) in [("Keep with next", "para.keepWithNext"), ("Keep lines together", "para.keepLinesTogether"), ("Page break before", "para.pageBreakBefore")] {
        if ui.add_enabled(enabled(app, id), egui::Button::new(tl!(label))).clicked() {
            run_from_ribbon(app, id, json!({}));
        }
    }
    if ui.link(tl!("Line & page breaks…")).clicked() {
        run_from_ribbon(app, "para.dialog", json!({}));
    }
    section(ui, t, "Language & Region");
    if ui.button(tl!("Set Proofing Language…")).clicked() {
        run_from_ribbon(app, "ui.dialog", json!({"name": "language"}));
    }
    section(ui, t, "Styles");
    if ui.button(tl!("Styles Pane")).clicked() {
        run_from_ribbon(app, "view.stylesPane", json!({"value": true}));
    }
}

fn document_page(app: &mut WordApp, ui: &mut Ui, t: &Tokens) {
    section(ui, t, "Printer & Paper Size");
    ui.horizontal(|ui| {
        for n in ["Letter", "A4", "Legal"] {
            if ui.button(n).clicked() {
                run_from_ribbon(app, "layout.size", json!({"name": n}));
            }
        }
    });
    section(ui, t, "Page Orientation");
    ui.horizontal(|ui| {
        if ui.button(tl!("Portrait")).clicked() {
            run_from_ribbon(app, "layout.orientation", json!({"value": "portrait"}));
        }
        if ui.button(tl!("Landscape")).clicked() {
            run_from_ribbon(app, "layout.orientation", json!({"value": "landscape"}));
        }
    });
    section(ui, t, "Document Margins");
    ui.horizontal(|ui| {
        for (l, k) in [("Normal", "normal"), ("Narrow", "narrow"), ("Wide", "wide")] {
            if ui.button(tl!(l)).clicked() {
                run_from_ribbon(app, "layout.margins", json!({"preset": k}));
            }
        }
    });
    section(ui, t, "Header & Footer");
    ui.horizontal(|ui| {
        if ui.button(tl!("Header")).clicked() {
            run_from_ribbon(app, "insert.editHeader", json!({}));
        }
        if ui.button(tl!("Footer")).clicked() {
            run_from_ribbon(app, "insert.editFooter", json!({}));
        }
    });
    section(ui, t, "Hyphenation");
    let mut hy = app.session.doc.settings.auto_hyphenation;
    if ui.checkbox(&mut hy, tl!("Hyphenate automatically")).changed() {
        run_from_ribbon(app, "layout.hyphenation", json!({"value": hy}));
    }
    ui.add_space(6.0);
    if ui.link(tl!("Page Setup…")).clicked() {
        run_from_ribbon(app, "layout.pageSetup", json!({}));
    }
}
