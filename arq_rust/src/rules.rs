// ─────────────────────────────────────────────────────────────────────────────
// rules.rs — Configuración de umbrales (thresholds) de hardware.
//
// Define los requerimientos mínimos e ideales para cada combinación
// de tarea y magnitud de carga. Valores en GB (RAM, VRAM, Storage)
// y cantidad de núcleos físicos (cores). Un valor de 0 significa que
// ese componente NO se evalúa para esa tarea.
// ─────────────────────────────────────────────────────────────────────────────

/// Requisitos mínimos e ideales para un componente.
#[derive(Debug, Clone, Copy)]
pub struct ComponentThreshold {
    pub min: f64,
    pub ideal: f64,
}

/// Umbral completo para una combinación (tarea, magnitud).
#[derive(Debug, Clone)]
pub struct Threshold {
    pub task: &'static str,
    pub magnitude: &'static str,
    pub description: &'static str,
    pub critical_components: &'static [&'static str],
    pub ram: ComponentThreshold,
    pub vram: ComponentThreshold,
    pub cores: ComponentThreshold,
    pub storage: ComponentThreshold,
}

impl Threshold {
    /// Devuelve el ComponentThreshold correspondiente al nombre dado.
    pub fn get_component(&self, name: &str) -> Option<&ComponentThreshold> {
        match name {
            "ram" => Some(&self.ram),
            "vram" => Some(&self.vram),
            "cores" => Some(&self.cores),
            "storage" => Some(&self.storage),
            _ => None,
        }
    }
}

/// Nombre legible para cada componente.
pub fn component_label<'a>(name: &'a str) -> &'a str {
    match name {
        "ram" => "RAM",
        "vram" => "VRAM (GPU)",
        "cores" => "Núcleos CPU",
        "storage" => "Almacenamiento",
        _ => name,
    }
}

/// Unidad de medida para cada componente.
pub fn component_unit<'a>(name: &'a str) -> &'a str {
    match name {
        "cores" => "núcleos",
        _ => "GB",
    }
}

/// Orden canónico de tareas para la presentación.
pub const TASK_ORDER: &[&str] = &[
    "IA Generativa",
    "Data Science",
    "Bases de Datos",
    "Gaming",
    "Uso General",
];

/// Orden de magnitudes.
pub const MAGNITUDE_ORDER: &[&str] = &["Alta", "Media", "Baja"];

/// Nombres de componentes en orden de evaluación.
pub const COMPONENT_ORDER: &[&str] = &["ram", "vram", "cores", "storage"];

// ─────────────────────────────────────────────────────────────────────────────
//  THRESHOLDS — Las 15 combinaciones (tarea × magnitud)
// ─────────────────────────────────────────────────────────────────────────────

pub const THRESHOLDS: &[Threshold] = &[
    // ═══ IA GENERATIVA ═══
    Threshold {
        task: "IA Generativa",
        magnitude: "Alta",
        description: "Modelos multi-agente, fine-tuning, entrenamiento avanzado",
        critical_components: &["ram", "vram", "cores"],
        ram:     ComponentThreshold { min: 16.0,  ideal: 128.0 },
        vram:    ComponentThreshold { min: 12.0,  ideal: 24.0 },
        cores:   ComponentThreshold { min: 8.0,   ideal: 16.0 },
        storage: ComponentThreshold { min: 100.0, ideal: 500.0 },
    },
    Threshold {
        task: "IA Generativa",
        magnitude: "Media",
        description: "Inferencias locales de modelos ~8B parámetros",
        critical_components: &["ram", "vram"],
        ram:     ComponentThreshold { min: 16.0, ideal: 32.0 },
        vram:    ComponentThreshold { min: 8.0,  ideal: 16.0 },
        cores:   ComponentThreshold { min: 6.0,  ideal: 12.0 },
        storage: ComponentThreshold { min: 50.0, ideal: 200.0 },
    },
    Threshold {
        task: "IA Generativa",
        magnitude: "Baja",
        description: "Inferencias ligeras, modelos ≤3B cuantizados",
        critical_components: &["ram"],
        ram:     ComponentThreshold { min: 8.0,  ideal: 16.0 },
        vram:    ComponentThreshold { min: 4.0,  ideal: 8.0 },
        cores:   ComponentThreshold { min: 4.0,  ideal: 8.0 },
        storage: ComponentThreshold { min: 30.0, ideal: 100.0 },
    },

    // ═══ DATA SCIENCE ═══
    Threshold {
        task: "Data Science",
        magnitude: "Alta",
        description: "Pipelines ETL masivos, datasets >100 GB, Spark/Dask",
        critical_components: &["ram", "cores"],
        ram:     ComponentThreshold { min: 32.0,  ideal: 64.0 },
        vram:    ComponentThreshold { min: 4.0,   ideal: 12.0 },
        cores:   ComponentThreshold { min: 8.0,   ideal: 16.0 },
        storage: ComponentThreshold { min: 200.0, ideal: 1000.0 },
    },
    Threshold {
        task: "Data Science",
        magnitude: "Media",
        description: "Notebooks con pandas/sklearn, datasets medianos",
        critical_components: &["ram", "cores"],
        ram:     ComponentThreshold { min: 16.0,  ideal: 32.0 },
        vram:    ComponentThreshold { min: 2.0,   ideal: 8.0 },
        cores:   ComponentThreshold { min: 4.0,   ideal: 8.0 },
        storage: ComponentThreshold { min: 100.0, ideal: 500.0 },
    },
    Threshold {
        task: "Data Science",
        magnitude: "Baja",
        description: "Análisis exploratorio liviano, CSVs pequeños",
        critical_components: &["ram"],
        ram:     ComponentThreshold { min: 8.0,   ideal: 16.0 },
        vram:    ComponentThreshold { min: 0.0,   ideal: 4.0 },
        cores:   ComponentThreshold { min: 2.0,   ideal: 4.0 },
        storage: ComponentThreshold { min: 50.0,  ideal: 200.0 },
    },

    // ═══ BASES DE DATOS ═══
    Threshold {
        task: "Bases de Datos",
        magnitude: "Alta",
        description: "PostgreSQL/MySQL con millones de registros, réplicas",
        critical_components: &["ram", "cores", "storage"],
        ram:     ComponentThreshold { min: 32.0,  ideal: 64.0 },
        vram:    ComponentThreshold { min: 0.0,   ideal: 0.0 },
        cores:   ComponentThreshold { min: 8.0,   ideal: 16.0 },
        storage: ComponentThreshold { min: 500.0, ideal: 2000.0 },
    },
    Threshold {
        task: "Bases de Datos",
        magnitude: "Media",
        description: "Bases de datos medianas, consultas complejas",
        critical_components: &["ram", "cores", "storage"],
        ram:     ComponentThreshold { min: 16.0,  ideal: 32.0 },
        vram:    ComponentThreshold { min: 0.0,   ideal: 0.0 },
        cores:   ComponentThreshold { min: 4.0,   ideal: 8.0 },
        storage: ComponentThreshold { min: 200.0, ideal: 500.0 },
    },
    Threshold {
        task: "Bases de Datos",
        magnitude: "Baja",
        description: "SQLite local, bases de prueba/desarrollo",
        critical_components: &["ram", "storage"],
        ram:     ComponentThreshold { min: 8.0,   ideal: 16.0 },
        vram:    ComponentThreshold { min: 0.0,   ideal: 0.0 },
        cores:   ComponentThreshold { min: 2.0,   ideal: 4.0 },
        storage: ComponentThreshold { min: 100.0, ideal: 200.0 },
    },

    // ═══ GAMING ═══
    Threshold {
        task: "Gaming",
        magnitude: "Alta",
        description: "AAA a 4K/Ultra, ray-tracing habilitado",
        critical_components: &["ram", "vram", "cores"],
        ram:     ComponentThreshold { min: 16.0,  ideal: 32.0 },
        vram:    ComponentThreshold { min: 8.0,   ideal: 12.0 },
        cores:   ComponentThreshold { min: 6.0,   ideal: 8.0 },
        storage: ComponentThreshold { min: 100.0, ideal: 500.0 },
    },
    Threshold {
        task: "Gaming",
        magnitude: "Media",
        description: "Juegos AAA a 1080p/High, e-sports competitivo",
        critical_components: &["ram", "vram"],
        ram:     ComponentThreshold { min: 8.0,  ideal: 16.0 },
        vram:    ComponentThreshold { min: 4.0,  ideal: 8.0 },
        cores:   ComponentThreshold { min: 4.0,  ideal: 6.0 },
        storage: ComponentThreshold { min: 50.0, ideal: 200.0 },
    },
    Threshold {
        task: "Gaming",
        magnitude: "Baja",
        description: "Juegos indie/retro, configuraciones mínimas",
        critical_components: &["ram"],
        ram:     ComponentThreshold { min: 4.0,  ideal: 8.0 },
        vram:    ComponentThreshold { min: 2.0,  ideal: 4.0 },
        cores:   ComponentThreshold { min: 2.0,  ideal: 4.0 },
        storage: ComponentThreshold { min: 30.0, ideal: 100.0 },
    },

    // ═══ USO GENERAL ═══
    Threshold {
        task: "Uso General",
        magnitude: "Alta",
        description: "Multitarea pesada: IDEs, navegador 50+ pestañas, VMs",
        critical_components: &["ram", "cores"],
        ram:     ComponentThreshold { min: 8.0,   ideal: 16.0 },
        vram:    ComponentThreshold { min: 0.0,   ideal: 0.0 },
        cores:   ComponentThreshold { min: 4.0,   ideal: 8.0 },
        storage: ComponentThreshold { min: 100.0, ideal: 500.0 },
    },
    Threshold {
        task: "Uso General",
        magnitude: "Media",
        description: "Ofimática, navegación, streaming de vídeo",
        critical_components: &["ram"],
        ram:     ComponentThreshold { min: 4.0,  ideal: 8.0 },
        vram:    ComponentThreshold { min: 0.0,  ideal: 0.0 },
        cores:   ComponentThreshold { min: 2.0,  ideal: 4.0 },
        storage: ComponentThreshold { min: 50.0, ideal: 200.0 },
    },
    Threshold {
        task: "Uso General",
        magnitude: "Baja",
        description: "Navegación básica, procesador de textos",
        critical_components: &["ram"],
        ram:     ComponentThreshold { min: 2.0,  ideal: 4.0 },
        vram:    ComponentThreshold { min: 0.0,  ideal: 0.0 },
        cores:   ComponentThreshold { min: 1.0,  ideal: 2.0 },
        storage: ComponentThreshold { min: 30.0, ideal: 100.0 },
    },
];

/// Busca todos los thresholds para una tarea específica, en orden de magnitud.
pub fn thresholds_for_task(task: &str) -> Vec<&'static Threshold> {
    MAGNITUDE_ORDER
        .iter()
        .filter_map(|mag| {
            THRESHOLDS
                .iter()
                .find(|t| t.task == task && t.magnitude == *mag)
        })
        .collect()
}
