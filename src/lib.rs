//! # horizen-attest — Zero-Knowledge Attestations on Horizen Base L3
//!
//! Horizen adaptation of [zk-attest](https://github.com/jjcav84/zk-attest) —
//! replaces Hedera HCS/HTS with ZEN token staking and ZenKinetic privacy
//! gate integration.
//!
//! ## How it works
//!
//! 1. **Issuer** signs a credential commitment (same as zk-attest)
//! 2. **User** generates a ZK proof (same circuit)
//! 3. **ZenKinetic gate** scores the proof's negentropy and determines fees
//! 4. **ZEN staking** grants access — Basic tier (100 ZEN) for attestations
//! 5. **Attestation settles** on Horizen Base L3 as a confidential NFT
//!
//! ## Quick Start
//!
//! ```rust
//! use horizen_attest::{AttestationSession, AttestationKind};
//!
//! let session = AttestationSession::new(AttestationKind::Age, 18, 0.95, 100.0);
//! let result = session.evaluate();
//! println!("Gate: {:?}", result.gate_decision);
//! println!("Negentropy: {:.1} bits", result.negentropy_bits);
//! ```

pub mod session;
pub mod types;

pub use session::AttestationSession;
pub use types::{AttestationKind, AttestationResult};

/// Re-export zenkinetic gate for direct access.
pub use zenkinetic::{PrivacyGate, TransactionProfile, GateDecision};
/// Re-export negentropy for scoring.
pub use negentropy::{Negentropy, RouteEnergy, Committor};
