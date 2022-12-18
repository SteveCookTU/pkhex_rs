use rand::rngs::ThreadRng;
use rand::Rng;

pub fn shuffle<T: PartialOrd>(items: &mut [T], start: usize, end: usize, rnd: &mut ThreadRng) {
    for i in start..end {
        let index = i + rnd.gen_range(0..(end - i));
        items.swap(index, i);
    }
}
