use std::collections::HashMap;

use secp256k1::{Secp256k1, SecretKey, PublicKey};
use rand::rngs::OsRng;
use rand::RngCore;

#[derive(Debug, Clone)]
pub struct Credentials {
    pub private_key: SecretKey,
    pub public_key: PublicKey,
}

pub fn generate_keypair() -> Credentials {
    let secp = Secp256k1::new();
    let mut rng = OsRng;
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
    let user_ids = vec!["@komron", "@prithvi", "@kinan", "@alice", "@bob"];
    for user_id in user_ids {
        let kp = generate_keypair();
        users.insert(user_id.to_string(), kp);
    }
    users
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
}