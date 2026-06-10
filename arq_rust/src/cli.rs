// ─────────────────────────────────────────────────────────────────────────────
// cli.rs — Interfaz de consola para el Analizador Heurístico de Hardware.
// ─────────────────────────────────────────────────────────────────────────────

use colored::*;
use comfy_table::{Table, Row, Cell, Color as TColor, Attribute, presets, ContentArrangement};
use chrono::Local;

use crate::engine::{evaluate, EvalResult, Status};
use crate::rules::{component_label, component_unit, COMPONENT_ORDER, TASK_ORDER};
use crate::scanner::{scan_hardware, HardwareInfo};

fn format_gb(val: f64) -> String {
    if val >= 1000.0 { format!("{:.1} TB", val / 1000.0) }
    else { format!("{:.1} GB", val) }
}

fn status_color(s: &Status) -> TColor {
    match s {
        Status::Optimo => TColor::Green,
        Status::Suficiente => TColor::Yellow,
        Status::Insuficiente => TColor::Red,
    }
}

fn progress_bar(actual: f64, ideal: f64, width: usize) -> String {
    if ideal == 0.0 { return "N/A".to_string(); }
    let ratio = (actual / ideal).min(1.0);
    let filled = (ratio * width as f64) as usize;
    let empty = width - filled;
    let pct = (ratio * 100.0) as u32;
    format!("{}{} {}%", "█".repeat(filled), "░".repeat(empty), pct)
}

fn print_box(title: &str, content: &str, color: &str) {
    let lines: Vec<&str> = content.lines().collect();
    let max_w = lines.iter().map(|l| l.chars().count()).max().unwrap_or(40).max(title.chars().count() + 4);
    let w = max_w + 4;
    let top = format!("╭─ {} {}╮", title, "─".repeat(w.saturating_sub(title.chars().count() + 4)));
    let bot = format!("╰{}╯", "─".repeat(w));
    let colored_top = match color {
        "cyan" => top.bright_cyan().to_string(),
        "green" => top.green().to_string(),
        "magenta" => top.bright_magenta().to_string(),
        "yellow" => top.bright_yellow().to_string(),
        "blue" => top.bright_blue().to_string(),
        "white" => top.bright_white().to_string(),
        _ => top,
    };
    let colored_bot = match color {
        "cyan" => bot.bright_cyan().to_string(),
        "green" => bot.green().to_string(),
        "magenta" => bot.bright_magenta().to_string(),
        "yellow" => bot.bright_yellow().to_string(),
        "blue" => bot.bright_blue().to_string(),
        "white" => bot.bright_white().to_string(),
        _ => bot,
    };
    println!("{}", colored_top);
    for line in &lines {
        let pad = w.saturating_sub(line.chars().count() + 1);
        println!("│ {}{} │", line, " ".repeat(pad));
    }
    println!("{}", colored_bot);
}

fn render_header() {
    println!();
    println!("{}", "╔══════════════════════════════════════════════════════════════════╗".bright_cyan());
    println!("{}", "║                                                                  ║".bright_cyan());
    println!("{}  ⚡ {} ⚡  {}", "║".bright_cyan(), "ANALIZADOR HEURÍSTICO DE HARDWARE".bright_white().bold(), "║".bright_cyan());
    println!("{}  {}  {}", "║".bright_cyan(), "Versión Rust — Evaluador de umbrales lógicos".dimmed(), "║".bright_cyan());
    println!("{}", "║                                                                  ║".bright_cyan());
    println!("{}", "╚══════════════════════════════════════════════════════════════════╝".bright_cyan());
    println!();
}

fn render_hw_summary(hw: &HardwareInfo) {
    // CPU
    let cpu_info = format!(
        "  Modelo:          {}\n  Núcleos Físicos: {}\n  Núcleos Lógicos: {}\n  Frecuencia:      {} MHz\n  Arquitectura:    {}",
        hw.cpu.brand, hw.cpu.physical_cores, hw.cpu.logical_cores,
        hw.cpu.frequency_mhz, hw.cpu.arch
    );
    print_box("🔧 CPU", &cpu_info, "cyan");
    println!();

    // RAM
    let ram_info = format!(
        "  Total:      {}\n  Disponible: {}\n  En uso:     {:.1}%",
        format_gb(hw.ram.total_gb), format_gb(hw.ram.available_gb), hw.ram.percent_used
    );
    print_box("💾 RAM", &ram_info, "green");
    println!();

    // GPU
    match &hw.gpu {
        Some(gpu) => {
            let gpu_info = format!(
                "  Modelo:     {}\n  VRAM Total: {}\n  VRAM Libre: {}\n  Driver:     {}",
                gpu.name, format_gb(gpu.vram_total_gb), format_gb(gpu.vram_free_gb), gpu.driver_version
            );
            print_box("🎮 GPU", &gpu_info, "magenta");
        }
        None => {
            let gpu_info = "  Estado: No se detectó GPU NVIDIA\n  VRAM:   0 GB";
            print_box("🎮 GPU", gpu_info, "magenta");
        }
    }
    println!();

    // Storage
    let stor_info = format!(
        "  Total:  {}\n  Libre:  {}\n  En uso: {:.1}%",
        format_gb(hw.storage.total_gb), format_gb(hw.storage.free_gb), hw.storage.percent_used
    );
    print_box("💿 Almacenamiento", &stor_info, "yellow");
    println!();
}

fn render_eval_tables(results: &[EvalResult]) {
    for task in TASK_ORDER {
        let task_results: Vec<&EvalResult> = results.iter().filter(|r| r.task == *task).collect();
        if task_results.is_empty() { continue; }

        let task_color = match *task {
            "IA Generativa" => TColor::Magenta,
            "Data Science" => TColor::Cyan,
            "Bases de Datos" => TColor::Blue,
            "Gaming" => TColor::Green,
            _ => TColor::White,
        };

        let mut table = Table::new();
        table
            .load_preset(presets::UTF8_FULL)
            .set_content_arrangement(ContentArrangement::Dynamic)
            .set_header(vec![
                Cell::new("Magnitud").add_attribute(Attribute::Bold),
                Cell::new("Descripción"),
                Cell::new("Veredicto").add_attribute(Attribute::Bold),
                Cell::new("RAM"),
                Cell::new("VRAM"),
                Cell::new("CPU Cores"),
                Cell::new("Storage"),
                Cell::new("Cuello de Botella"),
            ]);

        for r in &task_results {
            let sc = status_color(&r.overall_status);
            let mut row_cells = vec![
                Cell::new(&r.magnitude).add_attribute(Attribute::Bold),
                Cell::new(&r.description).fg(TColor::DarkGrey),
                Cell::new(format!("{} {}", r.overall_status.emoji(), r.overall_status.label()))
                    .fg(sc).add_attribute(Attribute::Bold),
            ];
            for comp in COMPONENT_ORDER {
                if let Some(d) = r.details.iter().find(|d| d.name == *comp) {
                    if d.min == 0.0 && d.ideal == 0.0 {
                        row_cells.push(Cell::new("N/A").fg(TColor::DarkGrey));
                    } else {
                        let c = status_color(&d.status);
                        row_cells.push(Cell::new(format!("{} {}", d.status.emoji(), d.status.label())).fg(c));
                    }
                }
            }
            let bn_color = match r.overall_status {
                Status::Insuficiente => TColor::Red,
                Status::Suficiente => TColor::Yellow,
                Status::Optimo => TColor::DarkGrey,
            };
            row_cells.push(Cell::new(&r.bottleneck).fg(bn_color));
            table.add_row(Row::from(row_cells));
        }

        let task_title = match *task {
            "IA Generativa" => format!("{}", "━━━ IA GENERATIVA ━━━".bright_magenta().bold()),
            "Data Science" => format!("{}", "━━━ DATA SCIENCE ━━━".bright_cyan().bold()),
            "Bases de Datos" => format!("{}", "━━━ BASES DE DATOS ━━━".bright_blue().bold()),
            "Gaming" => format!("{}", "━━━ GAMING ━━━".bright_green().bold()),
            _ => format!("{}", "━━━ USO GENERAL ━━━".bright_white().bold()),
        };
        println!("{}", task_title);
        println!("{table}");
        println!();
    }
}

fn render_detail_panels(results: &[EvalResult]) {
    let high_results: Vec<&EvalResult> = results.iter().filter(|r| r.magnitude == "Alta").collect();
    if high_results.is_empty() { return; }

    println!("{}", "╔══════════════════════════════════════════════════════════════════╗".bright_white());
    println!("{}  {} {}", "║".bright_white(), "📋 Detalle por Componente — Cargas Altas".bold(), "║".bright_white());
    println!("{}", "╚══════════════════════════════════════════════════════════════════╝".bright_white());
    println!();

    for r in &high_results {
        let mut table = Table::new();
        table
            .load_preset(presets::UTF8_FULL)
            .set_content_arrangement(ContentArrangement::Dynamic)
            .set_header(vec![
                Cell::new("Componente").add_attribute(Attribute::Bold),
                Cell::new("Actual").add_attribute(Attribute::Bold),
                Cell::new("Mínimo"),
                Cell::new("Ideal"),
                Cell::new("Progreso vs Ideal"),
            ]);

        for d in &r.details {
            if d.min == 0.0 && d.ideal == 0.0 { continue; }
            let label = component_label(d.name);
            let unit = component_unit(d.name);
            let bar = progress_bar(d.actual, d.ideal, 20);
            table.add_row(vec![
                Cell::new(label).add_attribute(Attribute::Bold),
                Cell::new(format!("{:.1} {}", d.actual, unit)),
                Cell::new(format!("{} {}", d.min, unit)).fg(TColor::DarkGrey),
                Cell::new(format!("{} {}", d.ideal, unit)).fg(TColor::DarkGrey),
                Cell::new(&bar),
            ]);
        }

        let title = format!("{} — Carga Alta", r.task);
        print_box(&title, &format!("{table}"), "blue");
        println!();
    }
}

fn render_footer() {
    let now = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    println!("╭──────────────────────────────────────────────────────────────────╮");
    println!(
        "│ 📅 Generado: {}  │  ⚠️  Basado en umbrales, no benchmarks.  │",
        now
    );
    println!("╰──────────────────────────────────────────────────────────────────╯");
    println!();
}

/// Punto de entrada de la interfaz CLI.
pub fn run() {
    render_header();
    println!("  {} Escaneando hardware...\n", "⏳".dimmed());
    let hw = scan_hardware();
    render_hw_summary(&hw);
    println!("  {} Evaluando umbrales...\n", "⏳".dimmed());
    let results = evaluate(&hw);
    render_eval_tables(&results);
    render_detail_panels(&results);
    render_footer();
}
