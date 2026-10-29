//! Window controls drawn by the app for a window without a system title bar (Windows, Pages
//! style): minimize, maximize and close at the toolbar's right end, in that order with Windows'
//! thin-line glyphs, in rounded hover wells, and resizing from the window's edges.

use egui::{CornerRadius, Rect, Sense, Stroke, Ui, pos2, vec2};

use crate::theme::Tokens;

/// One control's size and the gap between them.
const SIZE: egui::Vec2 = egui::vec2(30.0, 24.0);
const GAP: f32 = 3.0;
/// Room between the window's edge and the controls: the Share button's inset on the other side.
pub const INSET: f32 = 10.0;
/// Width of the window edge that resizes it.
const EDGE: f32 = 5.0;

/// The room [`buttons`] takes.
pub const WIDTH: f32 = 3.0 * SIZE.x + 2.0 * GAP;

/// Draws minimize, maximize and close left to right, centred on the row.
pub fn buttons(ui: &mut Ui) {
    let t = Tokens::get(ui.ctx());
    let maximized = ui.ctx().input(|i| i.viewport().maximized.unwrap_or(false));
    // GAP is the only space between them, so they fill exactly WIDTH.
    ui.spacing_mut().item_spacing.x = 0.0;
    for (i, (kind, label)) in [("min", tl!("Minimize")), ("zoom", if maximized { tl!("Restore Down") } else { tl!("Maximize") }), ("close", tl!("Close"))]
        .into_iter()
        .enumerate()
    {
        if i > 0 {
            ui.add_space(GAP);
        }
        let (r, resp) = ui.allocate_exact_size(SIZE, Sense::click());
        resp.widget_info(|| egui::WidgetInfo::labeled(egui::Role::Button, true, label));
        let hot = resp.hovered() || resp.is_pointer_button_down_on();
        let down = resp.is_pointer_button_down_on();
        let (fill, ink) = match (kind, hot) {
            ("close", true) => (if down { t.close_pressed } else { t.close_hover }, t.on_close),
            (_, true) => (if down { t.pressed } else { t.hover }, t.text),
            _ => (t.input, t.text),
        };
        // Each control sits in its own box.
        ui.painter().rect_filled(r, CornerRadius::same(6), fill);
        if !hot {
            ui.painter().rect_stroke(r, CornerRadius::same(6), Stroke::new(1.0, t.border), egui::StrokeKind::Inside);
        }
        glyph(ui, r, kind, maximized, ink);
        let resp = resp.on_hover_text(label);
        if resp.clicked() {
            let cmd = match kind {
                "close" => egui::ViewportCommand::Close,
                "min" => egui::ViewportCommand::Minimized(true),
                _ => egui::ViewportCommand::Maximized(!maximized),
            };
            ui.ctx().send_viewport_cmd(cmd);
        }
    }
}

fn glyph(ui: &Ui, r: Rect, kind: &str, maximized: bool, ink: egui::Color32) {
    let p = ui.painter();
    let c = pos2(p.round_to_pixel_center(r.center().x), p.round_to_pixel_center(r.center().y));
    let s = Stroke::new(1.0, ink);
    match kind {
        "close" => {
            let g = 3.5;
            p.line_segment([c + vec2(-g, -g), c + vec2(g, g)], s);
            p.line_segment([c + vec2(-g, g), c + vec2(g, -g)], s);
        }
        "min" => {
            p.line_segment([c + vec2(-4.0, 0.0), c + vec2(4.0, 0.0)], s);
        }
        _ if maximized => {
            // Two overlapping squares: restore.
            p.rect_stroke(Rect::from_center_size(c + vec2(-1.0, 1.0), vec2(6.0, 6.0)), 1.0, s, egui::StrokeKind::Middle);
            p.line_segment([c + vec2(-1.0, -3.5), c + vec2(3.5, -3.5)], s);
            p.line_segment([c + vec2(3.5, -3.5), c + vec2(3.5, 1.0)], s);
        }
        _ => {
            p.rect_stroke(Rect::from_center_size(c, vec2(8.0, 8.0)), 1.0, s, egui::StrokeKind::Middle);
        }
    }
}

/// Lets the window's edges and corners resize it, since there is no system frame to grab.
pub fn resize_edges(ctx: &egui::Context) {
    let maximized = ctx.input(|i| i.viewport().maximized.unwrap_or(false) || i.viewport().fullscreen.unwrap_or(false));
    if maximized {
        return;
    }
    let screen = ctx.content_rect();
    let Some(pos) = ctx.input(|i| i.pointer.hover_pos()) else {
        return;
    };
    let (l, r) = (pos.x - screen.min.x <= EDGE, screen.max.x - pos.x <= EDGE);
    let (top, b) = (pos.y - screen.min.y <= EDGE, screen.max.y - pos.y <= EDGE);
    use egui::ResizeDirection as D;
    let dir = match (l, r, top, b) {
        (true, _, true, _) => D::NorthWest,
        (_, true, true, _) => D::NorthEast,
        (true, _, _, true) => D::SouthWest,
        (_, true, _, true) => D::SouthEast,
        (true, ..) => D::West,
        (_, true, ..) => D::East,
        (_, _, true, _) => D::North,
        (_, _, _, true) => D::South,
        _ => return,
    };
    let icon = match dir {
        D::North | D::South => egui::CursorIcon::ResizeVertical,
        D::East | D::West => egui::CursorIcon::ResizeHorizontal,
        D::NorthWest | D::SouthEast => egui::CursorIcon::ResizeNwSe,
        D::NorthEast | D::SouthWest => egui::CursorIcon::ResizeNeSw,
    };
    ctx.set_cursor_icon(icon);
    if ctx.input(|i| i.pointer.primary_pressed()) {
        ctx.send_viewport_cmd(egui::ViewportCommand::BeginResize(dir));
    }
}

/// A strip that drags the window, and maximizes or restores it on a double click.
pub fn drag_area(ui: &Ui, rect: Rect, id: &str) {
    let drag = ui.interact(rect, ui.scope_id().with(id), Sense::click_and_drag());
    if drag.drag_started() {
        ui.ctx().send_viewport_cmd(egui::ViewportCommand::StartDrag);
    }
    if drag.double_clicked() {
        let max = ui.ctx().input(|i| i.viewport().maximized.unwrap_or(false));
        ui.ctx().send_viewport_cmd(egui::ViewportCommand::Maximized(!max));
    }
}
