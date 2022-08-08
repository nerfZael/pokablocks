// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "forge-std/Script.sol";
import "../src/token/erc721/common/BaseERC721.sol";
import "../src/token/erc721/PokablocksOrigin256.sol";

contract DeployPokablocksOrigin256 is Script {
    function setUp() public {}

    function run() public {
        vm.startBroadcast();
            
        BaseERC721 nft = new PokablocksOrigin256(
            0xE9FFd2d4c1e3eAF13f6e17DfDCD615a66f357dF4, 
            "Pokablocks", 
            "Pokablocks", 
            "https://wrap.link/i/ens/pokablocks.eth/metadata?id="
        );

        vm.stopBroadcast();
    }
}
