import { ethers } from "hardhat";

async function main() {

  const flyconomyAttractions = await ethers.deployContract("FlyconomyAttractions");

  await flyconomyAttractions.waitForDeployment();

  console.log(
    `FlyconomyAttractions deployed to ${flyconomyAttractions.target}`
  );
}

main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
