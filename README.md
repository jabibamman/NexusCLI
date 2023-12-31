[![wakatime](https://wakatime.com/badge/user/e52bef9d-e298-4ffd-b606-f63f36526478/project/018b063f-0deb-4974-a627-800e4e8ba9a8.svg)](https://wakatime.com/badge/user/e52bef9d-e298-4ffd-b606-f63f36526478/project/018b063f-0deb-4974-a627-800e4e8ba9a8)

# NexusCLI

## Table of Contents

- [Objective](#objective)
- [Prerequisites](#prerequisites)
- [Installation](#installation)
- [Usage](#usage)
- [Contribution](#contribution)
- [License](#license)

## Objective

NexusCLI is a command-line tool written in Rust for managing operations on a Nexus repository. It provides functionalities like uploading (`U`) and deleting (`D`) files.

## Prerequisites

- Rust and Cargo installed
- Access to a Nexus repository

## Installation

Clone the repository:

```sh
git clone https://github.com/jabibamman/NexusCLI.git
cd NexusCLI
```

## Usage

### Configuration

Create a .env file at the root of the project and add the necessary environment variables:

```env
DOMAIN=your_domain
PROXY=your_proxy
```

### Build

To build the project, run:

```sh
cargo build --release
```

### Install as CLI

To use NexusCLI as a global CLI command, either add the binary path to your PATH or move the binary to a folder already in your PATH:

```sh
cp target/release/NexusCLI /usr/local/bin/nexus
```

### Commands

##### To upload a file:

```sh
nexus --operation U --repository depot-local --directory rp/omer/ihm/homere-DV05.zip --source /home/jabib/Downloads/homere-DV05.zip
nexus -o U -r depot-local -d rp/omer/ihm/homere-DV05.zip -s /home/jabib/Downloads/homere-DV05.zip
```

##### To delete a file:

```sh
nexus --operation D --repository depot-local --directory rp/omer/ihm/homere-DV05.zip
nexus -o D -r depot-local -d rp/omer/ihm/homere-DV05.zip
```

## Contribution

1. Clone the project
1. Create a new branch (git checkout -b feat/myNewFeature)
1. Make your changes
1. Commit your changes (git commit -am 'Add myNewFeature')
1. Push to the branch (git push origin feat/myNewFeature)
1. Open a Pull Request
1. Wait for approval
1. Happy coding!

## License

[MIT License](https://raw.githubusercontent.com/jabibamman/NexusCLI/master/LICENSE)
