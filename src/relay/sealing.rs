// Code in this file is based on the `sealing` example from the `sgx-isa` crate.
// https://github.com/fortanix/rust-sgx

use aes_gcm::aead::{Aead, KeyInit};
use aes_gcm::{Aes128Gcm, Nonce};
use anyhow::Result;
use base64::{engine::general_purpose, Engine};
use rand::random;
use rand::Rng;
use serde::{Deserialize, Serialize};
use sgx_isa::{ErrorCode, Keyname, Keypolicy, Keyrequest, Report};

const SEAL_KEY_LABEL: [u8; 16] = [0; 16];

pub fn encrypt_string(key: &[u8; 16], plaintext: String) -> Result<String> {
    let cipher = Aes128Gcm::new_from_slice(key)?;
    let nonce: [u8; 12] = rand::thread_rng().gen();
    let nonce = Nonce::from_slice(&nonce);

    let ciphertext = cipher
        .encrypt(nonce, plaintext.as_bytes())
        .map_err(|_| anyhow::anyhow!("encryption failed"))?;

    let mut result = Vec::new();
    result.extend_from_slice(nonce);
    result.extend_from_slice(&ciphertext);
    Ok(general_purpose::STANDARD.encode(&result))
}

pub fn decrypt_string(key: &[u8; 16], ciphertext: String) -> Result<String> {
    let cipher = Aes128Gcm::new_from_slice(key)?;
    let ciphertext = general_purpose::STANDARD.decode(ciphertext.as_bytes())?;

    let nonce = Nonce::from_slice(&ciphertext[..12]);
    let plaintext = cipher
        .decrypt(nonce, &ciphertext[12..])
        .map_err(|_| anyhow::anyhow!("decryption failed"))?;

    Ok(String::from_utf8(plaintext)?)
}

/// Information about how the sealing key was derived. This
/// should be stored alongside the sealed data, so that the enclave
/// can rederive the same key later.
#[derive(Serialize, Deserialize)]
pub struct SealData {
    rand: [u8; 16],
    isvsvn: u16,
    cpusvn: [u8; 16],
}

/// Derive a sealing key for the current enclave given `label` and
/// `seal_data`.
fn egetkey(seal_data: &SealData) -> Result<[u8; 16], ErrorCode> {
    // Key ID is combined from fixed label and random data
    let mut keyid = [0; 32];
    {
        let (label_dst, rand_dst) = keyid.split_at_mut(16);
        label_dst.copy_from_slice(&SEAL_KEY_LABEL);
        rand_dst.copy_from_slice(&seal_data.rand);
    }

    Keyrequest {
        keyname: Keyname::Seal as _,
        keypolicy: Keypolicy::MRENCLAVE,
        isvsvn: seal_data.isvsvn,
        cpusvn: seal_data.cpusvn,
        attributemask: [!0; 2],
        keyid,
        miscmask: !0,
        ..Default::default()
    }
    .egetkey()
}

/// Get a key for sealing data.
///
/// The returned key may be used for authenticated encryption.
///
/// If you call `seal_key` at different places in your code to seal
/// different types of data, make sure to pass a different `label`.
/// The returned `SealData` should be stored alongside the
/// ciphertext to make sure the data can be unsealed again later.
pub fn seal_key() -> ([u8; 16], SealData) {
    let report = Report::for_self();
    let seal_data = SealData {
        // Generate fresh randomness for each sealing operation.
        rand: random(),
        // Copy the parameters of the current enclave into SealData.
        isvsvn: report.isvsvn,
        cpusvn: report.cpusvn,
    };

    // EGETKEY should never error here because we used the
    // information from `Report::for_self`.
    (egetkey(&seal_data).unwrap(), seal_data)
}

/// Get a key for unsealing data.
///
/// The returned key may be used for authenticated decryption.
///
/// Pass in the same `label` that was used to get the sealing key,
/// and pass in the `seal_data` that was returned when obtaining the
/// sealing key.
pub fn unseal_key(seal_data: SealData) -> Result<[u8; 16], ErrorCode> {
    // let report = Report::for_self();
    egetkey(&seal_data)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_seal_unseal() {
        let (seal_key, seal_data) = seal_key();
        let unseal_key = unseal_key(seal_data).unwrap();
        assert_eq!(seal_key, unseal_key);
    }
}
