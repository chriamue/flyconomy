import {
    time,
    loadFixture,
} from "@nomicfoundation/hardhat-toolbox/network-helpers";
import { anyValue } from "@nomicfoundation/hardhat-chai-matchers/withArgs";
import { expect } from "chai";
import { ethers } from "hardhat";
import { ContractTransactionResponse } from "ethers";

describe("FlyconomyAttractions", function () {
    async function deployFixture() {
        const [owner, otherAccount] = await ethers.getSigners();

        const FlyconomyAttractions = await ethers.getContractFactory("FlyconomyAttractions");
        const flyconomyAttractions = await FlyconomyAttractions.deploy();

        return { flyconomyAttractions, owner, otherAccount };
    }

    describe("Deployment", function () {
        it("Should set the right owner", async function () {
            const { flyconomyAttractions, owner } = await loadFixture(deployFixture);

            expect(await flyconomyAttractions.owner()).to.equal(owner.address);
        });
    });

    it("Should mint a new token", async function () {
        const { flyconomyAttractions, owner } = await loadFixture(deployFixture);

        // Mint a new token.
        await flyconomyAttractions.mint(owner.address);

        expect(await flyconomyAttractions.totalSupply()).to.equal(1);
        expect(await flyconomyAttractions.ownerOf(0)).to.equal(owner.address);
    });

    it("Should set and get location", async function () {
        const { flyconomyAttractions, owner } = await loadFixture(deployFixture);

        await flyconomyAttractions.mint(owner.address);
        await flyconomyAttractions.setLocation(0, 40000, -74000);

        const [lat, lon] = await flyconomyAttractions.getLocation(0);
        expect(lat).to.equal(40000);
        expect(lon).to.equal(-74000);
    });

    it("Should set and get name", async function () {
        const { flyconomyAttractions, owner } = await loadFixture(deployFixture);

        await flyconomyAttractions.mint(owner.address);
        await flyconomyAttractions.setName(0, "Central Park");

        const name = await flyconomyAttractions.getName(0);
        expect(name).to.equal("Central Park");
    });

    it("Should set and get description", async function () {
        const { flyconomyAttractions, owner } = await loadFixture(deployFixture);

        await flyconomyAttractions.mint(owner.address);
        await flyconomyAttractions.setDescription(0, "A large city park in NYC");

        const description = await flyconomyAttractions.getDescription(0);
        expect(description).to.equal("A large city park in NYC");
    });

    it("Should update token", async function () {
        const { flyconomyAttractions, owner } = await loadFixture(deployFixture);

        await flyconomyAttractions.mint(owner.address);
        await flyconomyAttractions.updateToken(0, "Central Park", "A large city park in NYC", 40000, -74000);

        const name = await flyconomyAttractions.getName(0);
        const description = await flyconomyAttractions.getDescription(0);
        const [lat, lon] = await flyconomyAttractions.getLocation(0);

        expect(name).to.equal("Central Park");
        expect(description).to.equal("A large city park in NYC");
        expect(lat).to.equal(40000);
        expect(lon).to.equal(-74000);
    });
});
