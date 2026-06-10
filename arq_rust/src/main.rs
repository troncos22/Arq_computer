// ─────────────────────────────────────────────────────────────────────────────
// main.rs — Punto de entrada del Analizador Heurístico de Hardware (Rust).
// ─────────────────────────────────────────────────────────────────────────────

mod rules;
mod scanner;
mod engine;
mod cli;

fn main() {
    cli::run();
}
