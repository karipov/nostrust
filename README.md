# nostrust

run with `cargo run --target x86_64-fortanix-unknown-sgx`


## Steps

1. TCP server to listen / accept connections
    - make sure it works within SGC
    - loop listens to a single client
2. TCP server needs to understand NIP-01 (and all the NIPs in the future)
3. Find a client we can run on the terminal / or use curl requests


1. Running a proper nostr server

15min presentation, demo and discussion


To deal with:
- Try to get tinyhttp to work by adding multithreading fortanix
- Http parsing
- Relay implementation + Nip 09+11
- Encrypted databases: Sealing
- Attestation
- Modified client for http