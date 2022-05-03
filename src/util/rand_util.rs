use rand::rngs::ThreadRng;
use rand::Rng;

pub fn shuffle<T: Copy>(items: &mut [T], start: usize, end: usize, rand: &mut ThreadRng) {
    for i in start..end {
        let index = i + rand.gen_range(0..(end - 1));
        let first = items[i];
        let second = items[index];
        items[index] = first;
        items[i] = second;
    }
}
