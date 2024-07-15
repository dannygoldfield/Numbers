# Numbers Project

## Project Description
The Numbers project is an experimental and commercial art project that involves a live auction of  inscribing unique numbers onto Satoshis on the Bitcoin blockchain.

## Table of Contents
- [Project Description](#project-description)
- [Prerequisites](#prerequisites)
- [Installation Instructions](#installation-instructions)
- [Configuration](#configuration)
- [Usage](#usage)
- [Running Tests](#running-tests)
- [Contribution Guidelines](#contribution-guidelines)
- [Roadmap](#roadmap)
- [License](#license)
- [Acknowledgments](#acknowledgments)
- [Contact](#contact)

## Prerequisites
- Python 3.8 or higher
- Rust 1.50 or higher
- Bitcoin Core 0.21.0 or higher
- Git

## Installation Instructions
1. Clone the repository:
    ```bash
    git clone https://github.com/your-username/numbers-project.git
    ```

2. Navigate to the project directory:
    ```bash
    cd numbers-project
    ```

3. Update the package list:
    ```bash
    sudo apt update
    ```

4. Install Git:
    ```bash
    sudo apt install git
    ```

## Configuration
### Bitcoin Core Configuration
Ensure your `bitcoin.conf` file is configured correctly:
```conf
rpcuser=your_rpc_username
rpcpassword=your_rpc_password
server=1
txindex=1
rpcallowip=127.0.0.1

## Testing

### Python Tests
1. Navigate to the project directory:
   ```sh
   cd /Volumes/Konstantine/Code/Numbers
   ```
2. Ensure you are in your virtual environment:
   ```sh
   source venv/bin/activate
   ```
3. Run the tests:
   ```sh
   pytest tests
   ```

### Rust Tests
1. Navigate to the Rust project directory:
   ```sh
   cd /Volumes/Konstantine/Code/Numbers/rust_project
   ```
2. Run the tests:
   ```sh
   cargo test
   ```

### Node.js Tests
1. Navigate to the project directory:
   ```sh
   cd /Volumes/Konstantine/Code/Numbers
   ```
2. Run the tests:
   ```sh
   NODE_OPTIONS=--experimental-vm-modules npx mocha tests
   ```

