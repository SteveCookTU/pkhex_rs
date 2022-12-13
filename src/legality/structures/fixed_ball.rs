use crate::game::enums::Ball;

pub trait FixedBall {
    fn fixed_ball(&self) -> Ball;
}
