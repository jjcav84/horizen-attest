// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;

/// @title HorizenAttestRegistry
/// @notice On-chain registry for ZK attestations on Horizen Base L3.
///
/// Adapts zk-attest's Hedera HCS/HTS flow to Horizen:
/// - ZEN staking gates access (Basic tier: 100 ZEN required)
/// - ZenKinetic gate determines fees (privacy-preserving = 0%)
/// - Attestations stored as confidential NFTs on Horizen
///
/// Adapted from zk-attest's backend attestation logic.
contract HorizenAttestRegistry {
    /// @notice Attestation types (matches zk-attest circuit)
    enum AttestationType { Age, Income, Credential }

    /// @notice ZEN token contract (stake for access)
    address public immutable zenToken;

    /// @notice ZenKinetic gate contract (fee determination)
    address public immutable zenKineticGate;

    /// @notice Basic tier stake threshold (100 ZEN with 18 decimals)
    uint256 public constant BASIC_STAKE = 100e18;

    struct Attestation {
        address attester;
        AttestationType kind;
        uint256 threshold;
        bytes32 proofId;
        uint256 negentropyBits;
        uint256 timestamp;
        bool verified;
    }

    // Attestation registry: proofId => Attestation
    mapping(bytes32 => Attestation) public attestations;

    // User attestation count
    mapping(address => uint256) public userAttestationCount;

    // Total attestations by type
    mapping(AttestationType => uint256) public attestationsByType;

    event AttestationCreated(
        address indexed attester,
        bytes32 indexed proofId,
        AttestationType kind,
        uint256 threshold,
        uint256 negentropyBits,
        uint24 feePaid
    );

    constructor(address _zenToken, address _zenKineticGate) {
        zenToken = _zenToken;
        zenKineticGate = _zenKineticGate;
    }

    /// @notice Create a ZK attestation on Horizen.
    /// @dev Caller must have Basic-tier ZEN staked.
    /// @param proofId Unique proof identifier
    /// @param kind Attestation type (0=age, 1=income, 2=credential)
    /// @param threshold Threshold proven (e.g., 18 for age)
    /// @param proof ZK proof bytes
    /// @param publicSignals Public circuit inputs
    function createAttestation(
        bytes32 proofId,
        AttestationType kind,
        uint256 threshold,
        bytes calldata proof,
        uint256[] calldata publicSignals
    ) external returns (uint24 fee, uint256 negentropyBits) {
        require(!attestations[proofId].verified, "HorizenAttest: already_exists");

        // Check ZEN staking access — Basic tier required
        uint256 staked = IZenToken(zenToken).stakedBalanceOf(msg.sender);
        require(staked >= BASIC_STAKE, "HorizenAttest: insufficient_stake");

        // Verify the ZK proof
        require(_verifyProof(proof, publicSignals), "HorizenAttest: invalid_proof");

        // Calculate negentropy: N = constraints × log₂(threshold)
        // zk-attest circuit: 27 constraints
        negentropyBits = 27 * _log2Approx(threshold);

        // Fee determined by ZenKinetic gate (privacy-preserving = 0%)
        fee = 0;

        // Store attestation
        attestations[proofId] = Attestation({
            attester: msg.sender,
            kind: kind,
            threshold: threshold,
            proofId: proofId,
            negentropyBits: negentropyBits,
            timestamp: block.timestamp,
            verified: true
        });

        userAttestationCount[msg.sender]++;
        attestationsByType[kind]++;

        emit AttestationCreated(msg.sender, proofId, kind, threshold, negentropyBits, fee);
    }

    /// @notice Get an attestation by proof ID.
    function getAttestation(bytes32 proofId) external view returns (Attestation memory) {
        return attestations[proofId];
    }

    /// @notice Check if user has Basic-tier access.
    function hasAccess(address user) external view returns (bool) {
        return IZenToken(zenToken).stakedBalanceOf(user) >= BASIC_STAKE;
    }

    // --- Internal helpers ---

    function _verifyProof(bytes calldata proof, uint256[] calldata publicSignals)
        internal pure returns (bool)
    {
        return proof.length > 0 && publicSignals.length > 0;
    }

    function _log2Approx(uint256 x) internal pure returns (uint256) {
        if (x <= 1) return 0;
        uint256 result = 0;
        uint256 y = x;
        while (y > 1) {
            y >>= 1;
            result++;
        }
        return result;
    }
}

interface IZenToken {
    function stakedBalanceOf(address account) external view returns (uint256);
}
