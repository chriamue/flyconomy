import Web3 from 'web3';
import dotenv from 'dotenv';
import fs from 'fs';

import FlyconomyAttractionsAbi from '../artifacts/contracts/FlyconomyAttractions.sol/FlyconomyAttractions.json';

// Load environment variables from .env file
dotenv.config();

const INFURA_API_KEY = process.env.INFURA_API_KEY || "";
const SEPOLIA_PRIVATE_KEY = process.env.SEPOLIA_PRIVATE_KEY || "";

// Load ABI
const abi = JSON.parse(fs.readFileSync('artifacts/contracts/FlyconomyAttractions.sol/FlyconomyAttractions.json', 'utf8'));

// Connect to Ethereum network
const web3 = new Web3(new Web3.providers.HttpProvider(`https://sepolia.infura.io/v3/${INFURA_API_KEY}`));

// The address of the deployed contract from .env file
const contractAddress = process.env.CONTRACT;

// Connect to the deployed contract
const flyconomyAttractions: any = new web3.eth.Contract(FlyconomyAttractionsAbi.abi, contractAddress);

// Set the default account to use for transactions
web3.eth.accounts.wallet.add("0x" + SEPOLIA_PRIVATE_KEY as string);

const wallet = web3.eth.accounts.wallet[0];

web3.eth.defaultAccount = wallet.address;
console.log(`Using account ${web3.eth.defaultAccount}`);

async function main() {
  const args = process.argv.slice(2);
  if (args.length !== 5) {
    console.error("Usage: script <tokenId> <name> <description> <lat> <lon>");
    return;
  }

  const [tokenId, name, description, lat, lon] = args;

  const gasPrice = await web3.eth.getGasPrice();
  let gasLimit = await flyconomyAttractions.methods.updateToken(tokenId, name, description, parseInt(lat, 10), parseInt(lon, 10)).estimateGas({ from: web3.eth.defaultAccount });

  const updateTx = {
    from: web3.eth.defaultAccount,
    to: contractAddress,
    data: flyconomyAttractions.methods.updateToken(tokenId, name, description, parseInt(lat, 10), parseInt(lon, 10)).encodeABI(),
    gasPrice,
    gasLimit,
  };

  // Sign the transaction locally
  const signedUpdateTx = await wallet.signTransaction(updateTx);
  // Send the signed transaction
  await web3.eth.sendSignedTransaction(signedUpdateTx.rawTransaction).on('receipt', console.log);

  console.log(`Updated token with ID ${tokenId} with values: Name: ${name}, Description: ${description}, Latitude: ${lat}, Longitude: ${lon}`);
}

main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
