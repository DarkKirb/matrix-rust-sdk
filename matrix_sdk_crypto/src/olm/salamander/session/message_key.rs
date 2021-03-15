use zeroize::Zeroize;

use aes::Aes256;
use block_modes::{block_padding::Pkcs7, BlockMode, Cbc};
use hkdf::Hkdf;
use hmac::{Hmac, Mac, NewMac};
use sha2::Sha256;

use super::{messages::OlmMessage, RatchetPublicKey};

type Aes256Cbc = Cbc<Aes256, Pkcs7>;

struct Aes256Key([u8; 32]);
struct Aes256IV([u8; 16]);
struct HmacSha256Key([u8; 32]);

impl Aes256Key {
    fn to_bytes(self) -> [u8; 32] {
        self.0
    }
}

impl HmacSha256Key {
    fn to_bytes(self) -> [u8; 32] {
        self.0
    }
}

impl Aes256IV {
    fn to_bytes(self) -> [u8; 16] {
        self.0
    }
}

pub(super) struct MessageKey {
    key: [u8; 32],
    ratchet_key: RatchetPublicKey,
    index: u64,
}

impl MessageKey {
    pub fn new(key: [u8; 32], ratchet_key: RatchetPublicKey, index: u64) -> Self {
        Self {
            key,
            ratchet_key,
            index,
        }
    }

    fn construct_message(self, ciphertext: Vec<u8>) -> OlmMessage {
        OlmMessage::from_parts(self.ratchet_key, self.index, ciphertext)
    }

    fn expand_keys(&self) -> (Aes256Key, HmacSha256Key, Aes256IV) {
        #[derive(Clone, Zeroize)]
        struct ExpandedKeys([u8; 80]);

        impl Drop for ExpandedKeys {
            fn drop(&mut self) {
                self.0.zeroize();
            }
        }

        impl ExpandedKeys {
            const HMAC_INFO: &'static [u8] = b"OLM_KEYS";

            fn new(message_key: &MessageKey) -> Self {
                let mut expanded_keys = [0u8; 80];
                let hkdf: Hkdf<Sha256> = Hkdf::new(Some(&[0]), &message_key.key);
                hkdf.expand(Self::HMAC_INFO, &mut expanded_keys).unwrap();

                Self(expanded_keys)
            }

            fn split(self) -> (Aes256Key, HmacSha256Key, Aes256IV) {
                let mut aes_key = Aes256Key([0u8; 32]);
                let mut hmac_key = HmacSha256Key([0u8; 32]);
                let mut iv = Aes256IV([0u8; 16]);

                aes_key.0.copy_from_slice(&self.0[0..32]);
                hmac_key.0.copy_from_slice(&self.0[32..64]);
                iv.0.copy_from_slice(&self.0[64..80]);

                (aes_key, hmac_key, iv)
            }
        }

        let expanded_keys = ExpandedKeys::new(&self);
        expanded_keys.split()
    }

    pub fn decrypt(self, ciphertext: Vec<u8>) -> Vec<u8> {
        let (aes_key, _hmac_key, iv) = self.expand_keys();
        let cipher = Aes256Cbc::new_var(&aes_key.to_bytes(), &iv.to_bytes()).unwrap();
        let plaintext = cipher.decrypt_vec(&ciphertext).unwrap();

        plaintext
    }

    pub fn encrypt(self, plaintext: &[u8]) -> OlmMessage {
        let (aes_key, hmac_key, iv) = self.expand_keys();

        let cipher = Aes256Cbc::new_var(&aes_key.to_bytes(), &iv.to_bytes()).unwrap();

        let ciphertext = cipher.encrypt_vec(&plaintext);
        let mut message = self.construct_message(ciphertext);

        let mut hmac = Hmac::<Sha256>::new_varkey(&hmac_key.to_bytes()).unwrap();
        hmac.update(message.as_payload_bytes());

        let mac = hmac.finalize().into_bytes();
        message.append_mac(&mac);

        message
    }
}