use secp256k1::{ecdsa::Signature, Message, PublicKey, Secp256k1, SecretKey};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Event {
    pub id: String,
    pub pubkey: String,
    pub created_at: usize,
    pub kind: usize,
    pub tags: Vec<Vec<String>>,
    pub content: String,
    pub sig: String,
}

impl Event {
    pub fn new(
        privkey: String,
        pubkey: String,
        kind: usize,
        tags: Vec<Vec<String>>,
        content: String,
    ) -> Self {
        let mut event = Self {
            id: "".to_string(), // will be computed later
            pubkey,
            created_at: 0, // FIXME: use chrono::Utc::now().timestamp() as u64,
            kind,
            tags,
            content,
            sig: "".to_string(),
        };

        // compute id of the event and sign it
        event.id = Self::compute_id(&event);
        event.sig = Self::sign(&event, privkey);

        event
    }

    fn compute_id(event: &Self) -> String {
        let serialized = serde_json::to_string(event).unwrap();
        let mut hasher = Sha256::new();
        hasher.update(serialized);

        hex::encode(hasher.finalize())
    }

    fn sign(event: &Self, private_key: String) -> String {
        let secp = Secp256k1::new();

        let decoded_id = hex::decode(event.id.clone()).unwrap();
        let decoded_private_key = hex::decode(private_key).unwrap();

        let message = Message::from_digest(decoded_id[..32].try_into().unwrap());
        let secret_key = SecretKey::from_slice(&decoded_private_key).unwrap();
        let signature = secp.sign_ecdsa(&message, &secret_key);

        hex::encode(signature.serialize_compact())
    }

    pub fn verify(&self) -> bool {
        let secp = Secp256k1::new();

        // recompute the id of the event
        let mut new_event = self.clone();
        new_event.sig = "".to_string();
        new_event.id = "".to_string();
        let computed_id = Self::compute_id(&new_event);

        let decoded_id = hex::decode(computed_id).unwrap();
        let decoded_pubkey = hex::decode(self.pubkey.clone()).unwrap();
        let decoded_sig = hex::decode(self.sig.clone()).unwrap();

        let message = Message::from_digest(decoded_id[..32].try_into().unwrap());
        let public_key = PublicKey::from_slice(&decoded_pubkey).unwrap();
        let signature = Signature::from_compact(&decoded_sig).unwrap();

        secp.verify_ecdsa(&message, &signature, &public_key).is_ok()
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_new_event_verify_fail_kind() {
        let seed: [u8; 32] = [
            0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e,
            0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c,
            0x1d, 0x1e, 0x1f, 0x20,
        ];

        let secp = Secp256k1::new();
        let privkey = SecretKey::from_slice(&seed).unwrap();
        let pubkey = PublicKey::from_secret_key(&secp, &privkey);

        let mut event = Event::new(
            hex::encode(privkey.secret_bytes()),
            hex::encode(pubkey.serialize()),
            0,
            vec![],
            "content".to_string(),
        );

        // modifying event kind to a deletion request (5) should be invalid
        event.kind = 5;

        assert!(!event.verify());
    }

    #[test]
    fn test_new_event_verify_fail_content() {
        let seed: [u8; 32] = [
            0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e,
            0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c,
            0x1d, 0x1e, 0x1f, 0x20,
        ];

        let secp = Secp256k1::new();
        let privkey = SecretKey::from_slice(&seed).unwrap();
        let pubkey = PublicKey::from_secret_key(&secp, &privkey);

        let mut event = Event::new(
            hex::encode(privkey.secret_bytes()),
            hex::encode(pubkey.serialize()),
            0,
            vec![],
            "content".to_string(),
        );

        // modifying the content of the event
        // should make the verification fail
        event.content = "lololol".to_owned();

        assert!(!event.verify());
    }

    #[test]
    fn test_new_event_verify_pass() {
        let seed: [u8; 32] = [
            0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e,
            0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c,
            0x1d, 0x1e, 0x1f, 0x20,
        ];

        let secp = Secp256k1::new();
        let privkey = SecretKey::from_slice(&seed).unwrap();
        let pubkey = PublicKey::from_secret_key(&secp, &privkey);

        let event = Event::new(
            hex::encode(privkey.secret_bytes()),
            hex::encode(pubkey.serialize()),
            0,
            vec![],
            "content".to_string(),
        );

        // No modifications to the event

        assert!(event.verify());
    }

    #[test]
    fn test_compute_id() {
        let event_one = Event {
            id: "id".to_string(),
            pubkey: "pubkey".to_string(),
            created_at: 0,
            kind: 0,
            tags: vec![],
            content: "content".to_string(),
            sig: "sig".to_string(),
        };

        let event_two_identical = Event {
            id: "id".to_string(),
            pubkey: "pubkey".to_string(),
            created_at: 0,
            kind: 0,
            tags: vec![],
            content: "content".to_string(),
            sig: "sig".to_string(),
        };

        let computed_id_one = Event::compute_id(&event_one);
        let computed_id_two_identical = Event::compute_id(&event_two_identical);

        assert_eq!(computed_id_one, computed_id_two_identical);
    }
}
