# nostrust

- run the relay with `cargo run --bin relay --target x86_64-fortanix-unknown-sgx`
- run the client with `cargo run --bin client --features untrusted`
- run the filerunner with `cargo run --bin filerunner --features untrusted`

## Overview
Nostrust is a project that aims to provide an attestable GDPR-compliant Nostr relay and supporting Nostr client through the use of Intel SGX enclaves. The relay runs in an SGX enclave, while the untrusted client and filerunner interact with the relay.

Users running the client can input six types of requests:
- **post** to post new content on the relay
- **follow** to subscribe to other users
- **unfollow** to unsubscribe to other users
- **get** to retreive the user’s feed based on their subscriptions
- **delete** to delete all posted content (GDPR deletion)
- **info** to retrieve information and an attestation measurement from the relay.

The relay verifies the client’s requests and processes them accordingly, sealing the user’s data in the enclave before storing it through the untrusted filerunner. The relay also provides an attestation measurement to the client to ensure the relay is running in an SGX enclave.