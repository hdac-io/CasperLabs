extern crate hex;

use alloc::string::String;

use secp256k1::{self, Message, PublicKey as Ver1PubKey, Signature};

pub fn signature_verification(
    ver1_pubkey_hex: String,
    message: String,
    signature_hex: String,
) -> bool {
    let ver1_pubkey_bytes = hex::decode(ver1_pubkey_hex).expect("Public key decode failed");
    let mut ver1_pubkey_byted_arr: [u8; 33] = [0u8; 33];
    ver1_pubkey_byted_arr.copy_from_slice(&ver1_pubkey_bytes.as_slice()[0..33]);
    let ver1_pubkey: Ver1PubKey = Ver1PubKey::parse_compressed(&ver1_pubkey_byted_arr)
        .expect("Invalid hex string of public key");

    // Message is already hashed. Don't have to hash again in here.
    let message_bytes = hex::decode(message).expect("Message decode failed");
    let mut hashed_msg: [u8; 32] = [0u8; 32];
    hashed_msg.copy_from_slice(&message_bytes);
    let message_struct = Message::parse(&hashed_msg);

    // 64-byted signature, not DER-encoded 71 byte
    let signature_vec = hex::decode(signature_hex).expect("Decode failed");
    let signature_byte: &[u8] = signature_vec.as_slice();
    let signature_obj = Signature::parse_slice(signature_byte).expect("Invalid signature");

    secp256k1::verify(&message_struct, &signature_obj, &ver1_pubkey)
}