// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "@openzeppelin/contracts/token/ERC721/extensions/ERC721Enumerable.sol";
import "@openzeppelin/contracts/access/Ownable.sol";
import "@openzeppelin/contracts/utils/Counters.sol";

contract FlyconomyAttractions is ERC721, ERC721Enumerable {
    struct Attraction {
        uint256 id;
        int32 lat; // multiplied by 10,000 for precision
        int32 lon; // multiplied by 10,000 for precision
        string name;
        string description;
    }

    using Counters for Counters.Counter;

    Counters.Counter private _tokenIdCounter;

    uint256 MAX_SUPPLY = 1000;

    mapping(uint256 => Attraction) public attractions;

    constructor() ERC721("Flyconomy Attractions", "FLYA") {}

    function mint(
        address to,
        string memory name,
        string memory description,
        int32 lat,
        int32 lon
    ) public {
        uint256 tokenId = _tokenIdCounter.current();
        require(tokenId <= MAX_SUPPLY, "Max supply reached");
        _tokenIdCounter.increment();

        Attraction memory newAttraction = Attraction({
            id: tokenId,
            lat: lat,
            lon: lon,
            name: name,
            description: description
        });
        attractions[tokenId] = newAttraction;

        _safeMint(to, tokenId);
    }

    function _beforeTokenTransfer(address from, address to, uint256 firstTokenId, uint256 batchSize) internal override(ERC721, ERC721Enumerable) {
        super._beforeTokenTransfer(from, to, firstTokenId, batchSize);
    }

    function supportsInterface(
        bytes4 interfaceId
    ) public view override(ERC721, ERC721Enumerable) returns (bool) {
        return super.supportsInterface(interfaceId);
    }

    function setLocation(uint256 id, int32 lat, int32 lon) public {
        require(ownerOf(id) == msg.sender, "You do not own this token");
        Attraction storage attraction = attractions[id];
        attraction.lat = lat;
        attraction.lon = lon;
    }

    function getLocation(uint256 id) public view returns (int32, int32) {
        Attraction storage attraction = attractions[id];
        return (attraction.lat, attraction.lon);
    }

    function setName(uint256 id, string memory name) public {
        require(ownerOf(id) == msg.sender, "You do not own this token");
        Attraction storage attraction = attractions[id];
        attraction.name = name;
    }

    function getName(uint256 id) public view returns (string memory) {
        Attraction storage attraction = attractions[id];
        return attraction.name;
    }

    function setDescription(uint256 id, string memory description) public {
        require(ownerOf(id) == msg.sender, "You do not own this token");
        Attraction storage attraction = attractions[id];
        attraction.description = description;
    }

    function getDescription(uint256 id) public view returns (string memory) {
        Attraction storage attraction = attractions[id];
        return attraction.description;
    }

    function updateToken(
        uint256 id,
        string memory name,
        string memory description,
        int32 lat,
        int32 lon
    ) public {
        require(ownerOf(id) == msg.sender, "You do not own this token");
        Attraction storage attraction = attractions[id];
        attraction.name = name;
        attraction.description = description;
        attraction.lat = lat;
        attraction.lon = lon;
    }

    function getAllLocations()
        public
        view
        returns (uint256[] memory, int32[] memory, int32[] memory)
    {
        uint256 total = totalSupply();
        uint256[] memory ids = new uint256[](total);
        int32[] memory lats = new int32[](total);
        int32[] memory lons = new int32[](total);
        for (uint256 i = 0; i < total; i++) {
            ids[i] = i;
            lats[i] = attractions[i].lat;
            lons[i] = attractions[i].lon;
        }
        return (ids, lats, lons);
    }
}
