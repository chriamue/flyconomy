# Flyconomy ✈️💰

Elevate your strategic thinking! 🌍 Dive into the world of airline economics with **Flyconomy**.

[![Status](https://img.shields.io/badge/status-ideation-blue.svg)](https://github.com/chriamue/flyconomy)
[![Github Repo](https://img.shields.io/badge/github-repo-green.svg)](https://github.com/chriamue/flyconomy)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![codecov](https://codecov.io/gh/chriamue/flyconomy/branch/main/graph/badge.svg?token=WTP4TXR51N)](https://codecov.io/gh/chriamue/flyconomy)

Flyconomy is a captivating 2D economic simulation game where you step into the shoes of an airline mogul. Aim to outpace competitors by making savvy decisions, from flight planning to fleet management.

## Key Features 🚀

- **Real-world Airport Data**: Step into the aviation world with genuine airport data sourced from OpenStreetMap.
- **Route Mastery**: Chalk out the most profitable routes keeping in mind distance, fees, and passenger demand.
- **Manage Your Fleet**: Dive deep into fleet management. Choose aircrafts wisely considering costs, passenger capacity, and fuel efficiency.
- **Grow Your Empire**: Watch your profits soar and use them to broaden your horizons and stretch your airline network.
- **User-friendly Interface**: Dive right in with an intuitive UI, crafted meticulously with Bevy and EGUI.
- **Single-Player Experience**: Embark solo! But keep an eye out for multiplayer adventures in future updates.

## Master the Skies: How to Play 🎮

1. **Set Your Base**: Kickstart by picking a home airport and invest in your very first aircraft.
2. **Chart Your Path**: Plan routes by weighing airport distances, fees, and demand. Be the captain of your destiny!
3. **Stay Economically Aware**: Monitor fluctuating fuel prices. Sometimes, shorter routes are golden.
4. **Expand and Conquer**: Earnings in your pocket? Time to reinvest. More planes, more routes, more profits!

## Getting Started 🛠️

### Installation

1. Clone the repository:
   ```sh
   git clone https://github.com/chriamue/flyconomy.git
   ```

2. Enter the directory:
   ```sh
   cd flyconomy
   ```

3. Get airborne:
   ```sh
   cargo run
   ```

## Web App

1. Ready your code for the web:
    ```sh
    wasm-pack build --target web
    ```

2. Take off online:
    ```sh
    python3 -m http.server
    ```

3. Land safely on [Localhost](http://localhost:8000).

## Web3 Integration for Attractions: NFTs 🌐

Flyconomy goes beyond traditional gaming by integrating Non-Fungible Tokens (NFTs) for attractions. This allows players to truly own, update, and transfer parts of the game world.

### How does it work?

1. **NFTs as Attractions**: Each attraction in Flyconomy is represented as an NFT. Owning an NFT gives you the right to update its name, description, and GPS location.
2. **Minting New Attractions**: Players can mint new attractions up to a total of 1000. This means you can create and add your own landmarks to the game, making it a truly community-driven experience.
3. **Transferring Ownership**: After the initial 1000 attractions are minted, you'll need to acquire a token from another player to update an attraction. This ensures that the game world remains dynamic, with attractions potentially changing based on player trades and decisions.

🔗 **Mint or Update an Attraction**: [https://blog.chriamue.de/flyconomy/contracts-app/](https://blog.chriamue.de/flyconomy/contracts-app/)

## Be a Part of Our Journey: Contributing 🤝

Your insights can propel Flyconomy to greater heights. Feel free to join our open-source journey.

1. Fork the project.
2. Create your feature branch (`git checkout -b feature/YourFeatureName`).
3. Commit your insights (`git commit -m 'Add YourFeatureName'`).
4. Spread your wings (`git push origin feature/YourFeatureName`).
5. Open a pull request.

## License 📜

Fly under the MIT License. Get more details in [LICENSE](LICENSE).

## Connect with Us 💌

Direct Flight to the Project: [https://github.com/chriamue/flyconomy](https://github.com/chriamue/flyconomy)

## Acknowledgments

Kudos to the creative minds:

- The spinning earth, materials, and shading insights from [bevy_mod_paramap](https://github.com/nicopap/bevy_mod_paramap/blob/main/examples/earth3d.rs).
- A shoutout for the brilliant tutorial: [frederickjjoubert/bevy-ball-game](https://github.com/frederickjjoubert/bevy-ball-game/tree/Episode-10).
