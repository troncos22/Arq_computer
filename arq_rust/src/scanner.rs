// ─────────────────────────────────────────────────────────────────────────────
// scanner.rs — Módulo de extracción de hardware.
//
// Detecta CPU, RAM, GPU (NVIDIA) y almacenamiento del equipo local.
// Maneja de forma segura la ausencia de GPU NVIDIA o drivers faltantes.
// ─────────────────────────────────────────────────────────────────────────────

use sysinfo::{System, Disks};

/// Información del procesador.
#[derive(Debug)]
pub struct CpuInfo {
    pub brand: String,
    pub physical_cores: usize,
    pub logical_cores: usize,
    pub frequency_mhz: u64,
    pub arch: String,
}

/// Información de la memoria RAM.
#[derive(Debug)]
pub struct RamInfo {
    pub total_gb: f64,
    pub available_gb: f64,
    pub percent_used: f64,
}

/// Información de la GPU NVIDIA.
#[derive(Debug)]
pub struct GpuInfo {
    pub name: String,
    pub vram_total_gb: f64,
    pub vram_used_gb: f64,
    pub vram_free_gb: f64,
    pub driver_version: String,
}

/// Información del almacenamiento principal.
#[derive(Debug)]
pub struct StorageInfo {
    pub total_gb: f64,
    pub used_gb: f64,
    pub free_gb: f64,
    pub percent_used: f64,
}

/// Contenedor con todo el hardware detectado.
#[derive(Debug)]
pub struct HardwareInfo {
    pub cpu: CpuInfo,
    pub ram: RamInfo,
    pub gpu: Option<GpuInfo>,
    pub storage: StorageInfo,
}

impl HardwareInfo {
    /// Devuelve el valor actual para un componente por nombre.
    pub fn actual_value(&self, component: &str) -> f64 {
        match component {
            "ram" => self.ram.total_gb,
            "vram" => self.gpu.as_ref().map_or(0.0, |g| g.vram_total_gb),
            "cores" => self.cpu.physical_cores as f64,
            "storage" => self.storage.free_gb,
            _ => 0.0,
        }
    }
}

/// Escanea la CPU usando sysinfo.
fn scan_cpu(sys: &System) -> CpuInfo {
    let cpus = sys.cpus();
    let brand = cpus
        .first()
        .map(|c| c.brand().to_string())
        .unwrap_or_else(|| "Desconocido".to_string());

    let frequency_mhz = cpus
        .first()
        .map(|c| c.frequency())
        .unwrap_or(0);

    CpuInfo {
        brand,
        physical_cores: sys.physical_core_count().unwrap_or(1),
        logical_cores: cpus.len().max(1),
        frequency_mhz,
        arch: std::env::consts::ARCH.to_string(),
    }
}

/// Escanea la memoria RAM usando sysinfo.
fn scan_ram(sys: &System) -> RamInfo {
    let total = sys.total_memory() as f64;
    let available = sys.available_memory() as f64;
    let used = sys.used_memory() as f64;

    let total_gb = total / (1024.0 * 1024.0 * 1024.0);
    let available_gb = available / (1024.0 * 1024.0 * 1024.0);
    let percent_used = if total > 0.0 {
        (used / total) * 100.0
    } else {
        0.0
    };

    RamInfo {
        total_gb: (total_gb * 10.0).round() / 10.0,
        available_gb: (available_gb * 10.0).round() / 10.0,
        percent_used: (percent_used * 10.0).round() / 10.0,
    }
}

/// Intenta detectar una GPU NVIDIA vía nvml-wrapper.
/// Devuelve None si no hay GPU NVIDIA o si fallan los drivers.
fn scan_gpu() -> Option<GpuInfo> {
    use nvml_wrapper::Nvml;

    let nvml = match Nvml::init() {
        Ok(n) => n,
        Err(_) => return None,
    };

    let device = match nvml.device_by_index(0) {
        Ok(d) => d,
        Err(_) => return None,
    };

    let name = device.name().unwrap_or_else(|_| "NVIDIA GPU".to_string());

    let mem = match device.memory_info() {
        Ok(m) => m,
        Err(_) => return None,
    };

    let driver = nvml
        .sys_driver_version()
        .unwrap_or_else(|_| "N/A".to_string());

    let total_gb = mem.total as f64 / (1024.0 * 1024.0 * 1024.0);
    let used_gb = mem.used as f64 / (1024.0 * 1024.0 * 1024.0);
    let free_gb = mem.free as f64 / (1024.0 * 1024.0 * 1024.0);

    Some(GpuInfo {
        name,
        vram_total_gb: (total_gb * 10.0).round() / 10.0,
        vram_used_gb: (used_gb * 10.0).round() / 10.0,
        vram_free_gb: (free_gb * 10.0).round() / 10.0,
        driver_version: driver,
    })
}

/// Escanea el almacenamiento principal.
fn scan_storage() -> StorageInfo {
    let disks = Disks::new_with_refreshed_list();

    // Buscar el disco raíz (/) o el de mayor tamaño
    let root_disk = disks
        .iter()
        .find(|d| d.mount_point() == std::path::Path::new("/"))
        .or_else(|| disks.iter().max_by_key(|d| d.total_space()));

    match root_disk {
        Some(disk) => {
            let total = disk.total_space() as f64;
            let free = disk.available_space() as f64;
            let used = total - free;

            let total_gb = total / (1024.0 * 1024.0 * 1024.0);
            let free_gb = free / (1024.0 * 1024.0 * 1024.0);
            let used_gb = used / (1024.0 * 1024.0 * 1024.0);
            let percent = if total > 0.0 {
                (used / total) * 100.0
            } else {
                0.0
            };

            StorageInfo {
                total_gb: (total_gb * 10.0).round() / 10.0,
                used_gb: (used_gb * 10.0).round() / 10.0,
                free_gb: (free_gb * 10.0).round() / 10.0,
                percent_used: (percent * 10.0).round() / 10.0,
            }
        }
        None => StorageInfo {
            total_gb: 0.0,
            used_gb: 0.0,
            free_gb: 0.0,
            percent_used: 0.0,
        },
    }
}

/// Ejecuta un escaneo completo del hardware del sistema.
pub fn scan_hardware() -> HardwareInfo {
    let mut sys = System::new_all();
    sys.refresh_all();

    HardwareInfo {
        cpu: scan_cpu(&sys),
        ram: scan_ram(&sys),
        gpu: scan_gpu(),
        storage: scan_storage(),
    }
}
