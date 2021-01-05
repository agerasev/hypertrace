use crate::traits::*;

#[derive(Clone)]
pub struct TypedVec<V: Value> {
    item_type: V::Type,
    items: Vec<V>,
}

impl<V: Value> TypedVec<V> {
    pub fn new(item_type: V::Type) -> Self {
        Self {
            item_type,
            items: Vec::new(),
        }
    }

    pub fn from_items(item_type: V::Type, items: Vec<V>) -> Option<Self> {
        if items.iter().all(|item| item.type_().id() == item_type.id()) {
            Some(Self { item_type, items })
        } else {
            None
        }
    }

    pub fn item_type(&self) -> &V::Type {
        &self.item_type
    }

    pub fn push(&mut self, item: V) -> Result<(), V> {
        if self.item_type.id() == item.type_().id() {
            self.items.push(item);
            Ok(())
        } else {
            Err(item)
        }
    }

    pub fn pop(&mut self) -> Option<V> {
        self.items.pop()
    }

    pub fn get(&self, pos: usize) -> Option<&V> {
        self.items.get(pos)
    }

    #[allow(clippy::result_unit_err)]
    pub fn update<F: FnOnce(&mut V)>(&mut self, pos: usize, f: F) -> Result<(), ()> {
        let item = self.items.get_mut(pos).ok_or(())?;
        f(item);
        assert_eq!(item.type_().id(), self.item_type.id());
        Ok(())
    }

    pub fn iter(&self) -> impl Iterator<Item = &V> {
        self.items.iter()
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn into_items(self) -> Vec<V> {
        self.items
    }
}
