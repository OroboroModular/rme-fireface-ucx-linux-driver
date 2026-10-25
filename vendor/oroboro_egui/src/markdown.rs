//! A page of Markdown in the family's look: the kinds of block a plugin's
//! documentation pages use (headings, paragraphs, lists, tables, rules) and,
//! in them, **bold**, *italics* and `code`. Anything else shows as its text.
//! Tables lie in columns where there's room, and row by row (each row's
//! first cell, then the others under it) where there isn't (a phone).

use egui::text::{LayoutJob, TextFormat};
use egui::{pos2, vec2, Color32, CornerRadius, FontId, Galley, Rect, Sense, Stroke, Ui};
use std::sync::Arc;

use crate::theme::{ACCENT, BUTTON, DIVIDER, MUTED, SECONDARY, TEXT, VALUE};
use crate::touch;

/// One block of a page.
#[derive(Debug, Clone, PartialEq)]
pub enum Block {
    /// `#`, `##`, `###`: its level and text.
    Heading(u8, String),
    Paragraph(String),
    /// `- ` items (a line indented two spaces goes on with the item).
    List(Vec<String>),
    /// `|`-separated rows, the head first (the `|---|` line left out).
    Table(Vec<Vec<String>>),
    /// `---`.
    Rule,
}

/// The blocks of `text`.
pub fn parse(text: &str) -> Vec<Block> {
    let mut blocks = Vec::new();
    let mut paragraph: Vec<&str> = Vec::new();
    let mut list: Vec<String> = Vec::new();
    let mut table: Vec<Vec<String>> = Vec::new();
    let flush =
        |blocks: &mut Vec<Block>, paragraph: &mut Vec<&str>, list: &mut Vec<String>, table: &mut Vec<Vec<String>>| {
            if !paragraph.is_empty() {
                blocks.push(Block::Paragraph(paragraph.join(" ")));
                paragraph.clear();
            }
            if !list.is_empty() {
                blocks.push(Block::List(std::mem::take(list)));
            }
            if !table.is_empty() {
                blocks.push(Block::Table(std::mem::take(table)));
            }
        };
    for line in text.lines() {
        let trimmed = line.trim();
        // (an item's next line, indented)
        if let Some(item) = list.last_mut().filter(|_| line.starts_with("  ") && !trimmed.is_empty()) {
            item.push(' ');
            item.push_str(trimmed);
            continue;
        }
        if trimmed.is_empty() {
            flush(&mut blocks, &mut paragraph, &mut list, &mut table);
        } else if trimmed.starts_with('#') {
            flush(&mut blocks, &mut paragraph, &mut list, &mut table);
            let level = trimmed.chars().take_while(|&c| c == '#').count();
            blocks.push(Block::Heading(level.min(3) as u8, trimmed[level..].trim().to_string()));
        } else if trimmed == "---" || trimmed == "***" {
            flush(&mut blocks, &mut paragraph, &mut list, &mut table);
            blocks.push(Block::Rule);
        } else if let Some(item) = trimmed.strip_prefix("- ").or_else(|| trimmed.strip_prefix("* ")) {
            if !paragraph.is_empty() || !table.is_empty() {
                flush(&mut blocks, &mut paragraph, &mut list, &mut table);
            }
            list.push(item.trim().to_string());
        } else if trimmed.starts_with('|') {
            if !paragraph.is_empty() || !list.is_empty() {
                flush(&mut blocks, &mut paragraph, &mut list, &mut table);
            }
            let cells = cells(trimmed);
            let rule = cells.iter().all(|c| !c.is_empty() && c.chars().all(|ch| matches!(ch, '-' | ':' | ' ')));
            if !rule {
                table.push(cells);
            }
        } else {
            if !list.is_empty() || !table.is_empty() {
                flush(&mut blocks, &mut paragraph, &mut list, &mut table);
            }
            paragraph.push(trimmed);
        }
    }
    flush(&mut blocks, &mut paragraph, &mut list, &mut table);
    blocks
}

/// A table row's cells: split at `|` (not at `\|`, which is a `|` in a cell).
fn cells(row: &str) -> Vec<String> {
    let row = row.strip_prefix('|').unwrap_or(row);
    let row = row.strip_suffix('|').filter(|r| !r.ends_with('\\')).unwrap_or(row);
    let mut cells = vec![String::new()];
    let mut chars = row.chars().peekable();
    while let Some(c) = chars.next() {
        match c {
            '\\' if chars.peek() == Some(&'|') => {
                cells.last_mut().unwrap().push_str("\\|");
                chars.next();
            }
            '|' => cells.push(String::new()),
            c => cells.last_mut().unwrap().push(c),
        }
    }
    cells.into_iter().map(|c| c.trim().to_string()).collect()
}

/// The sizes a page is set in: its text (larger with fingers).
fn body() -> f32 {
    touch::pick(12.0, 15.0)
}

/// `text`'s inline marks as a layout `width` wide, in `color` (bold in
/// `strong`).
fn inline(text: &str, size: f32, color: Color32, strong: Color32, width: f32) -> LayoutJob {
    let mut job = LayoutJob::default();
    job.wrap.max_width = width;
    let (mut bold, mut italic, mut code) = (false, false, false);
    let mut run = String::new();
    let add = |job: &mut LayoutJob, run: &mut String, bold: bool, italic: bool, code: bool| {
        if run.is_empty() {
            return;
        }
        let format = TextFormat {
            font_id: if code { FontId::monospace(size * 0.9) } else { FontId::proportional(size) },
            color: if bold { strong } else { color },
            italics: italic,
            background: if code { BUTTON() } else { Color32::TRANSPARENT },
            ..Default::default()
        };
        job.append(&std::mem::take(run), 0.0, format);
    };
    let mut chars = text.chars().peekable();
    while let Some(c) = chars.next() {
        match c {
            '\\' if chars.peek().is_some_and(|n| "\\`*_|".contains(*n)) => run.push(chars.next().unwrap()),
            '`' => {
                add(&mut job, &mut run, bold, italic, code);
                code = !code;
            }
            '*' if !code && chars.peek() == Some(&'*') => {
                chars.next();
                add(&mut job, &mut run, bold, italic, code);
                bold = !bold;
            }
            '*' if !code => {
                add(&mut job, &mut run, bold, italic, code);
                italic = !italic;
            }
            c => run.push(c),
        }
    }
    add(&mut job, &mut run, bold, italic, code);
    job
}

fn galley(ui: &Ui, job: LayoutJob) -> Arc<Galley> {
    ui.ctx().fonts_mut(|f| f.layout_job(job))
}

/// Draws `galley` at the next place in `ui`, taking its room.
fn place(ui: &mut Ui, galley: Arc<Galley>) {
    let (rect, _) = ui.allocate_exact_size(galley.size(), Sense::hover());
    ui.painter().galley(rect.min, galley, TEXT());
}

/// Draws the page `text` in `ui`, as wide as `ui` is.
pub fn show(ui: &mut Ui, text: &str) {
    ui.spacing_mut().item_spacing.y = 0.0;
    let blocks = parse(text);
    for (i, block) in blocks.iter().enumerate() {
        let size = body();
        let width = ui.available_width();
        match block {
            Block::Heading(1, text) => {
                place(ui, galley(ui, inline(text, size * 1.6, VALUE(), ACCENT(), width)));
                ui.add_space(size * 0.3);
            }
            Block::Heading(2, text) => {
                if i > 0 {
                    ui.add_space(size * 0.9);
                }
                // as the family's section heads: an accent mark, UPPERCASE
                let job = inline(&text.to_uppercase(), size * 0.85, SECONDARY(), SECONDARY(), width - 10.0);
                let g = galley(ui, job);
                let (rect, _) = ui.allocate_exact_size(vec2(width, g.size().y), Sense::hover());
                let mark = Rect::from_min_size(pos2(rect.left(), rect.center().y - 5.0), vec2(3.0, 10.0));
                ui.painter().rect_filled(mark, CornerRadius::same(1), ACCENT());
                ui.painter().galley(pos2(rect.left() + 9.0, rect.top()), g, SECONDARY());
                ui.add_space(size * 0.5);
            }
            Block::Heading(_, text) => {
                ui.add_space(size * 0.5);
                place(ui, galley(ui, inline(&format!("**{text}**"), size, TEXT(), VALUE(), width)));
                ui.add_space(size * 0.3);
            }
            Block::Paragraph(text) => {
                place(ui, galley(ui, inline(text, size, TEXT(), VALUE(), width)));
                ui.add_space(size * 0.6);
            }
            Block::List(items) => {
                let indent = size * 1.1;
                for item in items {
                    let g = galley(ui, inline(item, size, TEXT(), VALUE(), width - indent));
                    let (rect, _) = ui.allocate_exact_size(vec2(width, g.size().y), Sense::hover());
                    let row = g.rows.first().map_or(size, |r| r.height());
                    ui.painter().circle_filled(
                        pos2(rect.left() + size * 0.35, rect.top() + row * 0.55),
                        size * 0.12,
                        MUTED(),
                    );
                    ui.painter().galley(pos2(rect.left() + indent, rect.top()), g, TEXT());
                    ui.add_space(size * 0.3);
                }
                ui.add_space(size * 0.3);
            }
            Block::Table(rows) => table(ui, rows, size, width),
            Block::Rule => {
                let (rect, _) = ui.allocate_exact_size(vec2(width, size), Sense::hover());
                ui.painter().hline(rect.x_range(), rect.center().y, Stroke::new(1.0, DIVIDER()));
            }
        }
    }
}

/// Where a table lies in columns: at least this wide.
const COLUMNS_FROM: f32 = 440.0;

/// A table: its head (muted) and rows in columns, the last column taking
/// what the others leave and wrapping; on a narrow page, row by row.
fn table(ui: &mut Ui, rows: &[Vec<String>], size: f32, width: f32) {
    let Some((head, rows)) = rows.split_first() else { return };
    let n = rows.iter().map(Vec::len).chain([head.len()]).max().unwrap_or(0);
    let cell = |row: &Vec<String>, c: usize| row.get(c).map_or("", String::as_str).to_string();
    let small = size * 0.85;
    let gap = size * 0.9;
    if width < COLUMNS_FROM || n < 2 {
        // row by row: the first cell, the middle ones with their heads, the last under them
        for row in rows {
            let (rect, _) = ui.allocate_exact_size(vec2(width, 1.0), Sense::hover());
            ui.painter().hline(rect.x_range(), rect.top(), Stroke::new(1.0, DIVIDER()));
            ui.add_space(size * 0.3);
            let mut line = format!("**{}**", cell(row, 0));
            for c in 1..n.saturating_sub(1) {
                let value = cell(row, c);
                if !value.is_empty() {
                    line.push_str(&format!("   *{}* {}", cell(head, c), value));
                }
            }
            place(ui, galley(ui, inline(&line, size, TEXT(), VALUE(), width)));
            if n > 1 && !cell(row, n - 1).is_empty() {
                ui.add_space(size * 0.15);
                place(ui, galley(ui, inline(&cell(row, n - 1), size, TEXT(), VALUE(), width)));
            }
            ui.add_space(size * 0.5);
        }
        ui.add_space(size * 0.2);
        return;
    }
    // columns: all but the last as wide as their widest cell (to a share of
    // the page), the last the rest
    let natural = |text: &str, s: f32| galley(ui, inline(text, s, TEXT(), VALUE(), f32::INFINITY)).size().x;
    let mut widths: Vec<f32> = (0..n - 1)
        .map(|c| rows.iter().map(|r| natural(&cell(r, c), size)).fold(natural(&cell(head, c), small), f32::max))
        .map(|w| w.min(width * 0.28))
        .collect();
    let fixed: f32 = widths.iter().sum::<f32>() + gap * (n - 1) as f32;
    let least = width * 0.38;
    if width - fixed < least {
        let scale = (width - least - gap * (n - 1) as f32).max(1.0) / widths.iter().sum::<f32>();
        widths.iter_mut().for_each(|w| *w *= scale);
    }
    let fixed: f32 = widths.iter().sum::<f32>() + gap * (n - 1) as f32;
    widths.push((width - fixed).max(size * 4.0));
    let lay_row = |ui: &mut Ui, row: &Vec<String>, s: f32, color: Color32| {
        let galleys: Vec<_> = (0..n).map(|c| galley(ui, inline(&cell(row, c), s, color, VALUE(), widths[c]))).collect();
        let height = galleys.iter().map(|g| g.size().y).fold(0.0, f32::max);
        let (rect, _) = ui.allocate_exact_size(vec2(width, height), Sense::hover());
        let mut x = rect.left();
        for (c, g) in galleys.into_iter().enumerate() {
            ui.painter().galley(pos2(x, rect.top()), g, color);
            x += widths[c] + gap;
        }
    };
    lay_row(ui, head, small, MUTED());
    for row in rows {
        ui.add_space(size * 0.3);
        let (rect, _) = ui.allocate_exact_size(vec2(width, 1.0), Sense::hover());
        ui.painter().hline(rect.x_range(), rect.top(), Stroke::new(1.0, DIVIDER()));
        ui.add_space(size * 0.3);
        lay_row(ui, row, size, TEXT());
    }
    ui.add_space(size * 0.8);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_page_reads_as_its_blocks() {
        let page = "# VCO\n\n*Oscillators · modern*\n\nThe basic oscillator:\nsine and saw.\n\n## Inputs\n\n\
                    | Jack | What it does |\n|---|---|\n| V/Oct | Pitch, 1 V an octave |\n| A\\|B | either |\n\n\
                    - one\n  still one\n- two\n\n---\n";
        assert_eq!(
            parse(page),
            [
                Block::Heading(1, "VCO".into()),
                Block::Paragraph("*Oscillators · modern*".into()),
                Block::Paragraph("The basic oscillator: sine and saw.".into()),
                Block::Heading(2, "Inputs".into()),
                Block::Table(vec![
                    vec!["Jack".into(), "What it does".into()],
                    vec!["V/Oct".into(), "Pitch, 1 V an octave".into()],
                    vec!["A\\|B".into(), "either".into()],
                ]),
                Block::List(vec!["one still one".into(), "two".into()]),
                Block::Rule,
            ]
        );
    }

    #[test]
    fn marks_are_bold_italic_and_code() {
        let job = inline("a **b** *c* `d*e` x\\|y", 12.0, Color32::WHITE, Color32::RED, 100.0);
        let runs: Vec<(String, bool, bool, bool)> = job
            .sections
            .iter()
            .map(|s| {
                let f = &s.format;
                (
                    job.text[s.byte_range.clone()].to_string(),
                    f.color == Color32::RED,
                    f.italics,
                    f.font_id.family == egui::FontFamily::Monospace,
                )
            })
            .collect();
        let want = [
            ("a ", false, false, false),
            ("b", true, false, false),
            (" ", false, false, false),
            ("c", false, true, false),
            (" ", false, false, false),
            ("d*e", false, false, true),
            (" x|y", false, false, false),
        ];
        assert_eq!(runs, want.map(|(t, b, i, c)| (t.to_string(), b, i, c)));
    }
}
