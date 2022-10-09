const FR_DIACRITIC: [(char, char); 4] = [('È', 'E'), ('É', 'E'), ('Ê', 'E'), ('Ï', 'I')];

pub fn strip_diacritics_fr4(input: &mut [char]) {
    input.iter_mut().for_each(|c| {
        if let Some((_, v)) = FR_DIACRITIC.iter().find(|(k, _)| *k == *c) {
            *c = *v;
        }
    })
}
