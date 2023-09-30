// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "@openzeppelin/contracts/token/ERC721/extensions/ERC721Enumerable.sol";
import "@openzeppelin/contracts/access/Ownable.sol";

contract FlyconomyAttractions is ERC721Enumerable, Ownable {
    struct Attraction {
        uint256 id;
        int32 lat; // multiplied by 10,000 for precision
        int32 lon; // multiplied by 10,000 for precision
        string name;
        string description;
    }

    mapping(uint256 => Attraction) public attractions;

    constructor() ERC721("FlyconomyAttractions", "FLYA") {}

    function mint(address to) public onlyOwner returns (uint256) {
        uint256 newId = totalSupply();
        _mint(to, newId);
        Attraction memory newAttraction = Attraction({
            id: newId,
            lat: 0,
            lon: 0,
            name: "",
            description: ""
        });
        attractions[newId] = newAttraction;
        return newId;
    }

    function setLocation(uint256 id, int32 lat, int32 lon) public onlyOwner {
        Attraction storage attraction = attractions[id];
        attraction.lat = lat;
        attraction.lon = lon;
    }

    function getLocation(uint256 id) public view returns (int32, int32) {
        Attraction storage attraction = attractions[id];
        return (attraction.lat, attraction.lon);
    }

    function setName(uint256 id, string memory name) public onlyOwner {
        Attraction storage attraction = attractions[id];
        attraction.name = name;
    }

    function getName(uint256 id) public view returns (string memory) {
        Attraction storage attraction = attractions[id];
        return attraction.name;
    }

    function setDescription(uint256 id, string memory description) public onlyOwner {
        Attraction storage attraction = attractions[id];
        attraction.description = description;
    }

    function getDescription(uint256 id) public view returns (string memory) {
        Attraction storage attraction = attractions[id];
        return attraction.description;
    }

    function updateToken(uint256 id, string memory name, string memory description, int32 lat, int32 lon) public onlyOwner {
        Attraction storage attraction = attractions[id];
        attraction.name = name;
        attraction.description = description;
        attraction.lat = lat;
        attraction.lon = lon;
    }

    function getAllLocations() public view returns (uint256[] memory, int32[] memory, int32[] memory) {
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
