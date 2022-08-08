// SPDX-License-Identifier: MIT
pragma solidity 0.8.13;

import "./LibPart.sol";

interface RoyaltiesV2 {
    function getRaribleV2Royalties(uint256 id) external view returns (LibPart.Part[] memory);
}