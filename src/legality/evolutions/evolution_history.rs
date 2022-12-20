use crate::legality::evolutions::EvoCriteria;
use crate::pkm::shared::EntityContext;
use crate::{PKError, PKResult};

#[derive(Default, Clone)]
pub struct EvolutionHistory {
    pub gen_1: Vec<EvoCriteria>,
    pub gen_2: Vec<EvoCriteria>,
    pub gen_3: Vec<EvoCriteria>,
    pub gen_4: Vec<EvoCriteria>,
    pub gen_5: Vec<EvoCriteria>,
    pub gen_6: Vec<EvoCriteria>,
    pub gen_7: Vec<EvoCriteria>,
    pub gen_8: Vec<EvoCriteria>,
    pub gen_9: Vec<EvoCriteria>,

    pub gen_7b: Vec<EvoCriteria>,
    pub gen_8a: Vec<EvoCriteria>,
    pub gen_8b: Vec<EvoCriteria>,
}

impl EvolutionHistory {
    pub fn has_visited_gen_1(&self) -> bool {
        !self.gen_1.is_empty()
    }

    pub fn has_visited_gen_2(&self) -> bool {
        !self.gen_2.is_empty()
    }

    pub fn has_visited_gen_3(&self) -> bool {
        !self.gen_3.is_empty()
    }

    pub fn has_visited_gen_4(&self) -> bool {
        !self.gen_4.is_empty()
    }

    pub fn has_visited_gen_5(&self) -> bool {
        !self.gen_5.is_empty()
    }

    pub fn has_visited_gen_6(&self) -> bool {
        !self.gen_6.is_empty()
    }

    pub fn has_visited_gen_7(&self) -> bool {
        !self.gen_7.is_empty()
    }

    pub fn has_visited_swsh(&self) -> bool {
        !self.gen_8.is_empty()
    }

    pub fn has_visited_gen_9(&self) -> bool {
        !self.gen_9.is_empty()
    }

    pub fn has_visited_lgpe(&self) -> bool {
        !self.gen_7b.is_empty()
    }

    pub fn has_visited_pla(&self) -> bool {
        !self.gen_8a.is_empty()
    }

    pub fn has_visited_bdsp(&self) -> bool {
        !self.gen_8b.is_empty()
    }

    pub fn get(&self, context: EntityContext) -> PKResult<&[EvoCriteria]> {
        match context {
            EntityContext::Gen1 => Ok(&self.gen_1),
            EntityContext::Gen2 => Ok(&self.gen_2),
            EntityContext::Gen3 => Ok(&self.gen_3),
            EntityContext::Gen4 => Ok(&self.gen_4),
            EntityContext::Gen5 => Ok(&self.gen_5),
            EntityContext::Gen6 => Ok(&self.gen_6),
            EntityContext::Gen7 => Ok(&self.gen_7),
            EntityContext::Gen8 => Ok(&self.gen_8),
            EntityContext::Gen9 => Ok(&self.gen_9),
            EntityContext::Gen7b => Ok(&self.gen_7b),
            EntityContext::Gen8a => Ok(&self.gen_8a),
            EntityContext::Gen8b => Ok(&self.gen_8b),
            _ => Err(PKError::IndexOutOfRange {
                index: context as usize,
            }),
        }
    }

    pub fn get_mut(&mut self, context: EntityContext) -> PKResult<&mut Vec<EvoCriteria>> {
        match context {
            EntityContext::Gen1 => Ok(&mut self.gen_1),
            EntityContext::Gen2 => Ok(&mut self.gen_2),
            EntityContext::Gen3 => Ok(&mut self.gen_3),
            EntityContext::Gen4 => Ok(&mut self.gen_4),
            EntityContext::Gen5 => Ok(&mut self.gen_5),
            EntityContext::Gen6 => Ok(&mut self.gen_6),
            EntityContext::Gen7 => Ok(&mut self.gen_7),
            EntityContext::Gen8 => Ok(&mut self.gen_8),
            EntityContext::Gen9 => Ok(&mut self.gen_9),
            EntityContext::Gen7b => Ok(&mut self.gen_7b),
            EntityContext::Gen8a => Ok(&mut self.gen_8a),
            EntityContext::Gen8b => Ok(&mut self.gen_8b),
            _ => Err(PKError::IndexOutOfRange {
                index: context as usize,
            }),
        }
    }

    pub fn has_visited(&self, context: EntityContext, species: u16) -> PKResult<bool> {
        let evos = self.get(context)?;
        for evo in evos {
            if evo.species == species {
                return Ok(true);
            }
        }
        Ok(false)
    }

    pub fn set(&mut self, context: EntityContext, chain: &[EvoCriteria]) -> PKResult<()> {
        let arr = self.get_mut(context)?;
        *arr = chain.to_vec();
        Ok(())
    }
}
