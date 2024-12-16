# Nostrust

[[`Report`](/report.pdf)]

Nostrust is a project that aims to provide an attestable GDPR-compliant Nostr relay and supporting Nostr client through the use of Intel SGX enclaves. The relay runs in an SGX enclave, while the untrusted client and filerunner interact with the relay.

## Setup and Installation

You need an SGX-enabled machine to run Nostrust as intendended. Please consult Intel's [product list](https://www.intel.com/content/www/us/en/architecture-and-technology/software-guard-extensions-processors.html) for SGX supported hardware. SGX [Simulation Mode](https://www.intel.com/content/www/us/en/developer/articles/training/usage-of-simulation-mode-in-sgx-enhanced-application.html) is a viable path to running the Nostrust relay, though it will not attest to running on genuine SGX hardware. After obtaining this hardware, ensure that SGX functionality is enabled as part of a BIOS settings change.

Nostrust uses the [Fortanix Enclave Development Platform (EDP)](https://edp.fortanix.com/), which has an [installation guide](https://edp.fortanix.com/docs/installation/guide/). Note that EDP and Nostrust use Rust, for which installation is covered in the guide. After following these steps, you should see successful output from the EDP-provided `sgx-detect` command. We've additionally found the following installation guides helpful in debugging errors on our testing platform, though you may not need them: [[1]](https://docs.scrt.network/secret-network-documentation/infrastructure/running-a-node-validator/setting-up-a-node-validator/node-setup/install-sgx-1), [[2]](https://codentium.com/setting-up-intel-sgx/).

After `sgx-detect` outputs successfully, clone this repository and install all dependencies with `cargo install --path .` while within the project directory.

## Running Nostrust

After successfull installation, run the following:

- Run the relay with `cargo run --bin relay --target x86_64-fortanix-unknown-sgx`
- Run the client with `cargo run --bin client --features untrusted`
- Run the filerunner with `cargo run --bin filerunner --features untrusted`

## Client Commands

The client has four pre-configured users: `@komron`, `@prithvi`, `@alice` and `@bob`. Log in as any one of them.

Users running the client can input six types of requests:
- **post** to post new content on the relay
- **follow** to subscribe to other users
- **unfollow** to unsubscribe to other users
- **get** to retreive the user’s feed based on their subscriptions
- **delete** to delete all posted content (GDPR deletion)
- **info** to retrieve information and an attestation measurement from the relay.

The relay verifies the client’s requests and processes them accordingly, sealing the user’s data in the enclave before storing it through the untrusted filerunner. The relay also provides an attestation measurement to the client to ensure the relay is running in an SGX enclave.
