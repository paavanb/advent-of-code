use std::cmp;
use std::fs;

const DATA_FILE: &str = "data/bits_transmission.txt";

const SUM_TYPE: u8 = 0;
const PRODUCT_TYPE: u8 = 1;
const MIN_TYPE: u8 = 2;
const MAX_TYPE: u8 = 3;
const GREATER_THAN_TYPE: u8 = 5;
const LESS_THAN_TYPE: u8 = 6;
const EQUAL_TO_TYPE: u8 = 7;

#[derive(Debug)]
enum Packet {
    LiteralPacket {
        version: u8,
        type_id: u8,
        value: u64,
    },
    OperatorPacket {
        version: u8,
        type_id: u8,
        subpackets: Vec<Packet>,
    },
}

impl Packet {
    fn sum_versions(&self) -> u32 {
        let mut sum = 0;
        match self {
            Packet::LiteralPacket { version, .. } => {
                return sum + version.clone() as u32;
            }
            Packet::OperatorPacket {
                version,
                subpackets,
                ..
            } => {
                sum += version.clone() as u32;
                subpackets.into_iter().for_each(|subpacket| {
                    sum += subpacket.sum_versions();
                });

                return sum;
            }
        }
    }

    fn value(&self) -> u64 {
        match self {
            Packet::LiteralPacket { value, .. } => {
                value.clone().into()
            }
            Packet::OperatorPacket {
                type_id,
                subpackets,
                ..
            } => match type_id.clone() {
                SUM_TYPE => {
                    subpackets.into_iter().fold(0, |sum, p| sum + p.value())
                }
                PRODUCT_TYPE => {
                    subpackets
                        .into_iter()
                        .fold(1, |product, p| product * p.value())
                }
                MIN_TYPE => {
                    subpackets
                        .into_iter()
                        .fold(u64::MAX, |accum, p| cmp::min(accum, p.value()))
                }
                MAX_TYPE => {
                    subpackets
                        .into_iter()
                        .fold(0, |accum, p| cmp::max(accum, p.value()))
                }
                GREATER_THAN_TYPE => {
                    assert_eq!(subpackets.len(), 2);
                    if subpackets[0].value() > subpackets[1].value() {
                        1
                    } else {
                        0
                    }
                }
                LESS_THAN_TYPE => {
                    assert_eq!(subpackets.len(), 2);
                    if subpackets[0].value() < subpackets[1].value() {
                        1
                    } else {
                        0
                    }
                }
                EQUAL_TO_TYPE => {
                    assert_eq!(subpackets.len(), 2);
                    if subpackets[0].value() == subpackets[1].value() {
                        1
                    } else {
                        0
                    }
                }

                _ => panic!("Invalid type encountered: {}", type_id),
            },
        }
    }
}

fn main() {
    let hex = get_transmission(DATA_FILE);
    let (packet, _) = parse_packet(get_bits(hex.as_str()).as_str());
    println!("Version sum: {}", packet.sum_versions());
    println!("Value: {}", packet.value());
}

/// Parse a single packet from the binary string.
/// # Returns
/// A tuple with the packet that was parsed, and the next index at
/// which to continue parsing.
fn parse_packet(packet_bin: &str) -> (Packet, usize) {
    let version = bin_to_dec(&packet_bin[0..3]) as u8;
    let type_id = bin_to_dec(&packet_bin[3..6]) as u8;

    // Literal packet type
    if type_id == 4 {
        let (value, offset) = parse_literal_packet_value(&packet_bin[6..packet_bin.len()]);
        (
            Packet::LiteralPacket {
                version,
                type_id,
                value,
            },
            6 + offset,
        )
    } else {
        let length_type_id = &packet_bin[6..7];
        let mut subpackets = vec![];

        if length_type_id == "0" {
            let length = bin_to_dec(&packet_bin[7..22]) as usize;

            let mut offset = 22;
            let end = length + 22;
            while offset < end {
                let (packet, next_offset) = parse_packet(&packet_bin[offset..end]);
                offset += next_offset;
                subpackets.push(packet);
            }

            return (
                Packet::OperatorPacket {
                    version,
                    type_id,
                    subpackets,
                },
                offset,
            );
        } else {
            let num_packets = bin_to_dec(&packet_bin[7..18]);
            let mut offset = 18;
            for _ in 0..num_packets {
                let (packet, next_offset) = parse_packet(&packet_bin[offset..]);
                offset += next_offset;
                subpackets.push(packet);
            }

            return (
                Packet::OperatorPacket {
                    version,
                    type_id,
                    subpackets,
                },
                offset,
            );
        }
    }
}

/// Parse the literal packet value
/// # Returns
/// A tuple with the value, and the next index at which to continue parsing.
fn parse_literal_packet_value(value_bin: &str) -> (u64, usize) {
    let mut accum = 0;
    for i in (0..value_bin.len()).step_by(5) {
        accum <<= 4;
        accum += bin_to_dec(&value_bin[i + 1..i + 5]);
        if &value_bin[i..i + 1] == "0" {
            return (accum, i + 5);
        }
    }

    return (accum, value_bin.len());
}

fn bin_to_dec(bin: &str) -> u64 {
    bin.chars()
        .fold(0, |accum, bit| (accum * 2) + if bit == '1' { 1 } else { 0 })
}

fn get_bits(hex: &str) -> String {
    hex.chars()
        .map(to_binary)
        .fold(str::to_string(""), |mut all_bits, bin| {
            all_bits.push_str(&bin);
            all_bits
        })
}

fn to_binary(hex_char: char) -> String {
    let value = hex_char.to_digit(16).unwrap();
    let unpadded = format!("{:b}", value);
    // Left-pad with 0's to fill out 4 bits
    format!("{:0>4}", unpadded)
}

fn get_transmission(filename: &str) -> String {
    fs::read_to_string(filename)
        .expect("Something went wrong.")
        .trim_end()
        .to_owned()
}
