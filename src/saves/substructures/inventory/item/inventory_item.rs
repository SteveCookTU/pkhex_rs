pub trait InventoryItem: Copy + Clone {
    fn get_count(&self) -> u16;
    fn set_count(&mut self, count: u16);
    fn get_index(&self) -> u16;
    fn set_index(&mut self, index: u16);

    fn is_valid(&self, legal: &[u16], max_item_id: u16, hax: bool) -> bool {
        if self.get_index() == 0 {
            true
        } else if self.get_index() > max_item_id {
            false
        } else {
            hax || legal.contains(&self.get_index())
        }
    }

    fn clear(&mut self) {
        self.set_count(0);
        self.set_count(0);
    }

    fn set_new_details(&mut self, count: u16) {
        self.set_count(count);
    }

    fn merge_overwrite<T: InventoryItem>(&mut self, other: &T) {
        self.set_count(self.get_count().max(other.get_count()));
    }
}
