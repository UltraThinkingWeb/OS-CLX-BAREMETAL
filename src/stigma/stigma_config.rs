// OS-CLX · kernel/src/stigma_config.rs
// Parser for stigmaconfig.ndb (@block { key: value } format)
// Native NDB format — no external dependencies

// NOTE: std::collections and std::fs are used; for bare-metal, replace with no_std alternatives or feature-gate

#[derive(Debug, Clone)]
pub struct StigmaConfig {
    pub meta: MetaBlock,
    pub stigma: StigmaBlock,
    pub nanogrid: NanogridBlock,
    pub resonance: ResonanceBlock,
    pub governance: GovernanceBlock,
}

#[derive(Debug, Clone, Default)]
pub struct MetaBlock {
    pub format_version: String,
    pub flow_version: String,
    pub created_at: String,
    pub meta_model: String,
}

#[derive(Debug, Clone)]
pub struct StigmaBlock {
    pub wave: String,
    pub weight: f64,
    pub momentum: f64,
    pub memory: f64,
    pub mode: String,
    pub meta_model: String,
    pub memory_decay: f64,
    pub momentum_factor: f64,
    pub weight_factor: f64,
}

impl Default for StigmaBlock {
    fn default() -> Self {
        Self {
            wave: "alpha".into(),
            weight: 0.85,
            momentum: 0.72,
            memory: 0.60,
            mode: "resonant".into(),
            meta_model: "wwwmmm-v2".into(),
            memory_decay: 0.92,
            momentum_factor: 1.40,
            weight_factor: 1.00,
        }
    }
}

#[derive(Debug, Clone)]
pub struct NanogridBlock {
    pub grid_id: String,
    pub cell_id: String,
    pub cluster_id: String,
    pub pressure_threshold: f64,
    pub resonance_ndb: f64,
    /// derived from [tide] section
    pub fluidity: f64,
    pub quality_decay: f64,
}

impl Default for NanogridBlock {
    fn default() -> Self {
        Self {
            grid_id: "ng-primary".into(),
            cell_id: "cell-001".into(),
            cluster_id: "clx-core".into(),
            pressure_threshold: 0.75,
            resonance_ndb: -24.5,
            fluidity: 0.85,
            quality_decay: 0.97,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ResonanceBlock {
    pub frequency_hz: f64,
    pub amplitude: f64,
    pub phase: f64,
    pub ndb: f64,
    pub freq_base: f64,
    pub amp_base: f64,
    pub phase_shift: f64,
}

impl Default for ResonanceBlock {
    fn default() -> Self {
        Self {
            frequency_hz: 440.0,
            amplitude: 0.90,
            phase: 0.0,
            ndb: -18.0,
            freq_base: 0.10,
            amp_base: 0.05,
            phase_shift: 0.02,
        }
    }
}

#[derive(Debug, Clone)]
pub struct GovernanceBlock {
    pub max_calls_per_sec: u64,
    pub success_threshold: f64,
    pub quarantine_on_fail: bool,
    pub parallelism: u32,
    pub worker_pool_size: u32,
}

impl Default for GovernanceBlock {
    fn default() -> Self {
        Self {
            max_calls_per_sec: 100_000,
            success_threshold: 0.999,
            quarantine_on_fail: true,
            parallelism: 512,
            worker_pool_size: 128,
        }
    }
}

impl Default for StigmaConfig {
    fn default() -> Self {
        Self {
            meta: MetaBlock::default(),
            stigma: StigmaBlock::default(),
            nanogrid: NanogridBlock::default(),
            resonance: ResonanceBlock::default(),
            governance: GovernanceBlock::default(),
        }
    }
}
