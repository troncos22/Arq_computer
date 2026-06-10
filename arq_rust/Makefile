# ──────────────────────────────────────────────────────────────────────────────
#  Analizador Heurístico de Hardware (Rust) — Makefile
# ──────────────────────────────────────────────────────────────────────────────

.PHONY: help build run release clean

help: ## Muestra esta ayuda
	@echo ""
	@echo "  ⚡ Analizador Heurístico de Hardware (Rust)"
	@echo "  ───────────────────────────────────────────"
	@echo ""
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | \
		awk 'BEGIN {FS = ":.*?## "}; {printf "  \033[36m%-15s\033[0m %s\n", $$1, $$2}'
	@echo ""

build: ## Compila en modo debug
	cargo build

run: ## Compila y ejecuta el analizador
	cargo run

release: ## Compila en modo release (optimizado)
	cargo build --release
	@echo "✅ Binario optimizado en: target/release/arq_rust"

clean: ## Elimina artefactos de compilación
	cargo clean
	@echo "🧹 Limpieza completa"
