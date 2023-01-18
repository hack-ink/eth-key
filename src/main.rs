// std
use std::fmt::{Display, Formatter, Result as ResultFmt};
// crates.io
use anyhow::Result;
use secp256k1::{PublicKey, Secp256k1, SecretKey};
use tiny_keccak::{Hasher, Keccak};

#[derive(Debug)]
struct Key {
	address: Address,
	public_key: PublicKey,
	secret_key: SecretKey,
}
impl Display for Key {
	fn fmt(&self, f: &mut Formatter) -> ResultFmt {
		write!(
			f,
			"\
			address     = \"{}\"\n\
			public-key  = \"{}\"\n\
			secret-seed = \"{}\"\
			",
			self.address,
			array_bytes::bytes2hex("0x", &self.public_key.serialize()),
			array_bytes::bytes2hex("0x", self.secret_key.as_ref()),
		)
	}
}

#[derive(Debug, Default)]
struct Address([u8; 20]);
impl Display for Address {
	fn fmt(&self, f: &mut Formatter) -> ResultFmt {
		write!(f, "{}", array_bytes::bytes2hex("0x", &self.0))
	}
}

fn random() -> Key {
	let secp = Secp256k1::new();
	let (secret_key, public_key) = secp.generate_keypair(&mut rand::thread_rng());
	let mut hash = [0; 32];
	let mut hasher = Keccak::v256();

	hasher.update(&public_key.serialize_uncompressed()[1..]);
	hasher.finalize(&mut hash);

	let mut address = Address::default();

	address.0.copy_from_slice(&hash[12..]);

	Key { address, public_key, secret_key }
}

fn main() -> Result<()> {
	println!("{}", random());

	Ok(())
}
