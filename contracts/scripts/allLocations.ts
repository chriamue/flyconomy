import { ethers } from "hardhat";
import dotenv from 'dotenv';
import { _dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h0acc0a280baf7611 } from '../../pkg/flyconomy_bg.wasm.d';
dotenv.config();

const contractAddress = process.env.CONTRACT || "";

async function main() {

    // Connect to the deployed contract
    const FlyconomyAttractions = await ethers.getContractFactory("FlyconomyAttractions");
    const flyconomyAttractions: any = FlyconomyAttractions.attach(contractAddress);

    console.log(`Using contract at ${contractAddress}`);

    console.log(await flyconomyAttractions.getAllLocations());

    // Call the getAllLocations function
    const [ids, lats, lons] = await flyconomyAttractions.getAllLocations();

    // Log the locations
    console.log("All Locations:");
    for (let i = 0; i < ids.length; i++) {
        console.log(`ID: ${ids[i]}, Latitude: ${lats[i]}, Longitude: ${lons[i]}`);
    }
}

main().catch((error) => {
    console.error(error);
    process.exitCode = 1;
});
