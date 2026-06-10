// ─────────────────────────────────────────────────────────────────────────────
// engine.rs — Motor de evaluación heurística.
// ─────────────────────────────────────────────────────────────────────────────

use crate::rules::{
    component_label, component_unit, ComponentThreshold,
    COMPONENT_ORDER, THRESHOLDS, TASK_ORDER, MAGNITUDE_ORDER,
};
use crate::scanner::HardwareInfo;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Status {
    Insuficiente = 0,
    Suficiente = 1,
    Optimo = 2,
}

impl Status {
    pub fn label(&self) -> &'static str {
        match self {
            Status::Insuficiente => "INSUFICIENTE",
            Status::Suficiente => "SUFICIENTE",
            Status::Optimo => "ÓPTIMO",
        }
    }
    pub fn emoji(&self) -> &'static str {
        match self {
            Status::Insuficiente => "🔴",
            Status::Suficiente => "🟡",
            Status::Optimo => "🟢",
        }
    }
}

impl std::fmt::Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.emoji(), self.label())
    }
}

#[derive(Debug, Clone)]
pub struct ComponentDetail {
    pub name: &'static str,
    pub status: Status,
    pub actual: f64,
    pub min: f64,
    pub ideal: f64,
}

#[derive(Debug, Clone)]
pub struct EvalResult {
    pub task: &'static str,
    pub magnitude: &'static str,
    pub description: &'static str,
    pub overall_status: Status,
    pub bottleneck: String,
    pub details: Vec<ComponentDetail>,
}

fn evaluate_component(actual: f64, thresh: &ComponentThreshold) -> Status {
    if thresh.min == 0.0 && thresh.ideal == 0.0 {
        return Status::Optimo;
    }
    if actual < thresh.min { return Status::Insuficiente; }
    if actual >= thresh.ideal { return Status::Optimo; }
    Status::Suficiente
}

pub fn evaluate(hw: &HardwareInfo) -> Vec<EvalResult> {
    let mut results = Vec::new();
    for task in TASK_ORDER {
        for mag in MAGNITUDE_ORDER {
            let threshold = match THRESHOLDS.iter().find(|t| t.task == *task && t.magnitude == *mag) {
                Some(t) => t,
                None => continue,
            };
            let mut details = Vec::new();
            let mut worst = Status::Optimo;
            let mut bns: Vec<String> = Vec::new();

            for &comp in COMPONENT_ORDER {
                let ct = match threshold.get_component(comp) { Some(t) => t, None => continue };
                let actual = hw.actual_value(comp);
                let status = evaluate_component(actual, ct);
                details.push(ComponentDetail { name: comp, status, actual, min: ct.min, ideal: ct.ideal });
                let is_crit = threshold.critical_components.contains(&comp);
                if is_crit {
                    if status < worst { worst = status; }
                    if status == Status::Insuficiente {
                        let l = component_label(comp);
                        let u = component_unit(comp);
                        bns.push(format!("{}: {:.1} {} < {} {} mín", l, actual, u, ct.min, u));
                    }
                }
            }
            if worst == Status::Suficiente && bns.is_empty() {
                for &comp in threshold.critical_components {
                    if let Some(d) = details.iter().find(|d| d.name == comp) {
                        if d.status == Status::Suficiente {
                            let l = component_label(comp);
                            let u = component_unit(comp);
                            bns.push(format!("{}: {:.1} {} (ideal: {} {})", l, d.actual, u, d.ideal, u));
                        }
                    }
                }
            }
            results.push(EvalResult {
                task, magnitude: mag, description: threshold.description,
                overall_status: worst,
                bottleneck: if bns.is_empty() { "—".into() } else { bns.join(" | ") },
                details,
            });
        }
    }
    results
}
