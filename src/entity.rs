use std::{ops::{Shl}, fmt::{Display, Debug}, hash::Hash};

use hashbrown::HashSet;


use crate::{ComponentInterface, World, name::Name, world::WORLD};

#[derive(Debug, Hash, PartialEq, Eq, Copy, Clone, serde::Serialize, Deserialize)]
pub struct Entity {
    pub id: u16,
}

impl Shl for Entity {
    type Output = Self;

    fn shl(self, rhs: Entity) -> Self::Output {
        self.add_child(rhs)
    }
}

#[allow(dead_code)]
impl Entity {
    pub fn new_with_id(id: u16) -> Entity {
        Entity { 
            id, 
        }
    }
    // fn get_hierarchie(&self) -> EntityHierarchie{

    // }
    pub fn add<T: ComponentInterface + 'static>(self, component: T) -> Entity {
        T::write().storage.insert(self, component);
        self
    }
    pub fn set<T: ComponentInterface + 'static>(self, component: T) -> Entity {
        T::write().storage.insert(self, component);
        self
    }
    pub fn remove<T: ComponentInterface + 'static>(self) -> Entity {
        T::write().storage.remove(&self);
        self
    }
    pub fn with<T: ComponentInterface + 'static + Clone>(self) -> bool{
        T::read().storage.contains_key(&self)
        
    }
    pub fn add_child(self, child: Entity) -> Entity{
        World::add_child(self, child)
    }
    pub fn children(self) -> HashSet<Entity>{
        let lck = WORLD.read();

        for entity in lck.ent_hierarchie.get(&self){
            return entity.children.clone();
        };

        hashbrown::HashSet::new()
    }
    pub fn to_scene(self) -> crate::Scene{
        todo!()
    }

    pub fn get_parent(&self) -> Option<Entity>{
        World::read().get_entity_parent(self)
    }

}

impl Display for Entity{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Entity [{}]", self.id)
    }
}



use serde::{Deserialize};


impl Shl<&str> for Entity {
    type Output = Self;

    fn shl(self, rhs: &str) -> Self::Output {
        self << Name(rhs.to_string())
    }
}