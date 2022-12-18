use crate::pkm::shared::EntityContext;
use crate::pkm::util::pkx;

const EXTENSION_PB7: &str = "pb7";
const EXTENSION_PB8: &str = "pb8";
const EXTENSION_PA8: &str = "pa8";

pub const EXTENSIONS_7B: [&str; 1] = [EXTENSION_PB7];

pub fn get_extensions(max_generation: Option<u8>) -> Vec<String> {
    let max_generation = max_generation.unwrap_or(pkx::GENERATION);
    let min = if (3..7).contains(&max_generation) {
        3
    } else {
        1
    };
    let size = max_generation - min + 1 + 6;
    let mut result = Vec::with_capacity(size as usize);
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

pub fn get_context_from_extension(ext: &str, prefer: Option<EntityContext>) -> EntityContext {
    let prefer = prefer.unwrap_or(EntityContext::None);

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
            ext.chars().last().unwrap_or_default(),
            prefer,
        ))
    }
}

fn get_format_from_extension(last: char, prefer: EntityContext) -> u8 {
    if ('1'..='9').contains(&last) {
        return last as u8 - b'0';
    }
    if prefer.generation().unwrap_or_default() <= 7 && last == 'x' {
        return 6;
    }
    prefer as u8
}
