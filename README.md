# Quantum-Inspired Random Art Generator

This project implements a smart contract on the Internet Computer Protocol (ICP) blockchain that generates and stores unique pieces of "quantum-inspired" digital art.

## Overview

The Quantum-Inspired Random Art Generator is a Rust-based smart contract that allows users to create, store, and retrieve unique digital art pieces. Each art piece is generated using random numbers to simulate quantum-like properties such as superposition and entanglement.

### Key Features

- Generate unique art pieces using blockchain-based randomness
- Store art pieces on the ICP blockchain
- Retrieve individual art pieces or all created pieces
- Quantum-inspired art generation process

## Prerequisites

To work with this project, you'll need:

- Rust programming language
- Internet Computer SDK (dfx)
- A connection to the Internet Computer network

## Installation

1. Clone this repository:
   ```
   git clone https://github.com/Wamziss/random-art-generator.git
   cd random-art-generator
   ```

2. Start the project:
   ```
   dfx start --background
   ```

3. Deploy the canister:
   ```
   dfx deploy
   ```

## Usage

### Generating Art

To generate a new art piece, call the `generate_art` function:

```
dfx canister call quantum_art_generator generate_art
```

This will return the ID of the newly generated art piece.

### Retrieving Art

To retrieve a specific art piece by its ID:

```
dfx canister call quantum_art_generator get_art '(art_id)'
```

Replace `art_id` with the actual ID of the art piece.

To retrieve all art pieces:

```
dfx canister call quantum_art_generator get_all_art
```

## Project Structure

- `src/lib.rs`: The main smart contract code
- `Cargo.toml`: Rust dependencies and project configuration
- `dfx.json`: Internet Computer project configuration

## Contributing

Contributions to this project are welcome! Please fork the repository and submit a pull request with your changes.


## Acknowledgments

- This project is built on the Internet Computer Protocol
- Inspired by concepts from quantum mechanics and generative art

## Contact

For any questions or feedback, please open an issue in this repository or contact Hannah Mwangi at [hannahmwangi551@gmail.com].