"""
rules.py — Configuración de umbrales (thresholds) de hardware.

Define los requerimientos mínimos e ideales para cada combinación
de tarea y magnitud de carga. Los valores están en GB (RAM, VRAM, Storage)
y en cantidad de núcleos físicos (cores).

Un valor de 0 significa que ese componente NO se evalúa para esa tarea.
"""

# Componentes críticos por tarea: si alguno de estos está en INSUFICIENTE,
# todo el veredicto de la tarea baja a INSUFICIENTE.
# Los componentes no listados aquí se reportan pero no bloquean.

THRESHOLDS: dict = {
    # =========================================================================
    #  IA GENERATIVA / MODELOS MULTI-AGENTE
    # =========================================================================
    ("IA Generativa", "Alta"): {
        "description": "Modelos multi-agente, fine-tuning, entrenamiento avanzado",
        "critical_components": ["ram", "vram", "cores"],
        "ram":     {"min": 16,  "ideal": 128},
        "vram":    {"min": 12,  "ideal": 24},
        "cores":   {"min": 8,   "ideal": 16},
        "storage": {"min": 100, "ideal": 500},
    },
    ("IA Generativa", "Media"): {
        "description": "Inferencias locales de modelos ~8B parámetros",
        "critical_components": ["ram", "vram"],
        "ram":     {"min": 16, "ideal": 32},
        "vram":    {"min": 8,  "ideal": 16},
        "cores":   {"min": 6,  "ideal": 12},
        "storage": {"min": 50, "ideal": 200},
    },
    ("IA Generativa", "Baja"): {
        "description": "Inferencias ligeras, modelos ≤3B cuantizados",
        "critical_components": ["ram"],
        "ram":     {"min": 8,  "ideal": 16},
        "vram":    {"min": 4,  "ideal": 8},
        "cores":   {"min": 4,  "ideal": 8},
        "storage": {"min": 30, "ideal": 100},
    },

    # =========================================================================
    #  DATA SCIENCE
    # =========================================================================
    ("Data Science", "Alta"): {
        "description": "Pipelines ETL masivos, datasets >100 GB, Spark/Dask",
        "critical_components": ["ram", "cores"],
        "ram":     {"min": 32,  "ideal": 64},
        "vram":    {"min": 4,   "ideal": 12},
        "cores":   {"min": 8,   "ideal": 16},
        "storage": {"min": 200, "ideal": 1000},
    },
    ("Data Science", "Media"): {
        "description": "Notebooks con pandas/sklearn, datasets medianos",
        "critical_components": ["ram", "cores"],
        "ram":     {"min": 16, "ideal": 32},
        "vram":    {"min": 2,  "ideal": 8},
        "cores":   {"min": 4,  "ideal": 8},
        "storage": {"min": 100, "ideal": 500},
    },
    ("Data Science", "Baja"): {
        "description": "Análisis exploratorio liviano, CSVs pequeños",
        "critical_components": ["ram"],
        "ram":     {"min": 8,  "ideal": 16},
        "vram":    {"min": 0,  "ideal": 4},
        "cores":   {"min": 2,  "ideal": 4},
        "storage": {"min": 50, "ideal": 200},
    },

    # =========================================================================
    #  BASES DE DATOS
    # =========================================================================
    ("Bases de Datos", "Alta"): {
        "description": "PostgreSQL/MySQL con millones de registros, réplicas",
        "critical_components": ["ram", "cores", "storage"],
        "ram":     {"min": 32,  "ideal": 64},
        "vram":    {"min": 0,   "ideal": 0},
        "cores":   {"min": 8,   "ideal": 16},
        "storage": {"min": 500, "ideal": 2000},
    },
    ("Bases de Datos", "Media"): {
        "description": "Bases de datos medianas, consultas complejas",
        "critical_components": ["ram", "cores", "storage"],
        "ram":     {"min": 16,  "ideal": 32},
        "vram":    {"min": 0,   "ideal": 0},
        "cores":   {"min": 4,   "ideal": 8},
        "storage": {"min": 200, "ideal": 500},
    },
    ("Bases de Datos", "Baja"): {
        "description": "SQLite local, bases de prueba/desarrollo",
        "critical_components": ["ram", "storage"],
        "ram":     {"min": 8,   "ideal": 16},
        "vram":    {"min": 0,   "ideal": 0},
        "cores":   {"min": 2,   "ideal": 4},
        "storage": {"min": 100, "ideal": 200},
    },

    # =========================================================================
    #  GAMING
    # =========================================================================
    ("Gaming", "Alta"): {
        "description": "AAA a 4K/Ultra, ray-tracing habilitado",
        "critical_components": ["ram", "vram", "cores"],
        "ram":     {"min": 16,  "ideal": 32},
        "vram":    {"min": 8,   "ideal": 12},
        "cores":   {"min": 6,   "ideal": 8},
        "storage": {"min": 100, "ideal": 500},
    },
    ("Gaming", "Media"): {
        "description": "Juegos AAA a 1080p/High, e-sports competitivo",
        "critical_components": ["ram", "vram"],
        "ram":     {"min": 8,  "ideal": 16},
        "vram":    {"min": 4,  "ideal": 8},
        "cores":   {"min": 4,  "ideal": 6},
        "storage": {"min": 50, "ideal": 200},
    },
    ("Gaming", "Baja"): {
        "description": "Juegos indie/retro, configuraciones mínimas",
        "critical_components": ["ram"],
        "ram":     {"min": 4,  "ideal": 8},
        "vram":    {"min": 2,  "ideal": 4},
        "cores":   {"min": 2,  "ideal": 4},
        "storage": {"min": 30, "ideal": 100},
    },

    # =========================================================================
    #  USO GENERAL
    # =========================================================================
    ("Uso General", "Alta"): {
        "description": "Multitarea pesada: IDEs, navegador con 50+ pestañas, VMs",
        "critical_components": ["ram", "cores"],
        "ram":     {"min": 8,   "ideal": 16},
        "vram":    {"min": 0,   "ideal": 0},
        "cores":   {"min": 4,   "ideal": 8},
        "storage": {"min": 100, "ideal": 500},
    },
    ("Uso General", "Media"): {
        "description": "Ofimática, navegación, streaming de vídeo",
        "critical_components": ["ram"],
        "ram":     {"min": 4,  "ideal": 8},
        "vram":    {"min": 0,  "ideal": 0},
        "cores":   {"min": 2,  "ideal": 4},
        "storage": {"min": 50, "ideal": 200},
    },
    ("Uso General", "Baja"): {
        "description": "Navegación básica, procesador de textos",
        "critical_components": ["ram"],
        "ram":     {"min": 2,  "ideal": 4},
        "vram":    {"min": 0,  "ideal": 0},
        "cores":   {"min": 1,  "ideal": 2},
        "storage": {"min": 30, "ideal": 100},
    },
}

# Orden canónico de tareas para la presentación en CLI
TASK_ORDER: list[str] = [
    "IA Generativa",
    "Data Science",
    "Bases de Datos",
    "Gaming",
    "Uso General",
]

MAGNITUDE_ORDER: list[str] = ["Alta", "Media", "Baja"]

# Nombres legibles de componentes (para los reportes)
COMPONENT_LABELS: dict[str, str] = {
    "ram":     "RAM",
    "vram":    "VRAM (GPU)",
    "cores":   "Núcleos CPU",
    "storage": "Almacenamiento",
}
