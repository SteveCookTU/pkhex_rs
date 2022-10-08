use crate::EntityContext;

const EXTENSION_PB7: &str = "pb7";
const EXTENSION_PB8: &str = "pb8";
const EXTENSION_PA8: &str = "pa8";

pub fn get_extensions(max_generation: u8) -> Vec<String> {
    let min = if (3..7).contains(&max_generation) {
        3
    } else {
        1
    };
    let size = max_generation - min + 1 + 6;
    let mut result = Vec::with_capacity(size.into());

    for i in min..=max_generation {
        result.push(format!("pk{i}"));
    }

    if max_generation >= 3 {
        result.push("ck3".to_string());
        result.push("xk3".to_string());
    }

    if max_generation >= 4 {
        result.push("bk4".to_string());
        result.push("rk4".to_string());
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

pub fn get_context_from_extension(ext: &str, prefer: EntityContext) -> EntityContext {
    if ext.is_empty() {
        prefer
    } else {
        let is = |ext: &str, str: &str| -> bool { ext.ends_with(str) };

        if is(ext, "b8") {
            EntityContext::Gen8b
        } else if is(ext, "k8") {
            EntityContext::Gen8
        } else if is(ext, "b7") {
            EntityContext::Gen7b
        } else if is(ext, "k7") {
            EntityContext::Gen7
        } else if is(ext, "k6") {
            EntityContext::Gen6
        } else {
            EntityContext::from(get_format_from_extension(
                *ext.as_bytes().last().unwrap(),
                prefer,
            ))
        }
    }
}

fn get_format_from_extension(last: u8, prefer: EntityContext) -> u8 {
    if (b'1'..=b'9').contains(&last) {
        last - b'0'
    } else if prefer.generation() <= 7 && last == b'x' {
        6
    } else {
        prefer as u8
    }
}
