// SPDX-License-Identifier: MIT
pragma solidity 0.8.13;

import "openzeppelin-contracts/contracts/token/ERC721/extensions/ERC721Enumerable.sol";

/**
 * @title ERC721Enumerator
 * @dev Contract used for pagination of an ERC721Enumerable contract.
 */
contract ERC721Enumerator {
   
    function enumerateAllTokens(ERC721Enumerable token, uint256 start, uint256 count) public view returns(uint256[] memory ids, uint256 total)  {
        uint256 length = token.totalSupply();
        if(start + count > length) {
            count = length - start;
        }
        
        ids = new uint256[](count);

        for(uint256 i = 0; i < count; i++) {
            ids[i] = token.tokenByIndex(start + i);
        }

        return (ids, length);
    }

    function enumerateTokensOfOwner(ERC721Enumerable token, address owner, uint256 start, uint256 count) public view returns(uint256[] memory ids, uint256 total)  {
        uint256 length = token.balanceOf(owner);
        if(start + count > length) {
            count = length - start;
        }

        ids = new uint256[](count);

        for(uint256 i = 0; i < count; i++) {
            ids[i] = token.tokenOfOwnerByIndex(owner, start + i);
        }

        return (ids, length);
    }
}