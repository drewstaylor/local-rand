use std::hash::Hash;
use std::collections::HashSet;

// 128 bit Primes
const KX: u128 = 190922520971517748059306913241336366489;
const KY: u128 = 234572717015175419981392307326328716211;
// Pascal / Delphi
const KZ: u128 = 134775813;
const KW: u128 = 4294967296;

pub struct Rand {
    x: u128, y: u128, z: u128, w: u128
}

impl Rand{
    pub fn new(seed: u128) -> Rand {
        Rand{
            x: KX^seed, y: KY^seed,
            z: KZ, w: KW
        }
    }

    // Xorshift 128, taken from German Wikipedia
    pub fn rand(&mut self) -> u128 {
        let t = self.x ^ self.x.wrapping_shl(11);
        self.x = self.y;
        self.y = self.z; 
        self.z = self.w;
        self.w ^= self.w.wrapping_shr(19) ^ t ^ t.wrapping_shr(8);
        self.w
    }

    pub fn rand_range(&mut self, a: i128, b: i128) -> i128 {
        let m = (b-a+1) as u128;
        a + (self.rand() % m) as i128
    }
}

pub fn has_unique_elements<T>(iter: T) -> bool
where
    T: IntoIterator,
    T::Item: Eq + Hash,
{
    let mut uniq = HashSet::new();
    iter.into_iter().all(move |x| uniq.insert(x))
}

fn main() {
    let mut vals: Vec<String> = vec![
        "1".to_string(), "2".to_string(), "3".to_string(), "4".to_string(), "5".to_string(),
        "6".to_string(), "7".to_string(), "8".to_string(), "9".to_string(), "10".to_string(),
        "11".to_string(), "12".to_string(), "13".to_string(), "14".to_string(), "15".to_string(),
    ];
    
    // Recent timestamp dates as seconds (like: env.block.at_time.seconds())
    let seeds: Vec<u128> = vec![
        1700402033670, 1700402035093, 1700402040276, 1700402307714, 1700402316411,
        1700402349110, 1700402354251, 1700402359908, 1700402380789, 1700402386916,
        1700402393139, 1700402407675, 1700402412867, 1700402412867, 1700402449201,
    ];
    
    let mut res = vec![];
    
    assert_eq!(vals.len(), seeds.len());
    
    for seed in seeds.iter() {
        let mut rng = Rand::new(*seed);
        let rand_index: i128 = rng.rand_range(0, (vals.len()-1).try_into().unwrap());
        let rand_item = vals[rand_index as usize].clone();
        vals.retain(|item| *item.to_string() != rand_item);
        res.push(rand_item);
    }
    
    dbg!(res.clone());
    
    assert_eq!(vals.len(), 0);
    assert_eq!(res.len(), seeds.len());
    assert!(has_unique_elements(res));
}