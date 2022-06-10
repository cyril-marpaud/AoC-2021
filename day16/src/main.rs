use std::{self, fs::File, io::prelude::Read, path::Path};

use anyhow::{Context, Result};

const BIN: u32 = 2;
const HEX: u32 = 16;

#[derive(Debug)]
struct Packet {
	version: u8,
	type_id: PacketType,
}

impl Packet {
	const LTRL_GRP_LEN: usize = 4;
	const LTRL_LAST_GRP_STR: &'static str = "0";
	const LTRL_LAST_GRP_LEN: usize = Packet::LTRL_LAST_GRP_STR.len();
	const LTRL_TYPEID: u8 = 4;
	const TYPEID_BITS_LENGTH_BITS: usize = 15;
	const TYPEID_BITS_LENGTH_STR: &'static str = "0";
	const TYPEID_COUNT_LENGTH_BITS: usize = 11;
	const TYPEID_COUNT_LENGTH_STR: &'static str = "1";
	const TYPEID_LEN: usize = 3;
	const TYPEID_TYPE_LEN: usize = Packet::TYPEID_BITS_LENGTH_STR.len();
	const VERSION_LEN: usize = 3;

	fn get_version_sum(&self) -> u32 {
		let mut sum = self.version as u32;
		if let PacketType::Operation(subs) = &self.type_id {
			sum += subs.iter().fold(0, |acc, s| acc + s.get_version_sum());
		}
		sum
	}

	fn hex_to_bin(buffer: String) -> String {
		let mut bin = String::new();
		buffer.chars().for_each(|c| {
			bin.push_str(&format!("{:04b}", c.to_digit(HEX).unwrap()));
		});
		bin
	}

	fn parse(bits: &mut impl Iterator<Item = char>) -> (Packet, usize) {
		let mut buf = String::new();
		let mut pkt_len = 0;

		bits.by_ref()
			.take(Packet::VERSION_LEN)
			.for_each(|c| buf.push(c));
		pkt_len += Packet::VERSION_LEN;

		let version = u8::from_str_radix(&buf, BIN).unwrap();
		buf.clear();

		bits.by_ref()
			.take(Packet::TYPEID_LEN)
			.for_each(|c| buf.push(c));
		pkt_len += Packet::TYPEID_LEN;

		let type_id = u8::from_str_radix(&buf, BIN).unwrap();
		buf.clear();

		match type_id {
			Packet::LTRL_TYPEID => {
				let mut lit_buf = String::new();
				loop {
					bits.by_ref()
						.take(Packet::LTRL_LAST_GRP_LEN)
						.for_each(|c| buf.push(c));
					pkt_len += Packet::LTRL_LAST_GRP_LEN;

					let keep_going = buf != Packet::LTRL_LAST_GRP_STR;
					buf.clear();

					bits.by_ref()
						.take(Packet::LTRL_GRP_LEN)
						.for_each(|c| lit_buf.push(c));
					pkt_len += Packet::LTRL_GRP_LEN;

					if !keep_going {
						break;
					}
				}
				(
					Packet {
						version,
						type_id: PacketType::Literal(u64::from_str_radix(&lit_buf, BIN).unwrap()),
					},
					pkt_len,
				)
			}
			_ => {
				let mut subs = Vec::new();
				bits.by_ref()
					.take(Packet::TYPEID_TYPE_LEN)
					.for_each(|c| buf.push(c));
				pkt_len += Packet::TYPEID_TYPE_LEN;

				let typeid_type = buf.clone();
				buf.clear();

				let subs = match typeid_type.as_str() {
					Packet::TYPEID_BITS_LENGTH_STR => {
						bits.by_ref()
							.take(Packet::TYPEID_BITS_LENGTH_BITS)
							.for_each(|c| buf.push(c));
						pkt_len += Packet::TYPEID_BITS_LENGTH_BITS;

						let subs_len = usize::from_str_radix(&buf, BIN).unwrap();
						let mut parsed_len = 0usize;

						while parsed_len < subs_len {
							let (sub, sub_len) = Packet::parse(bits);
							subs.push(sub);
							parsed_len += sub_len;
						}
						pkt_len += parsed_len;
						assert_eq!(subs_len, parsed_len);

						subs
					}

					Packet::TYPEID_COUNT_LENGTH_STR => {
						bits.by_ref()
							.take(Packet::TYPEID_COUNT_LENGTH_BITS)
							.for_each(|c| buf.push(c));
						pkt_len += Packet::TYPEID_COUNT_LENGTH_BITS;

						let len = usize::from_str_radix(&buf, BIN).unwrap();
						(0..len).for_each(|_| {
							let (sub, sub_len) = Packet::parse(bits);
							subs.push(sub);
							pkt_len += sub_len;
						});
						subs
					}

					_ => panic!(),
				};
				(
					Packet {
						version,
						type_id: PacketType::Operation(subs),
					},
					pkt_len,
				)
			}
		}
	}
}

#[derive(Debug)]
enum PacketType {
	Literal(u64),
	Operation(Vec<Packet>),
}

fn main() -> Result<()> {
	let hex_input = get_input("input.txt")?;
	let bin_input = Packet::hex_to_bin(hex_input);
	let mut bits = bin_input.chars();

	let (packet, _) = Packet::parse(&mut bits);

	println!("answer: {}", packet.get_version_sum());

	Ok(())
}

fn get_input(filename: impl AsRef<Path>) -> Result<String> {
	let mut file = File::open(filename).with_context(|| "Can't open file")?;
	let mut buffer = String::new();
	file.read_to_string(&mut buffer)?;

	Ok(buffer)
}
