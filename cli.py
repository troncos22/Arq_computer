"""
cli.py — Interfaz de consola con Rich para el Analizador Heurístico de Hardware.

Renderiza un reporte elegante con paneles, tablas y colores de semáforo
mostrando la evaluación del hardware del sistema para distintas cargas.
"""

from __future__ import annotations

import sys
from datetime import datetime

from rich.console import Console
from rich.panel import Panel
from rich.table import Table
from rich.text import Text
from rich.columns import Columns
from rich import box

from scanner import scan_hardware
from engine import evaluate
from rules import TASK_ORDER, COMPONENT_LABELS


# ─── Constantes de estilo ────────────────────────────────────────────────────

STATUS_STYLES: dict[str, dict] = {
    "ÓPTIMO":       {"color": "bold green",  "emoji": "🟢"},
    "SUFICIENTE":   {"color": "bold yellow", "emoji": "🟡"},
    "INSUFICIENTE": {"color": "bold red",    "emoji": "🔴"},
}

TASK_COLORS: dict[str, str] = {
    "IA Generativa": "bright_magenta",
    "Data Science":  "bright_cyan",
    "Bases de Datos": "bright_blue",
    "Gaming":        "bright_green",
    "Uso General":   "bright_white",
}

console = Console()


# ─── Helpers ─────────────────────────────────────────────────────────────────

def _styled_status(status: str) -> Text:
    """Devuelve un Text de Rich con el emoji y color del estado."""
    style_info = STATUS_STYLES.get(status, {"color": "white", "emoji": "⚪"})
    return Text(f"{style_info['emoji']} {status}", style=style_info["color"])


def _format_gb(value: float) -> str:
    """Formatea un valor en GB de forma legible."""
    if value >= 1000:
        return f"{value / 1000:.1f} TB"
    return f"{value:.1f} GB"


def _progress_bar(actual: float, ideal: float, width: int = 20) -> Text:
    """Genera una barra de progreso visual ASCII."""
    if ideal == 0:
        return Text("N/A", style="dim")
    ratio = min(actual / ideal, 1.0)
    filled = int(ratio * width)
    empty = width - filled

    if ratio >= 1.0:
        color = "green"
    elif ratio >= 0.5:
        color = "yellow"
    else:
        color = "red"

    bar = Text()
    bar.append("█" * filled, style=color)
    bar.append("░" * empty, style="dim")
    bar.append(f" {ratio * 100:.0f}%", style=f"bold {color}")
    return bar


# ─── Renderizado de Secciones ────────────────────────────────────────────────

def render_header() -> None:
    """Renderiza el encabezado del reporte."""
    title = Text()
    title.append("⚡ ", style="bright_yellow")
    title.append("ANALIZADOR HEURÍSTICO DE HARDWARE", style="bold bright_white")
    title.append(" ⚡", style="bright_yellow")

    subtitle = Text()
    subtitle.append(
        "Evaluador de umbrales lógicos — No es un benchmark de estrés",
        style="dim italic",
    )

    header_panel = Panel(
        Text.assemble(title, "\n", subtitle),
        border_style="bright_cyan",
        box=box.DOUBLE_EDGE,
        padding=(1, 2),
    )
    console.print(header_panel)


def render_hardware_summary(hw: dict) -> None:
    """Renderiza un panel con el resumen del hardware detectado."""
    cpu = hw["cpu"]
    ram = hw["ram"]
    gpu = hw["gpu"]
    storage = hw["storage"]

    # ── CPU Card ──
    cpu_table = Table(
        show_header=False, box=None, padding=(0, 1),
        title="[bold bright_cyan]🔧 CPU[/]",
        title_style="bold",
    )
    cpu_table.add_column("Key", style="dim", width=14)
    cpu_table.add_column("Value", style="bold")
    cpu_table.add_row("Modelo", cpu["brand"])
    cpu_table.add_row("Núcleos Físicos", str(cpu["physical_cores"]))
    cpu_table.add_row("Núcleos Lógicos", str(cpu["logical_cores"]))
    cpu_table.add_row("Frecuencia", f"{cpu['frequency_mhz']:.0f} MHz")
    cpu_table.add_row("Arquitectura", cpu["arch"])

    # ── RAM Card ──
    ram_table = Table(
        show_header=False, box=None, padding=(0, 1),
        title="[bold bright_green]💾 RAM[/]",
        title_style="bold",
    )
    ram_table.add_column("Key", style="dim", width=14)
    ram_table.add_column("Value", style="bold")
    ram_table.add_row("Total", _format_gb(ram["total_gb"]))
    ram_table.add_row("Disponible", _format_gb(ram["available_gb"]))
    ram_table.add_row("En uso", f"{ram['percent_used']}%")

    # ── GPU Card ──
    gpu_table = Table(
        show_header=False, box=None, padding=(0, 1),
        title="[bold bright_magenta]🎮 GPU[/]",
        title_style="bold",
    )
    gpu_table.add_column("Key", style="dim", width=14)
    gpu_table.add_column("Value", style="bold")
    if gpu["available"]:
        gpu_table.add_row("Modelo", gpu["name"])
        gpu_table.add_row("VRAM Total", _format_gb(gpu["vram_total_gb"]))
        gpu_table.add_row("VRAM Libre", _format_gb(gpu["vram_free_gb"]))
        gpu_table.add_row("Driver", gpu["driver_version"])
    else:
        gpu_table.add_row("Estado", "[dim italic]No se detectó GPU NVIDIA[/]")
        gpu_table.add_row("VRAM", "[dim]0 GB[/]")

    # ── Storage Card ──
    stor_table = Table(
        show_header=False, box=None, padding=(0, 1),
        title="[bold bright_yellow]💿 Almacenamiento[/]",
        title_style="bold",
    )
    stor_table.add_column("Key", style="dim", width=14)
    stor_table.add_column("Value", style="bold")
    stor_table.add_row("Total", _format_gb(storage["total_gb"]))
    stor_table.add_row("Libre", _format_gb(storage["free_gb"]))
    stor_table.add_row("En uso", f"{storage['percent_used']}%")

    # Componer las 4 tarjetas en un panel
    hw_panel = Panel(
        Columns(
            [cpu_table, ram_table, gpu_table, stor_table],
            equal=True,
            expand=True,
        ),
        title="[bold]📊 Hardware Detectado[/]",
        border_style="bright_blue",
        box=box.ROUNDED,
        padding=(1, 2),
    )
    console.print(hw_panel)


def render_evaluation(results: list[dict], hw: dict) -> None:
    """Renderiza las tablas de evaluación agrupadas por tarea."""
    current_task = None

    for result in results:
        task = result["task"]
        mag = result["magnitude"]

        # Nueva tarea → nueva tabla
        if task != current_task:
            if current_task is not None:
                # Cerrar tabla anterior
                console.print(table)
                console.print()

            current_task = task
            task_color = TASK_COLORS.get(task, "white")

            table = Table(
                title=f"[bold {task_color}]{'━' * 3} {task.upper()} {'━' * 3}[/]",
                box=box.HEAVY_HEAD,
                border_style="dim",
                show_lines=True,
                padding=(0, 1),
                expand=True,
            )
            table.add_column("Magnitud", style="bold", width=8, justify="center")
            table.add_column("Descripción", style="italic dim", width=28)
            table.add_column("Veredicto", width=18, justify="center")
            table.add_column("RAM", width=14, justify="center")
            table.add_column("VRAM", width=14, justify="center")
            table.add_column("CPU Cores", width=14, justify="center")
            table.add_column("Storage", width=14, justify="center")
            table.add_column("Cuello de Botella", style="italic", width=36)

        # Preparar celdas de componentes
        details = result["details"]
        component_cells = []
        for comp in ("ram", "vram", "cores", "storage"):
            d = details[comp]
            if d["min"] == 0 and d["ideal"] == 0:
                component_cells.append(Text("N/A", style="dim"))
            else:
                component_cells.append(_styled_status(d["status"]))

        # Texto del cuello de botella
        bottleneck_text = result["bottleneck"]
        if result["overall_status"] == "INSUFICIENTE":
            bn_style = "bold red"
        elif result["overall_status"] == "SUFICIENTE":
            bn_style = "yellow"
        else:
            bn_style = "dim green"

        table.add_row(
            Text(mag, style="bold"),
            result["description"],
            _styled_status(result["overall_status"]),
            component_cells[0],
            component_cells[1],
            component_cells[2],
            component_cells[3],
            Text(bottleneck_text, style=bn_style),
        )

    # Imprimir la última tabla
    if current_task is not None:
        console.print(table)


def render_detail_panels(results: list[dict], hw: dict) -> None:
    """Renderiza paneles de detalle con barras de progreso para cada tarea Alta."""
    high_results = [r for r in results if r["magnitude"] == "Alta"]

    panels = []
    for result in high_results:
        task_color = TASK_COLORS.get(result["task"], "white")
        detail_table = Table(
            show_header=True, box=box.SIMPLE,
            padding=(0, 1),
        )
        detail_table.add_column("Componente", style="bold", width=16)
        detail_table.add_column("Actual", width=10, justify="right")
        detail_table.add_column("Mínimo", width=10, justify="right", style="dim")
        detail_table.add_column("Ideal", width=10, justify="right", style="dim")
        detail_table.add_column("Progreso vs Ideal", width=28)

        for comp in ("ram", "vram", "cores", "storage"):
            d = result["details"][comp]
            label = COMPONENT_LABELS[comp]
            unit = "GB" if comp != "cores" else ""

            if d["min"] == 0 and d["ideal"] == 0:
                continue  # Omitir componentes no relevantes

            detail_table.add_row(
                label,
                f"{d['actual']:.1f} {unit}".strip(),
                f"{d['min']} {unit}".strip(),
                f"{d['ideal']} {unit}".strip(),
                _progress_bar(d["actual"], d["ideal"]),
            )

        panel = Panel(
            detail_table,
            title=f"[bold {task_color}]{result['task']} — Carga Alta[/]",
            border_style=task_color,
            box=box.ROUNDED,
            width=82,
        )
        panels.append(panel)

    if panels:
        console.print()
        console.print(
            Panel(
                "[bold]📋 Detalle por Componente — Cargas Altas[/]",
                border_style="bright_white",
                box=box.DOUBLE_EDGE,
            )
        )
        for panel in panels:
            console.print(panel)


def render_footer() -> None:
    """Renderiza el pie del reporte."""
    now = datetime.now().strftime("%Y-%m-%d %H:%M:%S")
    footer = Text()
    footer.append(f"📅 Generado: {now}", style="dim")
    footer.append("  │  ", style="dim")
    footer.append("⚠️  ", style="bright_yellow")
    footer.append(
        "Este análisis se basa en umbrales lógicos, no en benchmarks de rendimiento real.",
        style="dim italic",
    )

    console.print()
    console.print(
        Panel(
            footer,
            border_style="dim",
            box=box.ROUNDED,
        )
    )


# ─── Main ────────────────────────────────────────────────────────────────────

def main() -> None:
    """Punto de entrada principal del analizador."""
    console.print()
    render_header()
    console.print()

    # Fase 1: Escaneo
    with console.status("[bold bright_cyan]Escaneando hardware...[/]", spinner="dots"):
        hw = scan_hardware()

    # Fase 2: Resumen de HW
    render_hardware_summary(hw)
    console.print()

    # Fase 3: Evaluación
    with console.status("[bold bright_cyan]Evaluando umbrales...[/]", spinner="dots"):
        results = evaluate(hw)

    # Fase 4: Tablas de evaluación
    render_evaluation(results, hw)

    # Fase 5: Detalle de cargas altas
    render_detail_panels(results, hw)

    # Fase 6: Pie
    render_footer()
    console.print()


if __name__ == "__main__":
    try:
        main()
    except KeyboardInterrupt:
        console.print("\n[dim]Análisis cancelado por el usuario.[/]")
        sys.exit(0)
