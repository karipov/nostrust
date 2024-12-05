use secp256k1::{Secp256k1, SecretKey, PublicKey};
use rand::rngs::OsRng;
use rand::RngCore;

pub fn generate_keypair() -> (SecretKey, PublicKey) {
    let secp = Secp256k1::new();
    let mut rng = OsRng;
    let mut keybytes = [0u8; 32];
    rng.fill_bytes(&mut keybytes);
    let sk = SecretKey::from_slice(&keybytes).unwrap();
    let pk = PublicKey::from_secret_key(&secp, &sk);

    println!("Client Private Key: {}", hex::encode(sk.secret_bytes()));
    println!("Client Public Key: {}", hex::encode(pk.serialize()));

    (sk, pk)
}

// test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_keypair() {
        let (sk, pk) = generate_keypair();
        assert_eq!(sk, sk);
        assert_eq!(pk, pk);
    }

    #[test]
    fn test_priv_key_event_and_verify() {
        use core::event::Event;
        let (privkey, pubkey) = generate_keypair();

        let event = Event::new(
            hex::encode(privkey.secret_bytes()),
            hex::encode(pubkey.serialize()),
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
        let (privkey, pubkey) = generate_keypair();

        let mut event = Event::new(
            hex::encode(privkey.secret_bytes()),
            hex::encode(pubkey.serialize()),
            0,
            vec![],
            "content".to_string(),
        );

        print!("{:?}", event);

        // modifying event kind to a deletion request (5) should be invalid
        event.kind = 5;

        assert!(!event.verify());
    }
}