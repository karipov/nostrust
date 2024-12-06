# nostrust

- run the relay with `cargo run --bin relay --target x86_64-fortanix-unknown-sgx`

- run the client with `cargo run --bin client --features client`


## Steps

1. TCP server to listen / accept connections
    - make sure it works within SGC
    - loop listens to a single client
2. TCP server needs to understand NIP-01 (and all the NIPs in the future)
3. Find a client we can run on the terminal / or use curl requests


1. Running a proper nostr server

15min presentation, demo and discussion

CAN KEEP KEYS IN MEMORY
  
To deal with:
- Try to get tinyhttp to work by adding multithreading fortanix
- Http parsing
- Relay implementation + Nip 09+11
- Encrypted databases: Sealing
- Attestation
- Modified client for http


structure (roughly) for nip-01:
- some kind of maybe terminal input or something (later step) to signify a client sending:
    - a new event (message)
    - a request (of events or subscriptions)
    - a close request to stop a previous subscription
- these should be generated using functions in client – generating events, requests, etc.
- the relay should have functions to handle all of these client messages and respond accordingly:
    - filtering etc to return requested events
        - (creating a subscription) if needed
    - accepting or denying event messages from the client (and storing them)
    - closing a subscription or changing a subscription attached to a subscription id
    - sending human-readable error messages or other notices to client (don't have to worry about how these are treated tho)

These messages are sent in the following formats:
By the client:
["EVENT", <event struct defined defined in event.rs as json>], used to publish events.
["REQ", <subscription_id>, <filters1>, <filters2>, ...], used to request events and subscribe to new updates.
 - a filtersx object is a json looking like this:
    {
    "ids": <a list of event ids>,
    "authors": <a list of lowercase pubkeys, the pubkey of an event must be one of these>,
    "kinds": <a list of a kind numbers>,
    "#<single-letter (a-zA-Z)>": <a list of tag values, for #e — a list of event ids, for #p — a list of pubkeys, etc.>,
    "since": <an integer unix timestamp in seconds. Events must have a created_at >= to this to pass>,
    "until": <an integer unix timestamp in seconds. Events must have a created_at <= to this to pass>,
    "limit": <maximum number of events relays SHOULD return in the initial query>
    }
["CLOSE", <subscription_id>], used to stop previous subscriptions.

By the relay:
["EVENT", <subscription_id>, <event JSON as defined above>], used to send events requested by clients.
["OK", <event_id>, <true|false>, <message>], used to indicate acceptance or denial of an EVENT message.
["EOSE", <subscription_id>], used to indicate the end of stored events and the beginning of events newly received in real-time.
["CLOSED", <subscription_id>, <message>], used to indicate that a subscription was ended on the server side.
["NOTICE", <message>], used to send human-readable error messages or other things to clients.

Encrypted database:
TBD

Nip-09:

- Event deletion requests are actually j defined as Events, but with kind = 5. Here's an example:
{
  "kind": 5,
  "pubkey": <32-bytes hex-encoded public key of the event creator>,
  "tags": [
    ["e", "dcd59..464a2"],
    ["e", "968c5..ad7a4"],
    ["a", "<kind>:<pubkey>:<d-identifier>"],
    ["k", "1"],
    ["k", "30023"]
  ],
  "content": "these posts were published by accident",
  // other fields...
}
- The k tag indicates the kind of the events being requested for deletion, and the event is identified by the `pubkey`
- Event should be deleted on our encrypted database
- Since it's sent as an event, an ok message from the relay can be sent back to the client to indicate acceptance or denial of the request
- "Publishing a deletion request event against a deletion request has no effect"

Nip-11:
When a relay receives an HTTP(s) request with an Accept header of application/nostr+json to a URI supporting WebSocket upgrades, they SHOULD return a document with the following structure:
{
  "name": <string identifying relay>,
  "description": <string with detailed information>,
  "banner": <a link to an image (e.g. in .jpg, or .png format)>,
  "icon": <a link to an icon (e.g. in .jpg, or .png format>,
  "pubkey": <administrative contact pubkey>,
  "contact": <administrative alternate contact>,
  "supported_nips": <a list of NIP numbers supported by the relay>,
  "software": <string identifying relay software URL>,
  "version": <string version identifier>
}
Any field may be omitted, and clients MUST ignore any additional fields they do not understand.


NOTE: handling users and their keys is not a concern for this project. We can assume that the client has a keypair and knows how to use it. The relay should not be concerned with the identity of the client, only the validity of the messages they send.