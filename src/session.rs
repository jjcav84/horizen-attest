//! Attestation session — evaluates a ZK attestation through the ZenKinetic gate.

use crate::types::{AttestationKind, AttestationResult};
use zenkinetic::{PrivacyGate, TransactionProfile};
use zenkinetic::staking::StakeTier;

/// Configuration for a Horizen attestation session.
///
/// Adapts zk-attest's `AttestationPotential` to the Horizen ecosystem:
/// - ZEN staking replaces HBAR costs (HCS + HTS)
/// - ZenKinetic gate scores privacy alignment
/// - Negentropy scoring via negentropy crate (same formula)
#[derive(Debug, Clone)]
pub struct AttestationSession {
    /// Attestation kind (age, income, credential)
    pub kind: AttestationKind,
    /// Threshold to prove (e.g., 18 for age, 30000 for income)
    pub threshold: u64,
    /// Issuer trust score (0..1)
    pub issuer_trust: f64,
    /// ZEN tokens staked by the user
    pub zen_staked: f64,
    /// Attestation age in seconds (recency decay)
    pub attestation_age_secs: f64,
    /// Proof generation latency in ms
    pub proof_latency_ms: u64,
    /// Circuit constraint count (zk-attest default: 27)
    pub constraint_count: u64,
}

impl Default for AttestationSession {
    fn default() -> Self {
        Self {
            kind: AttestationKind::Age,
            threshold: 18,
            issuer_trust: 0.95,
            zen_staked: 100.0, // Basic tier
            attestation_age_secs: 0.0,
            proof_latency_ms: 800,
            constraint_count: 27, // zk-attest circuit: 27 constraints
        }
    }
}

impl AttestationSession {
    pub fn new(kind: AttestationKind, threshold: u64, issuer_trust: f64, zen_staked: f64) -> Self {
        assert!(issuer_trust.is_finite() && (0.0..=1.0).contains(&issuer_trust), "issuer_trust must be in [0,1]");
        assert!(zen_staked.is_finite() && zen_staked >= 0.0, "zen_staked must be non-negative and finite");
        Self {
            kind,
            threshold,
            issuer_trust,
            zen_staked,
            ..Default::default()
        }
    }

    /// Evaluate the attestation through the ZenKinetic privacy gate.
    ///
    /// This replaces zk-attest's `AttestationPotential::energy()` + Hedera
    /// HCS/HTS submission with a ZenKinetic gate evaluation.
    pub fn evaluate(&self) -> AttestationResult {
        // Build a ZenKinetic transaction profile for the attestation
        let profile = TransactionProfile {
            has_zk_proof: true,
            constraint_count: self.constraint_count,
            anonymity_set_bits: 0, // attestations don't have anonymity sets
            proof_age_secs: self.attestation_age_secs,
            proof_latency_ms: self.proof_latency_ms,
            verify_latency_ms: 27,
            zen_staked: self.zen_staked,
        };

        let gate = PrivacyGate::evaluate(&profile);

        // Check ZEN staking access — Basic tier required for attestations
        let stake_tier = StakeTier::from_staked(self.zen_staked);
        let access_granted = stake_tier.grants_confidential_transfer();

        // Negentropy: N = constraints × log₂(threshold)
        let negentropy_bits =
            negentropy::Negentropy::from_constraints(self.constraint_count, self.threshold).bits();

        AttestationResult {
            gate_decision: format!("{:?}", gate.decision),
            fee_bps: gate.discounted_fee_bps,
            negentropy_bits,
            alignment: gate.alignment,
            committor: gate.committor,
            stake_tier: stake_tier.label().to_string(),
            kind: self.kind.label().to_string(),
            access_granted,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_attestation_aligned() {
        let session = AttestationSession::default();
        let result = session.evaluate();

        assert_eq!(result.gate_decision, "Allow");
        assert!(result.negentropy_bits > 0.0);
        assert!(result.access_granted);
    }

    #[test]
    fn test_insufficient_stake_denied() {
        let session = AttestationSession {
            zen_staked: 50.0, // Below Basic tier
            ..Default::default()
        };
        let result = session.evaluate();

        assert!(!result.access_granted);
    }

    #[test]
    fn test_credential_attestation() {
        let session = AttestationSession::new(AttestationKind::Credential, 999, 0.95, 100.0);
        let result = session.evaluate();

        assert_eq!(result.kind, "credential");
        assert!(result.negentropy_bits > 0.0);
    }

    #[test]
    fn test_higher_threshold_more_negentropy() {
        let low = AttestationSession::new(AttestationKind::Income, 30_000, 0.8, 100.0).evaluate();
        let high = AttestationSession::new(AttestationKind::Income, 100_000, 0.8, 100.0).evaluate();

        assert!(high.negentropy_bits > low.negentropy_bits);
    }

    #[test]
    fn test_stale_attestation_decays() {
        let fresh = AttestationSession::default().evaluate();
        let stale = AttestationSession {
            attestation_age_secs: 7200.0,
            ..Default::default()
        }
        .evaluate();

        assert!(stale.alignment < fresh.alignment);
    }
}
