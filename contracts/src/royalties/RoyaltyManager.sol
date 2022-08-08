// SPDX-License-Identifier: MIT
pragma solidity 0.8.13;

import "./rarible/RoyaltiesV2.sol";
import "./rarible/LibPart.sol";
import "./rarible/LibRoyaltiesV2.sol";
import "./IERC2981.sol";

/**
 * @title RoyaltyManager
 * @dev Contract for managing Rarible and EIP2981 royalties
 */
abstract contract RoyaltyManager is RoyaltiesV2, IERC2981 {
    event RoyaltiesForAllSet(
        address prevRecipient,
        uint256 prevRoyalties,
        address newRecipient,
        uint256 newRoyalties
    );

    address payable public royaltyRecipient;
    uint256 public royaltyPercentageBasisPoints;

    bytes4 private constant _INTERFACE_ID_ERC2981 = 0x2a55205a;

    /**
     * @dev Sets royalties for all tokens to be paid to the given address
     */
    function _setRoyaltiesForAll(
        address payable recipientAddress,
        uint96 percentageBasisPoints
    ) internal {
        emit RoyaltiesForAllSet(
            royaltyRecipient,
            royaltyPercentageBasisPoints,
            recipientAddress,
            percentageBasisPoints
        );

        royaltyRecipient = recipientAddress;
        royaltyPercentageBasisPoints = percentageBasisPoints;
    }

    /**
     * @dev Retrieve royalties for a given token for Rarible
     */
    function getRaribleV2Royalties(uint256 id)
        external
        view
        override
        returns (LibPart.Part[] memory)
    {
        LibPart.Part[] memory _royalties = new LibPart.Part[](1);

        _royalties[0] = LibPart.Part(
            royaltyRecipient,
            uint96(royaltyPercentageBasisPoints)
        );

        return _royalties;
    }

    /**
     * @dev Retrieve royalty info using the ERC2981 standard
     */
    function royaltyInfo(uint256 _tokenId, uint256 _salePrice)
        external
        view
        override
        returns (address receiver, uint256 royaltyAmount)
    {
        if (royaltyRecipient == address(0)) {
            return (address(0), 0);
        }

        return (
            royaltyRecipient,
            (_salePrice * royaltyPercentageBasisPoints) / 10000
        );
    }

    function supportsInterface(bytes4 interfaceId)
        public
        view
        virtual
        override
        returns (bool)
    {
        return
            interfaceId == LibRoyaltiesV2._INTERFACE_ID_ROYALTIES ||
            interfaceId == _INTERFACE_ID_ERC2981;
    }
}
