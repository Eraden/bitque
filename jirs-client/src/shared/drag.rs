use std::collections::HashSet;

#[derive(Default, Debug)]
pub struct DragState {
    pub last_id: Option<i32>,
    pub dragged_id: Option<i32>,
    pub dirty: HashSet<i32>,
}

impl DragState {
    #[inline]
    pub fn mark_dirty(&mut self, id: i32) {
        self.dirty.insert(id);
    }

    #[inline]
    pub fn drag(&mut self, id: i32) {
        self.dragged_id = Some(id);
        self.mark_dirty(id);
    }

    #[inline]
    pub fn dragged_or_last(&self, id: i32) -> bool {
        self.dragged_id == Some(id) || self.last_id == Some(id)
    }

    #[inline]
    pub fn clear_last(&mut self) {
        self.last_id = None;
    }

    #[inline]
    pub fn clear(&mut self) {
        self.last_id = None;
        self.dragged_id = None;
        self.dirty.clear();
    }
}
