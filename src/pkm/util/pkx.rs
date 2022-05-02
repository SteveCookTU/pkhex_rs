pub const GENERATION: usize = 8;

const EXTENSION_PB7: &str = "pb7";
const EXTENSION_PB8: &str = "pb8";
const EXTENSION_PA8: &str = "pa8";

pub fn get_pkm_extensions(max_generation: usize) -> Vec<String> {
    let mut result = vec![];
    let min = if max_generation <= 2 || max_generation >= 7 {
        1
    } else {
        0
    };
    for i in min..=max_generation {
        result.push(format!("pk{}", i));
    }

    if max_generation >= 3 {
        result.push("ck3".to_string());
        result.push("xk3".to_string());
    }
    if max_generation >= 4 {
        result.push("bk4".to_string());
    }
    if max_generation >= 7 {
        result.push(EXTENSION_PB7.to_string());
    }
    if max_generation >= 8 {
        result.push(EXTENSION_PB8.to_string());
        result.push(EXTENSION_PA8.to_string());
    }
    result
}
