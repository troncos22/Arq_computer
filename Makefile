# ──────────────────────────────────────────────────────────────────────────────
#  Analizador Heurístico de Hardware — Makefile
# ──────────────────────────────────────────────────────────────────────────────

VENV      := venv
PYTHON    := $(VENV)/bin/python
PIP       := $(VENV)/bin/pip
SRC_FILES := rules.py scanner.py engine.py cli.py

.PHONY: help venv install run scan lint clean

# ─── Default target ──────────────────────────────────────────────────────────

help: ## Muestra esta ayuda
	@echo ""
	@echo "  ⚡ Analizador Heurístico de Hardware"
	@echo "  ────────────────────────────────────"
	@echo ""
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | \
		awk 'BEGIN {FS = ":.*?## "}; {printf "  \033[36m%-15s\033[0m %s\n", $$1, $$2}'
	@echo ""

# ─── Setup ───────────────────────────────────────────────────────────────────

venv: ## Crea el entorno virtual
	python3 -m venv $(VENV)
	@echo "✅ Entorno virtual creado en ./$(VENV)"

install: venv ## Instala las dependencias en el entorno virtual
	$(PIP) install --upgrade pip
	$(PIP) install -r requirements.txt
	@echo "✅ Dependencias instaladas"

# ─── Run ─────────────────────────────────────────────────────────────────────

run: install ## Ejecuta el analizador completo (CLI con reporte Rich)
	$(PYTHON) cli.py

scan: install ## Ejecuta solo el escáner de hardware (salida JSON)
	$(PYTHON) scanner.py

# ─── Quality ─────────────────────────────────────────────────────────────────

lint: install ## Ejecuta verificación de sintaxis con py_compile
	@echo "🔍 Verificando sintaxis..."
	@for f in $(SRC_FILES); do \
		$(PYTHON) -m py_compile $$f && echo "  ✓ $$f"; \
	done
	@echo "✅ Sin errores de sintaxis"

# ─── Cleanup ─────────────────────────────────────────────────────────────────

clean: ## Elimina el entorno virtual y archivos temporales
	rm -rf $(VENV)
	find . -type d -name "__pycache__" -exec rm -rf {} + 2>/dev/null || true
	find . -type f -name "*.pyc" -delete 2>/dev/null || true
	@echo "🧹 Limpieza completa"
