
# Ibc_Token

## Description
Ibc_Token is a blockchain-based project focused on inter-blockchain communication (IBC) token management. It primarily uses Rust and TypeScript, leveraging the Anchor framework for development.

## Installation
To set up the project locally, follow these steps:

1. **Clone the repository:**
   ```sh
   git clone https://github.com/Spyder15/Ibc_Token.git
   cd Ibc_Token
   ```

2. **Install Rust:**
   Make sure you have Rust installed. If not, install it from [here](https://www.rust-lang.org/tools/install).

3. **Install Anchor CLI:**
   ```sh
   cargo install --git https://github.com/project-serum/anchor anchor-cli --locked
   ```

4. **Install Yarn:**
   Ensure you have Yarn installed. If not, install it from [here](https://classic.yarnpkg.com/en/docs/install).

5. **Install dependencies:**
   ```sh
   yarn install
   ```

6. **Build the project:**
   ```sh
   anchor build
   ```

## Usage
1. **Start the local validator:**
   ```sh
   solana-test-validator
   ```

2. **Deploy the program:**
   ```sh
   anchor deploy
   ```

3. **Run tests:**
   ```sh
   anchor test
   ```

## Contributing
Contributions are welcome! Please follow these steps:

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/YourFeature`)
3. Commit your changes (`git commit -m 'Add YourFeature'`)
4. Push to the branch (`git push origin feature/YourFeature`)
5. Open a pull request
---
