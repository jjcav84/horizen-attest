//! Types for Horizen attestations.

use serde::{Deserialize, Serialize};

/// Attestation kind — mirrors zk-attest's AttestationType.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AttestationKind {
    Age,
    Income,
    Credential,
}

impl AttestationKind {
    pub fn from_u64(v: u64) -> Option<Self> {
        match v {
            0 => Some(Self::Age),
            1 => Some(Self::Income),
            2 => Some(Self::Credential),
            _ => None,
        }
    }

    /// Base confidence depth — same as zk-attest's AttestationType::base_depth().
    pub fn base_depth(&self) -> f64 {
        match self {
            Self::Age => 100.0,
            Self::Income => 50.0,
            Self::Credential => 80.0,
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            Self::Age => "age",
            Self::Income => "income",
            Self::Credential => "credential",
        }
    }
}

/// Result of a Horizen attestation evaluation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttestationResult {
    /// ZenKinetic gate decision
    pub gate_decision: String,
    /// Fee in basis points (after ZEN stake discount)
    pub fee_bps: u32,
    /// Negentropy extracted by the proof (bits)
    pub negentropy_bits: f64,
    /// Privacy alignment score (0..1)
    pub alignment: f64,
    /// Committor probability (attestation validity confidence)
    pub committor: f64,
    /// ZEN stake tier
    pub stake_tier: String,
    /// Attestation kind
    pub kind: String,
    /// Whether ZEN staking grants access to attestations
    pub access_granted: bool,
}
