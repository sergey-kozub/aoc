use crate::intcode::IntCode;

type Payload = (i64, i64);
type Packet = (usize, Payload);

struct Network {
    nodes: Vec<IntCode>,
    nat: Option<Payload>,
}

impl Network {
    fn new(size: usize, program: &str) -> Network {
        let mut nodes: Vec<IntCode> = Vec::new();
        for addr in 0..size {
            let mut cpu = IntCode::from(program);
            cpu.input.push_back(addr as i64);
            assert!(cpu.run().is_none());
            nodes.push(cpu);
        }
        Network { nodes, nat: None }
    }

    fn read(&mut self) -> Option<Packet> {
        for node in &mut self.nodes {
            if node.input.is_empty() {
                node.input.push_back(-1);
            }
            if let Some(addr) = node.wait() {
                let a = node.wait_many(2).unwrap();
                let packet = (addr as usize, (a[0], a[1]));
                return Some(packet);
            }
        }
        None
    }

    fn next(&mut self) -> (Packet, bool) {
        let (packet, is_nat) = match self.read() {
            Some(packet) => {
                if packet.0 == 255 {
                    self.nat = Some(packet.1);
                    return (packet, true);
                }
                (packet, false)
            },
            None => {
                let packet = (0, self.nat.unwrap());
                (packet, true)
            },
        };
        let (addr, data) = packet;
        let input = &mut self.nodes[addr].input;
        input.push_back(data.0);
        input.push_back(data.1);
        (packet, is_nat)
    }

    fn next_nat(&mut self) -> Option<Packet> {
        loop {
            let (packet, is_nat) = self.next();
            if is_nat { return Some(packet); }
        }
    }
}

pub fn run(content: &str) {
    let mut network = Network::new(50, content);
    let first = network.next_nat().unwrap().1;

    let mut last: i64 = 0;
    while let Some((addr, data)) = network.next_nat() {
        if addr == 0 {
            if data.1 == last { break; }
            last = data.1;
        }
    }
    println!("{} {}", first.1, last);
}
