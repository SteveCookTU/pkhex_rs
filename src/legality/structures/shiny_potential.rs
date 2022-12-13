use crate::legality::Shiny;

pub trait ShinyPotential {
    fn shiny(&self) -> Shiny;
}
