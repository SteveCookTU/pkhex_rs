use crate::substructures::inventory::item::InventoryItem;
use crate::substructures::inventory::InventoryType;

pub trait InventoryPouch<Item: InventoryItem>: Copy + Clone {
    fn inventory_type(&self) -> InventoryType;

    fn legal_items(&self) -> Vec<u16>;

    fn is_item_legal(&self, item: u16) -> bool;

    fn max_count(&self) -> u32;

    fn count(&self) -> usize {
        self.get_items()
            .iter()
            .filter(|i| i.get_count() > 0)
            .count()
    }

    fn is_cramped(&self) -> bool {
        self.legal_items().len() > self.get_items().len()
    }

    fn get_items(&self) -> Vec<Item>;

    fn set_items(&mut self, items: &[Item]);

    fn offset(&self) -> usize;

    fn pouch_data_size(&self) -> usize;

    fn get_pouch(&self, data: &[u8]);

    fn set_pouch(&mut self, data: &mut [u8]);

    fn sort_by_count(&mut self) {
        let mut items = self.get_items();
        items.sort_by(|x, y| x.get_count().cmp(&y.get_count()));
        self.set_items(&items);
    }

    fn sort_by_index(&mut self) {
        let mut items = self.get_items();
        items.sort_by(|x, y| x.get_index().cmp(&y.get_index()));
        self.set_items(&items);
    }

    fn sort_by_names(&mut self, names: &[&str]) {
        let mut items = self
            .get_items()
            .into_iter()
            .zip(names)
            .collect::<Vec<(Item, &&str)>>();
        items.sort_by(|(_, n), (_, n2)| n.cmp(n2));
        self.set_items(&items.into_iter().map(|i| i.0).collect::<Vec<Item>>());
    }

    fn sort_by_empty(&mut self) {
        let mut items = self.get_items();
        items.sort_by(|x, y| (x.get_count() == 0).cmp(&(y.get_count() == 0)));
        self.set_items(&items);
    }

    fn sanitize(&mut self, max_item_id: u16, hax: bool) {
        let mut ctr = 0;
        let mut items = self.get_items();
        for item in 0..items.len() {
            if items[item].is_valid(&self.legal_items(), max_item_id, hax) {
                items[ctr] = items[item];
                ctr += 1;
            }
        }
        while ctr < items.len() {
            items[ctr].clear();
            ctr += 1;
        }
        self.set_items(&items);
    }

    fn clear_count_0_(&mut self) {
        let mut ctr = 0;
        let mut items = self.get_items();
        for item in 0..items.len() {
            if items[item].get_count() != 0 {
                items[ctr] = items[item];
                ctr += 1;
            }
        }
        while ctr < items.len() {
            items[ctr].clear();
            ctr += 1;
        }
        self.set_items(&items);
    }

    fn remove_all(&mut self) {
        let mut items = self.get_items();
        items.iter_mut().for_each(|i| i.clear());
        self.set_items(&items);
    }

    fn remove_all_with_filter<T>(&mut self, filter: T)
    where
        T: Fn(&Item) -> bool,
    {
        let mut items = self.get_items();
        items
            .iter_mut()
            .filter(|i| filter(i))
            .for_each(|i| i.clear());
        self.set_items(&items);
    }
}
