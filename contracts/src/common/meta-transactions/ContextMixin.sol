// SPDX-License-Identifier: MIT
pragma solidity 0.8.13;

abstract contract ContextMixin {
    /**
     * Return the sender of this call.
     * if the call came through a trusted forwarder (EIP-2771), return the original sender.
     * if the call came from the contract itself (EIP-712 meta transactions), return the original sender.
     * otherwise, return `msg.sender`.
     * should be used in the contract anywhere instead of msg.sender
     */
    function msgSender()
        internal
        view
        returns (address payable sender)
    {
        if (msg.sender == address(this)) {
            // If the sender is the contract itself, then it's using the EIP-712 meta transactions.
            bytes memory array = msg.data;
            uint256 index = msg.data.length;
            assembly {
                // Load the 32 bytes word from memory with the address on the lower 20 bytes, and mask those.
                sender := and(
                    mload(add(array, index)),
                    0xffffffffffffffffffffffffffffffffffffffff
                )
            }
        } else {
            sender = payable(msg.sender);
        }

        return sender;
    }
}
