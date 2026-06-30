<p align="center">
  <a href="https://www.orkidlabs.com"><img src="assets/logo.png" alt="Orkid Labs" width="220" /></a>
</p>

# horizen-attest — Zero-Knowledge Attestations on Horizen Base L3

> *Prove credentials without revealing them. Pay zero fees for preserving
> privacy.*
>
> **By [Orkid Labs](https://www.orkidlabs.com)** — privacy-first crypto engineering

Horizen adaptation of [zk-attest](https://github.com/jjcav84/zk-attest) —
replaces Hedera HCS/HTS with ZEN token staking and
[ZenKinetic](https://github.com/jjcav84/zenkinetic) privacy gate
integration.

[![License: MIT](https://img.shields.io/badge/License-MIT-a78bfa.svg)](LICENSE)
[![Horizen](https://img.shields.io/badge/Horizen-Base%20L3-ff6b35.svg)](https://horizen.org)
[![negentropy](https://img.shields.io/badge/powered%20by-negentropy-a78bfa.svg)](https://github.com/jjcav84/negentropy)

## How it works

1. **Issuer** signs a credential commitment (same as zk-attest)
2. **User** generates a ZK proof (same circuit)
3. **ZenKinetic gate** scores the proof's negentropy — privacy-preserving = 0% fee
4. **ZEN staking** grants access — Basic tier (100 ZEN) for attestations
5. **Attestation settles** on Horizen Base L3 as a confidential NFT

## Quick Start

```rust
use horizen_attest::{AttestationSession, AttestationKind};

let session = AttestationSession::new(AttestationKind::Age, 18, 0.95, 100.0);
let result = session.evaluate();

println!("Gate: {:?}", result.gate_decision);    // Allow
println!("Fee: {} bps", result.fee_bps);          // 0
println!("Negentropy: {:.1} bits", result.negentropy_bits);
println!("Access: {}", result.access_granted);    // true
```

## Attestation Types

| Kind | Base Confidence | Use Case |
|------|----------------|----------|
| Age | 100 | Government ID-backed age |
| Income | 50 | Bank statement income proof |
| Credential | 80 | Authority-issued credentials |

## ZEN Token Utility

| Stake Tier | Min ZEN | Fee Discount | Attestation Access |
|-----------|---------|-------------|-------------------|
| Basic | 100 | 25% off | ✓ |
| Pro | 1,000 | 50% off | ✓ |
| Max | 10,000 | 75% off | ✓ |

## Architecture

```
horizen-attest
├── depends on → negentropy (physics scoring)
├── depends on → zenkinetic (privacy gate + ZEN staking)
├── adapts → zk-attest (ZK attestation circuit)
└── deploys on → Horizen Base L3
```

## Origin

This is the Horizen-native adaptation of [zk-attest](https://github.com/jjcav84/zk-attest).
The ZK circuit and proof generation are the same; the chain integration
changes from Hedera HCS/HTS to Horizen Base L3 with ZEN staking and
ZenKinetic privacy gating.

## Ecosystem

Part of the negentropy-powered privacy stack for Horizen:

- [negentropy](https://github.com/jjcav84/negentropy) — shared physics engine
- [zenkinetic](https://github.com/jjcav84/zenkinetic) — thermodynamic privacy gate
- [horizen-age](https://github.com/jjcav84/horizen-age) — age verification
- [horizen-attest](https://github.com/jjcav84/horizen-attest) — **this repo**
- [horizen-ballot](https://github.com/jjcav84/horizen-ballot) — anonymous voting

## About

Built by [Orkid Labs](https://www.orkidlabs.com) — a privacy-first crypto
engineering lab building thermodynamic infrastructure for decentralized
systems.

## License

MIT — see [LICENSE](LICENSE).
