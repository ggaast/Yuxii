

use std::{fmt::Debug};


use hashbrown::{HashMap, hash_map::{IterMut, Iter}, HashSet};
use static_init::lazy::lesser_locked_lazy::{ReadGuard, WriteGuard};
use crate::{Entity};

#[typetag::serde(tag = "type")]
pub trait ComponentInterface: Debug + 'static + Send{
    fn add_entity(&self, entity: Entity);
    fn read() -> ReadGuard<'static, Table<Self>> where Self: Sized;
    fn write() -> WriteGuard<'static, Table<Self>> where Self: Sized;
    fn value_read(&self) -> ReadGuard<'static, Table<Self>> where Self: Sized{
        Self::read()
    }
    fn value_write(&self) -> WriteGuard<'static, Table<Self>> where Self: Sized{
        Self::write() 
    }
}

pub trait ResourceInterface: Sized {
    fn read() -> ReadGuard<'static, Self>;
    fn write() -> WriteGuard<'static, Self>;
    fn value_read(&self) -> ReadGuard<'static, Self>{
        Self::read()
    }
    fn value_write(&self) -> WriteGuard<'static, Self>{
        Self::write()
        
    }
}

pub trait TableInterface: 'static{
    fn contains(&self, entity: &Entity) -> bool;
}

impl<T: 'static> TableInterface for Table<T>{
    fn contains(&self, entity: &Entity) -> bool {
        self.contains(entity)
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Table<T> {
    pub(crate) storage: HashMap<Entity, T>,
    pub(crate) added: HashSet<Entity>,
    pub(crate) removed: HashSet<Entity>,
}

#[allow(dead_code)]
impl<T: Debug> Table<T> {
    pub fn new() -> Table<T> {
        Table {
            storage: HashMap::new(),
            added: HashSet::new(),
            removed: HashSet::new(),
        }
    }
    pub fn remove(&mut self, entity: &Entity){
        self.storage.remove(entity);
    }
    pub fn add(&mut self, entity: Entity, component: T) -> Entity{
        self.storage.insert(entity, component);
        entity
    }
    pub fn get_mut(&mut self, entity: &Entity) -> Option<&mut T> {
        self.storage.get_mut(entity)
    }
    pub fn get(&self, entity: &Entity) -> Option<&T> {
        self.storage.get(entity)
    }
    pub fn iter_mut(&mut self) -> IterMut<Entity, T> {
        self.storage.iter_mut()
    }
    pub fn iter(&self) -> Iter<Entity, T> {
        self.storage.iter()
    }
    pub fn contains(&self, entity: &Entity) -> bool {
        self.storage.contains_key(entity)
    }
    pub fn is_added(&self, entity: &Entity) -> bool {
        self.added.contains(entity)
    }
    pub fn added_entities(&self) -> &HashSet<Entity>{
        &self.added
    }
    pub fn is_removed(&self, entity: &Entity) -> bool {
        self.removed.contains(entity)
    }
    pub fn removed_entities(&self) -> &HashSet<Entity>{
        &self.removed
    }
    pub fn entities_amount(&self) -> usize{
        self.storage.len()
    }
    pub fn entities(&self) -> HashSet<Entity>{
        self.storage.keys().cloned().collect()
    }
}


