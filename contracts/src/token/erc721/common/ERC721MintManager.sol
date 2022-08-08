// SPDX-License-Identifier: MIT
pragma solidity ^0.8.9;

import "./ERC721Tradable.sol";

error OnlyAuthorizedMinter();
error NonMatchingArgumentLengths();

/**
 * @title ERC721MintManager
 * @dev Contract for managing minters of the ERC721 token
 */
abstract contract ERC721MintManager is ERC721Tradable {
    event MinterPrivilegesChange(address indexed account, bool trusted);
    
    mapping(address => bool) private minters;

    /**
     * @dev Checks if the specified account is an authorized minter
     */
    function isMinter(address account) public view returns (bool) {
        return minters[account];
    }

    /**
     * @dev Mints a token to the specified address
     */
    function mintTo(address to) public onlyMinters returns(uint256 tokenId) {
        return _mintTo(to);
    }

    /**
     * @dev Mints a token with the specified ID to specified address
     */
    function mintById(address to, uint256 tokenId) public onlyMinters {
        _safeMint(to, tokenId);
    }

    /**
     * @dev Mints many tokens to many accounts
     * Can specify a different number of tokens for each account
     */
    function mintMany(address[] memory recipients, uint256[] memory tokenCounts) public onlyMinters {
        for(uint256 i = 0; i < recipients.length; i++) {
            uint256 cnt = tokenCounts[i];

            for(uint256 j = 0; j < cnt; j++) {
                _mintTo(recipients[i]);
            }
        }
    }

    /**
     * @notice Mints many tokens to many accounts
     * Can specify different tokenIds for each account
     */
    function mintManyByIds(address[] memory recipients, uint256[][] memory tokenIds) public onlyMinters {
        for(uint256 i = 0; i < recipients.length; i++) {
            uint256 cnt = tokenIds[i].length;

            for(uint256 j = 0; j < cnt; j++) {
                _safeMint(recipients[i], tokenIds[i][j]);
            }
        }
    }

    /**
     * @dev Allow or disallow the specified account to mint new tokens
     */
    function _setMinter(address account, bool trusted) internal {
        minters[account]= trusted;

        emit MinterPrivilegesChange(account, trusted);
    }

    /**
     * @dev Allow or disallow the specified accounts to mint new tokens
     */
    function _setMinters(address[] memory _minters, bool[] memory trusted) internal {
        if(_minters.length != trusted.length) {
            revert NonMatchingArgumentLengths();
        }

        for(uint256 i = 0; i < _minters.length; i++) {
            minters[_minters[i]] = trusted[i];
            emit MinterPrivilegesChange(_minters[i], trusted[i]);
        }
    }

    /**
    * @dev Throws if called by an account other than an authorized minter.
    */
    modifier onlyMinters() {
        if(!minters[_msgSender()]) {
            revert OnlyAuthorizedMinter();
        }
        _;
    }
}