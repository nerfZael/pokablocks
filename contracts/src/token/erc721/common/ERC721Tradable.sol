// SPDX-License-Identifier: MIT
pragma solidity 0.8.13;

import "openzeppelin-contracts/contracts/token/ERC721/ERC721.sol";
import "openzeppelin-contracts/contracts/token/ERC721/extensions/ERC721Enumerable.sol";
import "openzeppelin-contracts/contracts/token/ERC721/extensions/ERC721Burnable.sol";
import "openzeppelin-contracts/contracts/utils/Strings.sol";
import "../../../common/meta-transactions/ContextMixin.sol";
import "../../../common/meta-transactions/NativeMetaTransaction.sol";

/**
 * @title ERC721Tradable
 * @dev ERC721 enumerable and burnable token contract with meta-transactions support
 */
abstract contract ERC721Tradable is ERC721, ERC721Enumerable, ERC721Burnable, ContextMixin, NativeMetaTransaction {
    event BaseTokenURIChange(string prevURI, string newURI);
    event NextTokenIdChange(uint256 prevId, uint256 newId);
  
    /**
     * @dev A base URI for token metadata
     */ 
    string public baseTokenURI;
    /**
     * @dev The next ID to be assigned to a token when minting without specifying an ID
     */ 
    uint256 public nextTokenId;
    
    constructor(
        string memory _name,
        string memory _symbol
    ) ERC721(_name, _symbol) {
        _initializeEIP712(_name);
        nextTokenId = 1;
    }

    /**
     * @dev Get the token URI for a token ID
     * @param tokenId The ID of the token to get the URI for
     */
    function tokenURI(uint256 tokenId) override public view returns (string memory) {
        return string(abi.encodePacked(baseTokenURI, Strings.toString(tokenId)));
    }

    /**
     * @dev Check existance of a token by its ID
     * @param tokenId The ID of the token to check for existance
     */ 
    function exists(uint256 tokenId) public view returns (bool) {
        return _exists(tokenId);
    }

    /**
     * @dev See {IERC165-supportsInterface}.
     */
    function supportsInterface(bytes4 interfaceId)
        public
        view
        virtual
        override(ERC721, ERC721Enumerable )
        returns (bool) {
        return super.supportsInterface(interfaceId);
    }

    /**
     * @dev Set baseTokenURI that is the base URI for token metadata
     * @param uri the base URI for token metadata
     */
    function _setBaseTokenURI(string memory uri) internal {
        emit BaseTokenURIChange(baseTokenURI, uri);
        baseTokenURI = uri;
    }

    /**
     * @dev Set the next token ID to be assigned to a token when minting without specifying an ID
     * @param _nextTokenId the id of the next token to mint when minting
     */
    function _setNextTokenId(uint256 _nextTokenId) internal {
        emit NextTokenIdChange(nextTokenId, _nextTokenId);
        nextTokenId = _nextTokenId;
    }

    /**
     * @dev Mints a token to the specified address
     @param to the address to mint the token to
     */
    function _mintTo(address to) internal virtual returns(uint256 tokenId) {
        tokenId = nextTokenId;
        
        _safeMint(to, tokenId);
        nextTokenId++;

        return tokenId;
    }
    
    function _beforeTokenTransfer(
        address from,
        address to,
        uint256 tokenId
    ) internal virtual override(ERC721, ERC721Enumerable) {
        ERC721Enumerable._beforeTokenTransfer(from, to, tokenId);
    }
}