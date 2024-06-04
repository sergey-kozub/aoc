use std::char;
use std::fs;

#[derive(Debug)]
enum StreamItem {
    Sum,
    Product,
    Minimum,
    Maximum,
    Literal(u64),
    Greater,
    Less,
    Equal,
}

#[derive(Debug)]
struct Packet {
    version: u8,
    bits: usize,
    packets: u16,
    item: StreamItem,
}

#[derive(Debug)]
struct StreamIter<'a> {
    stream: &'a[u8],
    position: usize,
    current: u8,  // 4 bits
}

impl<'a> StreamIter<'a> {
    fn from(source: &'a str) -> StreamIter<'a> {
        StreamIter {
            stream: source.as_bytes(),
            position: 0,
            current: 0,
        }
    }

    fn next_bit(&mut self) -> Option<u8> {
        let bit = self.position % 4;
        if bit == 0 {
            let index = self.position / 4;
            if index == self.stream.len() { return None; }
            let ch = char::from_u32(self.stream[index] as u32)?;
            self.current = ch.to_digit(16)? as u8;
        }
        self.position += 1;
        Some((self.current >> (3 - bit)) & 1)
    }

    fn read(&mut self, bits: usize) -> Option<u64> {
        let mut acc: u64 = 0;
        for _ in 0..bits {
            acc = acc << 1 | self.next_bit()? as u64;
        }
        Some(acc)
    }

    fn read_literal(&mut self) -> Option<u64> {
        let mut acc: u64 = 0;
        loop {
            let part = self.read(5)?;
            acc = acc << 4 | part & 15;
            if part < 16 { break; }
        }
        Some(acc)
    }

    fn operator(&mut self, current: Packet) -> Option<u64> {
        let mut args: Vec<u64> = Vec::new();
        if current.packets != 0 {
            for _ in 0..current.packets {
                args.push(self.process()?);
            }
        } else {
            let start = self.position - 22;  // ignore header
            while self.position - start < current.bits {
                args.push(self.process()?);
            }
        }
        match current.item {
            StreamItem::Sum => Some(args.iter().sum()),
            StreamItem::Product => Some(args.iter().product()),
            StreamItem::Minimum => Some(*args.iter().min().unwrap()),
            StreamItem::Maximum => Some(*args.iter().max().unwrap()),
            StreamItem::Greater => Some((args[0] > args[1]) as u64),
            StreamItem::Less => Some((args[0] < args[1]) as u64),
            StreamItem::Equal => Some((args[0] == args[1]) as u64),
            _ => None,
        }
    }

    fn process(&mut self) -> Option<u64> {
        let packet = self.next()?;
        match packet.item {
            StreamItem::Literal(value) => Some(value),
            _ => self.operator(packet),
        }
    }
}

impl Iterator for StreamIter<'_> {
    type Item = Packet;

    fn next(&mut self) -> Option<Self::Item> {
        let start = self.position;
        let version = self.read(3)? as u8;
        let type_id = self.read(3)? as u8;

        let mut more_bits: usize = 0;
        let mut more_packets: u16 = 0;
        if type_id != 4 {
            if self.next_bit()? == 0 {
                more_bits = self.read(15)? as usize;
            } else {
                more_packets = self.read(11)? as u16;
            }
        }

        let item = match type_id {
            0 => StreamItem::Sum,
            1 => StreamItem::Product,
            2 => StreamItem::Minimum,
            3 => StreamItem::Maximum,
            4 => StreamItem::Literal(self.read_literal()?),
            5 => StreamItem::Greater,
            6 => StreamItem::Less,
            7 => StreamItem::Equal,
            _ => panic!("unknown command"),
        };
        Some(Packet {
            version,
            bits: self.position - start + more_bits,
            packets: more_packets,
            item,
        })
    }
}

fn main() {
    let input: String = fs::read_to_string("input.txt").expect("Error reading input")
        .trim_end().to_string();
    let score: u32 = StreamIter::from(&input).map(|x| x.version as u32).sum();
    let value: u64 = StreamIter::from(&input).process().unwrap();
    println!("{} {}", score, value)
}
