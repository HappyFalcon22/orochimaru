// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;
import './VRF.sol';

contract OrochiECVRF is VRF {
  function verifyProof(Proof memory proof, uint256 alpha) external view returns (uint256 output) {
    verifyVRFProof(
      proof.pk,
      proof.gamma,
      proof.c,
      proof.s,
      alpha,
      proof.uWitness,
      proof.cGammaWitness,
      proof.sHashWitness,
      proof.zInv
    );
    output = uint256(keccak256(abi.encode('Orochi Network', proof.gamma)));
  }
}