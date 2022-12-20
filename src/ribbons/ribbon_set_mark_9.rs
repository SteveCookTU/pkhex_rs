use crate::ribbons::RibbonHasMark;

pub trait RibbonSetMark9: RibbonHasMark {
    fn ribbon_mark_jumbo(&self) -> bool;
    fn set_ribbon_mark_jumbo(&mut self, val: bool);
    fn ribbon_mark_mini(&self) -> bool;
    fn set_ribbon_mark_mini(&mut self, val: bool);
    fn ribbon_mark_item_finder(&self) -> bool;
    fn set_ribbon_mark_item_finder(&mut self, val: bool);
    fn ribbon_mark_partner(&self) -> bool;
    fn set_ribbon_mark_partner(&mut self, val: bool);
    fn ribbon_mark_gourmand(&self) -> bool;
    fn set_ribbon_mark_gourmand(&mut self, val: bool);
    fn ribbon_mark_alpha(&self) -> bool;
    fn set_ribbon_mark_alpha(&mut self, val: bool);
    fn ribbon_mark_mightiest(&self) -> bool;
    fn set_ribbon_mark_mightiest(&mut self, val: bool);
    fn ribbon_mark_titan(&self) -> bool;
    fn set_ribbon_mark_titan(&mut self, val: bool);

    fn ribbon_bits(&self) -> Vec<bool> {
        vec![
            self.ribbon_mark_jumbo(),
            self.ribbon_mark_mini(),
            self.ribbon_mark_item_finder(),
            self.ribbon_mark_partner(),
            self.ribbon_mark_gourmand(),
            self.ribbon_mark_alpha(),
            self.ribbon_mark_mightiest(),
            self.ribbon_mark_titan(),
        ]
    }
}
