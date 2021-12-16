#[derive(Debug)]
enum Packet {
    Literal {
        version: u8,
        packet_type: u8,
        number: u64
    },
    Operator {
        version: u8,
        packet_type: u8,
        sub_packets: Vec<Packet>
    }
}

impl Packet {
    pub fn from(bits: Vec<bool>) -> (Self, Vec<bool>) {
        let version = bits_to_int(&bits[0..3].to_vec()) as u8;
        let packet_type = bits_to_int(&bits[3..6].to_vec()) as u8;
        match packet_type {
            4 => {
                let mut number_bits = vec![];
                let mut i = 6;
                while bits[i] {
                    number_bits.append(&mut bits[i + 1..i + 5].to_vec());
                    i += 5;
                }
                number_bits.append(&mut bits[i + 1..i + 5].to_vec());
                (Packet::Literal { version, packet_type, number: bits_to_int(&number_bits) }, bits[i + 5..].to_vec())
            },
            _ => {
                let type_id = bits[6];
                if type_id {
                    let sub_packet_no = bits_to_int(&bits[7..18].to_vec());
                    let mut sub_packet_bits = bits[18..].to_vec();
                    let mut sub_packets = vec![];
                    for _ in 0..sub_packet_no {
                        let (sub_packet, bits_left) = Packet::from(sub_packet_bits);
                        sub_packets.push(sub_packet);
                        sub_packet_bits = bits_left;
                    }

                    (Packet::Operator { version, packet_type, sub_packets }, sub_packet_bits)
                } else {
                    let sub_packet_length = bits_to_int(&bits[7..22].to_vec()) as usize;
                    let mut sub_packet_bits = bits[22..22 + sub_packet_length].to_vec();
                    let mut sub_packets = vec![];
                    while sub_packet_bits.iter().any(|&b| b) {
                        let (sub_packet, bits_left) = Packet::from(sub_packet_bits);
                        sub_packets.push(sub_packet);
                        sub_packet_bits = bits_left;
                    }

                    (Packet::Operator { version, packet_type, sub_packets }, bits[22 + sub_packet_length..].to_vec())
                }
            }
        }
    }
}

fn byte_to_bits(number: u8, total_bits: u8) -> Vec<bool> {
    let mut bits: Vec<_> = format!("{:b}", number).chars()
        .map(|c| c == '1')
        .collect();

    while bits.len() < total_bits as usize {
        bits.insert(0, false);
    }
    bits
}

fn bits_to_int(bits: &Vec<bool>) -> u64 {
    let mut number = 0;
    for (i, &bit) in bits.iter().enumerate() {
        if bit {
            number += 1 << (bits.len() - 1 - i)
        }
    }
    number
}

pub mod part1 {
    use crate::days::day16::{byte_to_bits, Packet};
    use crate::util::input_parser;

    pub fn start() -> u32 {
        let input = input_parser::parse_file_raw("inputs/day16.txt").unwrap();

        let mut bits = Vec::with_capacity(input.len() * 4);
        for i in (0..input.len() - 1).step_by(2) {
            bits.append(&mut byte_to_bits(u8::from_str_radix(&input[i..i+2], 16).unwrap(), 8));
        }

        let (root_packet, _) = Packet::from(bits);
        println!("{:?}", root_packet);
        version_number_sum(&root_packet)
    }

    fn version_number_sum(packet: &Packet) -> u32 {
        match packet {
            Packet::Literal { version, packet_type: _, number: _ } => *version as u32,
            Packet::Operator { version, packet_type: _, sub_packets } => *version as u32 + sub_packets.iter().map(|p| version_number_sum(p)).sum::<u32>()
        }
    }
}

pub mod part2 {
    use crate::days::day16::{byte_to_bits, Packet};
    use crate::util::input_parser;

    pub fn start() -> u64 {
        let input = input_parser::parse_file_raw("inputs/day16.txt").unwrap();

        let mut bits = Vec::with_capacity(input.len() * 4);
        for i in (0..input.len() - 1).step_by(2) {
            bits.append(&mut byte_to_bits(u8::from_str_radix(&input[i..i+2], 16).unwrap(), 8));
        }

        let (root_packet, _) = Packet::from(bits);
        println!("{:?}", root_packet);
        packet_expression(&root_packet)
    }

    fn packet_expression(packet: &Packet) -> u64 {
        match packet {
            Packet::Literal { version: _, packet_type: _, number} => *number,
            Packet::Operator { version: _, packet_type, sub_packets} => {
                let mapped = sub_packets.iter().map(|p| packet_expression(p));
                match packet_type {
                    0 => mapped.sum(),
                    1 => mapped.product(),
                    2 => mapped.min().unwrap(),
                    3 => mapped.max().unwrap(),
                    5 => {
                        let packets: Vec<_> = mapped.collect();
                        (packets[0] > packets[1]) as u64
                    },
                    6 => {
                        let packets: Vec<_> = mapped.collect();
                        (packets[0] < packets[1]) as u64
                    },
                    7 => {
                        let packets: Vec<_> = mapped.collect();
                        (packets[0] == packets[1]) as u64
                    }
                    _ => 0
                }
            }
        }
    }
}