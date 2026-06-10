# ⚡ Analizador Heurístico de Hardware

Herramienta local en Python que escanea los recursos físicos de tu equipo (CPU, RAM, GPU/VRAM, Almacenamiento) y evalúa su eficiencia para distintas cargas de trabajo según umbrales lógicos configurables.

> **No es un benchmark de estrés**, sino un evaluador de umbrales que identifica cuellos de botella.

---

## 📊 Cargas de trabajo evaluadas

| Tarea | Magnitudes | Componentes Críticos |
|---|---|---|
| **IA Generativa** | Alta · Media · Baja | RAM, VRAM, CPU |
| **Data Science** | Alta · Media · Baja | RAM, CPU |
| **Bases de Datos** | Alta · Media · Baja | RAM, CPU, Storage |
| **Gaming** | Alta · Media · Baja | RAM, VRAM, CPU |
| **Uso General** | Alta · Media · Baja | RAM, CPU |

## 🚀 Inicio rápido

```bash
# Clonar el repositorio
git clone https://github.com/tu-usuario/Arq_computer.git
cd Arq_computer

# Instalar y ejecutar (con Make)
make run

# O manualmente
python3 -m venv venv
source venv/bin/activate
pip install -r requirements.txt
python cli.py
```

## 📁 Estructura del proyecto

```
Arq_computer/
├── rules.py            # Umbrales de hardware (mínimos e ideales)
├── scanner.py          # Detección de CPU, RAM, GPU, Storage
├── engine.py           # Motor de evaluación (lógica de cuello de botella)
├── cli.py              # Interfaz de consola con Rich
├── requirements.txt    # Dependencias
├── Makefile            # Automatización de tareas
├── .gitignore          # Archivos excluidos de git
└── README.md           # Este archivo
```

## 🔧 Comandos del Makefile

| Comando | Descripción |
|---|---|
| `make help` | Muestra todos los comandos disponibles |
| `make install` | Crea el venv e instala las dependencias |
| `make run` | Ejecuta el analizador completo con reporte Rich |
| `make scan` | Ejecuta solo el escáner (salida JSON cruda) |
| `make lint` | Verifica la sintaxis de todos los módulos |
| `make clean` | Elimina el venv y archivos temporales |

## 🎯 Lógica de evaluación

El sistema usa el principio de **cuello de botella**:

- 🔴 **INSUFICIENTE** — El componente no alcanza el mínimo requerido
- 🟡 **SUFICIENTE** — Cumple el mínimo pero no alcanza el ideal
- 🟢 **ÓPTIMO** — Iguala o supera los valores ideales

El veredicto global de cada tarea es el **peor estado** entre sus componentes críticos. Un solo componente insuficiente hace que toda la tarea sea insuficiente.

## 📋 Dependencias

- [`psutil`](https://github.com/giampaolo/psutil) — RAM, Storage, CPU frequency
- [`py-cpuinfo`](https://github.com/workhorsy/py-cpuinfo) — CPU brand/model
- [`nvidia-ml-py`](https://pypi.org/project/nvidia-ml-py/) — GPU NVIDIA / VRAM
- [`rich`](https://github.com/Textualize/rich) — Interfaz de consola

> **Nota:** Si tu equipo no tiene GPU NVIDIA, el programa funciona normalmente mostrando VRAM como 0 GB.

## 📄 Licencia

MIT
