extern crate hex;

use alloc::{string::String, vec::Vec};
use ripemd160::Ripemd160;
use secp256k1::{self, Message, PublicKey as Ver1PubKey, Signature};
use sha2::{Digest, Sha256};

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

pub fn derive_ver1_address(ver1_pubkey_hex: String) -> String {
    let ver1_pubkey_bytes = hex::decode(ver1_pubkey_hex).expect("Public key decode failed");

    // hash160
    let mut sha256hasher = Sha256::new();
    sha256hasher.input(ver1_pubkey_bytes);
    let sha256res = sha256hasher.result();

    let mut ripemd160hasher = Ripemd160::new();
    ripemd160hasher.input(sha256res);
    let hash160res = ripemd160hasher.result();

    // payload
    let mut payload: Vec<u8> = Vec::new();
    payload.push(0x28);
    for item in hash160res.iter() {
        payload.push(*item);
    }

    // Hdac checksum
    // 1. checksum body
    let mut sha256hasher = Sha256::new();
    sha256hasher.input(payload.clone());
    let sha256res_for_checksum_1st = sha256hasher.result();
    let mut sha256hasher = Sha256::new();
    sha256hasher.input(sha256res_for_checksum_1st);
    let mut sha256res_for_checksum_2nd = sha256hasher.result();
    let (sha256res_for_checksum_sliced, _) = sha256res_for_checksum_2nd.split_at_mut(4);
    sha256res_for_checksum_sliced.reverse();

    // 2. 48444143
    let mut dummy_bytes = hex::decode("48444143").expect("Message decode failed");
    dummy_bytes.reverse();

    // 3. XOR
    let mut buffered: Vec<u8> = Vec::new();
    for idx in 0..4 {
        buffered.push(sha256res_for_checksum_sliced[idx] ^ dummy_bytes[idx]);
    }
    buffered.reverse();

    // 'H' + hash160 + Hdac ver1 checksum = 1 + 20 + 4 = 25 bytes
    let mut res: Vec<u8> = Vec::new();
    for item in payload.iter() {
        res.push(*item);
    }
    for item in buffered.iter() {
        res.push(*item);
    }

    bs58::encode(res).into_string()
}
