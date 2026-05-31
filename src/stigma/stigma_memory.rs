//! stigma/stigma_memory.rs — Stigma Memory System mbi NDB Fluid Mesh
//
// Ky modul modelon memory si "bias field" mbi NDB, me interferencë valësh

// use crate::ndb_mesh::{FluidMesh, NodeId};

#[derive(Clone, Debug)]
pub struct StigmaMemory {
    pub bias_field: f64,
    pub resonance: f64,
}

impl StigmaMemory {
    pub fn new() -> Self {
        StigmaMemory {
            bias_field: 0.0,
            resonance: 0.0,
        }
    }

    /// Përditëson bias field sipas NDB mesh (wave interference)
    pub fn update_from_mesh(&mut self /*, mesh: &FluidMesh */) {
        // TODO: Në version real, përdor interferencë të vlerave të node-ve
        // let sum: f64 = mesh.nodes.values().map(|n| n.state.value).sum();
        // let count = mesh.nodes.len() as f64;
        // self.bias_field = if count > 0.0 { sum / count } else { 0.0 };
        // Resonanca si variancë e vlerave
        // self.resonance = mesh.nodes.values().map(|n| (n.state.value - self.bias_field).powi(2)).sum::<f64>() / count.max(1.0);
    }
}

// TODO: Lidh me memory ops të kernelit, përdor bias/resonance për vendimmarrje memory
