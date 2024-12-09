use std::collections::HashMap;

use rand::rngs::StdRng;
use rand::{RngCore, SeedableRng, distributions::Alphanumeric, Rng};
use secp256k1::{PublicKey, Secp256k1, SecretKey};

const SEED: u64 = 42;
const USER_IDS: [&str; 5] = ["@komron", "@prithvi", "@kinan", "@alice", "@bob"];

#[derive(Debug, Clone)]
pub struct Credentials {
    pub private_key: SecretKey,
    pub public_key: PublicKey,
}

pub fn generate_keypair() -> Credentials {
    let secp = Secp256k1::new();
    let mut rng = rand::thread_rng();
    let mut keybytes = [0u8; 32];
    rng.fill_bytes(&mut keybytes);
    let sk = SecretKey::from_slice(&keybytes).unwrap();
    let pk = PublicKey::from_secret_key(&secp, &sk);

    Credentials {
        private_key: sk,
        public_key: pk,
    }
}

pub fn generate_users() -> HashMap<String, Credentials> {
    let mut users = HashMap::new();
    for user_id in USER_IDS.iter() {
        let kp = generate_keypair();
        users.insert(user_id.to_string(), kp.clone());
    }
    users
}

pub fn generate_subscription_id() -> String {
    let subscription_id: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(64)
        .map(char::from)
        .collect();

    subscription_id
}

// test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_keypair() {
        let kp = generate_keypair();
        let sk = kp.private_key;
        let pk = kp.public_key;
        println!("Client Private Key: {}", hex::encode(sk.secret_bytes()));
        println!("Client Public Key: {}", hex::encode(pk.serialize()));
        assert_eq!(sk, sk);
        assert_eq!(pk, pk);
    }

    #[test]
    fn test_priv_key_event_and_verify() {
        use core::event::Event;
        let kp = generate_keypair();
        let sk = kp.private_key;
        let pk = kp.public_key;

        let event = Event::new(
            hex::encode(sk.secret_bytes()),
            hex::encode(pk.serialize()),
            0,
            vec![],
            "content".to_string(),
        );

        print!("{:?}", event);

        assert!(event.verify());
    }

    #[test]
    fn test_priv_key_generation_fail_verify() {
        use core::event::Event;
        let kp = generate_keypair();
        let sk = kp.private_key;
        let pk = kp.public_key;

        let mut event = Event::new(
            hex::encode(sk.secret_bytes()),
            hex::encode(pk.serialize()),
            0,
            vec![],
            "content".to_string(),
        );

        print!("{:?}", event);

        // modifying event kind to a deletion request (5) should be invalid
        event.kind = 5;

        assert!(!event.verify());
    }

    #[test]
    fn test_generate_users() {
        let users = generate_users();
        print!("{:?}", users);
        assert_eq!(users.len(), 5);
    }

    #[test]
    fn test_generate_subscription_id() {
        let id = generate_subscription_id();
        println!("Subscription ID: {}", id);
        assert!(id.len() <= 64);
    }
}
