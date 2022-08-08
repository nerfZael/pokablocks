// SPDX-License-Identifier: MIT
pragma solidity 0.8.13;

import "./common/BaseERC721.sol";

contract PokablocksOrigin256 is BaseERC721 {
    constructor(
        address _owner, 
        string memory _name,
        string memory _symbol,
        string memory _baseURI
    ) BaseERC721(_owner, _name, _symbol, _baseURI) {
    }
}
