
const DECRYPTION_KEY: isize = 811_589_153;

#[derive(Debug)]
struct Mixer {
    index: Vec<isize>,
    value: Vec<isize>,
    size: isize,
}

impl Mixer {
    fn from(input: Vec<isize>, k: isize) -> Mixer {
        let size = input.len() as isize;
        Mixer {
            index: (0..size).collect(),
            value: input.into_iter().map(|x| x * k).collect(),
            size,
        }
    }

    fn get(&self) -> Vec<isize> {
        let mut pairs: Vec<(isize, isize)> = self.index.iter().cloned()
            .zip(self.value.iter().cloned()).collect();
        pairs.sort();
        pairs.iter().map(|x| x.1).collect()
    }

    fn shl(&mut self, i: isize, n: isize) -> isize {
        let inc = i >= n;
        let r = if inc {(i - n)..i} else {i..(self.size - n + i)};
        for p in 0..(self.size as usize) {
            if r.contains(&self.index[p]) == inc {
                let next = self.index[p] + 1;
                self.index[p] = if next < self.size {next} else {0};
            }
        }
        if inc {r.start} else {r.end}
    }

    fn shr(&mut self, i: isize, n: isize) -> isize {
        let inc = i + n < self.size;
        let r = if inc {(i + 1)..=(i + n)} else {((i + n + 1) % self.size)..=i};
        for p in 0..(self.size as usize) {
            if r.contains(&self.index[p]) == inc {
                let prev = self.index[p] - 1;
                self.index[p] = if prev >= 0 {prev} else {self.size - 1};
            }
        }
        if inc {*r.end()} else {*r.start() - 1}
    }

    fn shuffle(&mut self) {
        for p in 0..(self.size as usize) {
            let i = self.index[p];
            let n = self.value[p] % (self.size - 1);
            if n > 0 {
                self.index[p] = self.shr(i, n);
            } else if n < 0 {
                self.index[p] = self.shl(i, -n);
            }
        }
    }
}

fn score(data: Vec<isize>) -> isize {
    let p = data.iter().position(|&x| x == 0).unwrap();
    [1000, 2000, 3000].map(|i| data[(p + i) % data.len()]).iter().sum::<isize>()
}

pub fn run(content: &str) {
    let values: Vec<isize> = content.lines().map(|s| s.parse::<isize>().unwrap()).collect();
    let mut mix_1 = Mixer::from(values.clone(), 1);
    mix_1.shuffle();
    let mut mix_2 = Mixer::from(values.clone(), DECRYPTION_KEY);
    for _ in 0..10 { mix_2.shuffle(); }
    println!("{} {}", score(mix_1.get()), score(mix_2.get()));
}

#[cfg(test)]
mod tests {
    #[test]
    pub fn shuffle() {
        let check = |n: usize, k: isize, r: isize| {
            let mut mixer = super::Mixer::from(vec![1, 2, -3, 3, -2, 0, 4], k);
            for _ in 0..n { mixer.shuffle(); }
            assert_eq!(super::score(mixer.get()), r);
        };
        check(1, 1, 3);
        check(10, super::DECRYPTION_KEY, 1_623_178_306);
    }
}
