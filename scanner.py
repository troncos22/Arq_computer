"""
scanner.py — Módulo de extracción de hardware.

Detecta CPU, RAM, GPU (NVIDIA) y almacenamiento del equipo local.
Maneja de forma segura la ausencia de GPU NVIDIA o drivers faltantes.
"""

from __future__ import annotations

import platform
from typing import Any

import psutil

try:
    import cpuinfo
except ImportError:
    cpuinfo = None  # type: ignore[assignment]


def _scan_cpu() -> dict[str, Any]:
    """Obtiene información del procesador."""
    info: dict[str, Any] = {
        "brand": "Desconocido",
        "physical_cores": psutil.cpu_count(logical=False) or 1,
        "logical_cores": psutil.cpu_count(logical=True) or 1,
        "frequency_mhz": 0.0,
        "arch": platform.machine(),
    }

    # Frecuencia vía psutil
    freq = psutil.cpu_freq()
    if freq:
        info["frequency_mhz"] = round(freq.max or freq.current, 1)

    # Marca/modelo vía py-cpuinfo (puede ser lento, ~1-2 s)
    if cpuinfo is not None:
        try:
            ci = cpuinfo.get_cpu_info()
            info["brand"] = ci.get("brand_raw", info["brand"])
            # py-cpuinfo reporta logical count; preferimos psutil para physical
        except Exception:
            pass  # Si falla cpuinfo, usamos lo de psutil

    return info


def _scan_ram() -> dict[str, Any]:
    """Obtiene información de la memoria RAM."""
    vm = psutil.virtual_memory()
    return {
        "total_gb": round(vm.total / (1024 ** 3), 1),
        "available_gb": round(vm.available / (1024 ** 3), 1),
        "percent_used": vm.percent,
    }


def _scan_gpu() -> dict[str, Any]:
    """
    Intenta detectar una GPU NVIDIA vía pynvml.

    Si no hay GPU NVIDIA, los drivers no están instalados, o pynvml
    no está disponible, devuelve un dict con available=False y valores en 0.
    """
    gpu_info: dict[str, Any] = {
        "available": False,
        "name": "No detectada",
        "vram_total_gb": 0.0,
        "vram_used_gb": 0.0,
        "vram_free_gb": 0.0,
        "driver_version": "N/A",
    }

    try:
        from pynvml import (
            nvmlInit,
            nvmlShutdown,
            nvmlDeviceGetHandleByIndex,
            nvmlDeviceGetName,
            nvmlDeviceGetMemoryInfo,
            nvmlSystemGetDriverVersion,
        )

        nvmlInit()
        try:
            handle = nvmlDeviceGetHandleByIndex(0)
            mem = nvmlDeviceGetMemoryInfo(handle)
            name_raw = nvmlDeviceGetName(handle)
            driver_raw = nvmlSystemGetDriverVersion()

            # pynvml puede devolver bytes o str según la versión
            name = name_raw.decode("utf-8") if isinstance(name_raw, bytes) else name_raw
            driver = driver_raw.decode("utf-8") if isinstance(driver_raw, bytes) else driver_raw

            gpu_info.update({
                "available": True,
                "name": name,
                "vram_total_gb": round(mem.total / (1024 ** 3), 1),
                "vram_used_gb": round(mem.used / (1024 ** 3), 1),
                "vram_free_gb": round(mem.free / (1024 ** 3), 1),
                "driver_version": driver,
            })
        finally:
            nvmlShutdown()

    except Exception:
        # Cualquier error (ImportError, NVMLError, etc.) → sin GPU
        pass

    return gpu_info


def _scan_storage() -> dict[str, Any]:
    """Obtiene información del disco principal (punto de montaje /)."""
    try:
        usage = psutil.disk_usage("/")
    except OSError:
        # Fallback para Windows
        usage = psutil.disk_usage("C:\\")

    return {
        "total_gb": round(usage.total / (1024 ** 3), 1),
        "used_gb": round(usage.used / (1024 ** 3), 1),
        "free_gb": round(usage.free / (1024 ** 3), 1),
        "percent_used": usage.percent,
    }


def scan_hardware() -> dict[str, Any]:
    """
    Ejecuta un escaneo completo del hardware y devuelve un diccionario
    consolidado con las secciones: cpu, ram, gpu, storage.
    """
    return {
        "cpu": _scan_cpu(),
        "ram": _scan_ram(),
        "gpu": _scan_gpu(),
        "storage": _scan_storage(),
    }


# Permite ejecutar el escáner de forma aislada para debug
if __name__ == "__main__":
    import json
    hw = scan_hardware()
    print(json.dumps(hw, indent=2, ensure_ascii=False))
