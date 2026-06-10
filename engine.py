"""
engine.py — Motor de evaluación heurística.

Compara el hardware detectado contra los umbrales de rules.py y genera
un veredicto por cada combinación (tarea, magnitud) basado en el
principio de cuello de botella.
"""

from __future__ import annotations

from typing import Any

from rules import THRESHOLDS, TASK_ORDER, MAGNITUDE_ORDER, COMPONENT_LABELS


# Estados posibles, ordenados de peor a mejor
STATUS_LEVELS = {
    "INSUFICIENTE": 0,
    "SUFICIENTE": 1,
    "ÓPTIMO": 2,
}


def _evaluate_component(
    actual: float,
    min_val: float,
    ideal_val: float,
) -> str:
    """
    Evalúa un componente individual contra sus umbrales.

    Returns:
        "INSUFICIENTE" | "SUFICIENTE" | "ÓPTIMO"
    """
    if min_val == 0 and ideal_val == 0:
        # Componente no requerido para esta tarea → se considera óptimo
        return "ÓPTIMO"
    if actual < min_val:
        return "INSUFICIENTE"
    if actual >= ideal_val:
        return "ÓPTIMO"
    return "SUFICIENTE"


def _extract_actual_values(hardware: dict[str, Any]) -> dict[str, float]:
    """
    Extrae los valores numéricos relevantes del dict de hardware
    en un formato plano alineado con las claves de rules.py.
    """
    return {
        "ram":     hardware["ram"]["total_gb"],
        "vram":    hardware["gpu"]["vram_total_gb"],
        "cores":   hardware["cpu"]["physical_cores"],
        "storage": hardware["storage"]["free_gb"],
    }


def _format_bottleneck(
    component: str,
    actual: float,
    min_val: float,
    status: str,
) -> str:
    """Genera una explicación legible del cuello de botella."""
    label = COMPONENT_LABELS.get(component, component)
    unit = "GB" if component != "cores" else "núcleos"

    if status == "INSUFICIENTE":
        return f"{label}: {actual:.1f} {unit} < {min_val:.0f} {unit} mínimo"
    return ""


def evaluate(hardware: dict[str, Any]) -> list[dict[str, Any]]:
    """
    Evalúa el hardware contra todos los umbrales definidos en rules.py.

    Args:
        hardware: Diccionario generado por scanner.scan_hardware().

    Returns:
        Lista de resultados, uno por cada (tarea, magnitud).
    """
    actual = _extract_actual_values(hardware)
    results: list[dict[str, Any]] = []

    for task in TASK_ORDER:
        for mag in MAGNITUDE_ORDER:
            key = (task, mag)
            thresholds = THRESHOLDS.get(key)
            if thresholds is None:
                continue

            critical = thresholds["critical_components"]
            details: dict[str, dict[str, Any]] = {}
            bottlenecks: list[str] = []
            worst_status = "ÓPTIMO"

            for comp in ("ram", "vram", "cores", "storage"):
                comp_thresh = thresholds[comp]
                min_val = comp_thresh["min"]
                ideal_val = comp_thresh["ideal"]
                actual_val = actual[comp]

                status = _evaluate_component(actual_val, min_val, ideal_val)

                details[comp] = {
                    "status": status,
                    "actual": round(actual_val, 1),
                    "min": min_val,
                    "ideal": ideal_val,
                }

                # Solo los componentes críticos afectan el veredicto global
                if comp in critical:
                    if STATUS_LEVELS[status] < STATUS_LEVELS[worst_status]:
                        worst_status = status

                    # Registrar cuello de botella si es insuficiente
                    if status == "INSUFICIENTE":
                        explanation = _format_bottleneck(
                            comp, actual_val, min_val, status
                        )
                        if explanation:
                            bottlenecks.append(explanation)

            # Si no hay cuellos de botella explícitos pero el estado es
            # SUFICIENTE, indicamos qué componentes podrían mejorar
            if worst_status == "SUFICIENTE" and not bottlenecks:
                for comp in critical:
                    if details[comp]["status"] == "SUFICIENTE":
                        label = COMPONENT_LABELS.get(comp, comp)
                        ideal = details[comp]["ideal"]
                        act = details[comp]["actual"]
                        unit = "GB" if comp != "cores" else "núcleos"
                        bottlenecks.append(
                            f"{label}: {act:.1f} {unit} (ideal: {ideal} {unit})"
                        )

            results.append({
                "task": task,
                "magnitude": mag,
                "description": thresholds["description"],
                "overall_status": worst_status,
                "bottleneck": " | ".join(bottlenecks) if bottlenecks else "—",
                "details": details,
            })

    return results


# Permite ejecutar el motor de forma aislada para debug
if __name__ == "__main__":
    import json
    from scanner import scan_hardware

    hw = scan_hardware()
    results = evaluate(hw)
    print(json.dumps(results, indent=2, ensure_ascii=False))
