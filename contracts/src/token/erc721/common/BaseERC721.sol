// SPDX-License-Identifier: MIT
pragma solidity 0.8.13;

import "./ERC721MintManager.sol";
import "../../../access/Ownable.sol";
import "../../../royalties/RoyaltyManager.sol";

contract BaseERC721 is ERC721MintManager, RoyaltyManager, Ownable {
    uint256 public currentTokenId;

    constructor(
        address _owner, 
        string memory _name,
        string memory _symbol,
        string memory _baseURI
    ) Ownable(_owner) ERC721Tradable(_name, _symbol) {
        _setBaseTokenURI(_baseURI);
    }

    function setBaseTokenURI(string memory uri) public onlyOwner {
        _setBaseTokenURI(uri);
    }

    /**
     * @dev Allow or disallow given account to mint new tokens
     */
    function setMinter(address account, bool trusted) public onlyOwner {
        _setMinter(account, trusted);
    }
  
    /**
    * @dev Allow or disallow given accounts to mint new tokens
    */
    function setMinters(address[] memory _minters, bool[] memory trusted) public onlyOwner {
        _setMinters(_minters, trusted);
    }

    /**
     * @dev Expose internal _setNextTokenId to the owner of the contract
     * @param _nextTokenId the id of the next token to mint when minting
     */
    function setNextTokenId(uint256 _nextTokenId) public onlyOwner {
        _setNextTokenId(_nextTokenId);
    }

    /**
     * @dev Expose internal _setRoyaltiesForAll to the owner of the contract
     */
    function setRoyaltiesForAll(address payable recipientAddress, uint96 percentageBasisPoints) public onlyOwner {
        _setRoyaltiesForAll(recipientAddress, percentageBasisPoints);
    }

    function supportsInterface(bytes4 interfaceId) public view override(ERC721Tradable, Ownable, RoyaltyManager) returns (bool) {
        return ERC721Tradable.supportsInterface(interfaceId) ||
            Ownable.supportsInterface(interfaceId) ||
            RoyaltyManager.supportsInterface(interfaceId);
    }

    /**
     * This is used instead of msg.sender as transactions won't be sent by the original token owner, but by OpenSea.
     */
    function _msgSender()
        internal
        override
        view
        returns (address sender)
    {
        return ContextMixin.msgSender();
    }
}