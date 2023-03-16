use std::{fmt::Debug, ops::Not};

use hashbrown::HashSet;


use crate::{ComponentInterface, Entity, TableInterface};


#[macro_export]
macro_rules! query_old {
    ( $st:ty ) => {
        {

        let mut filter = Filter::new(Vec::new());

        filter.0.push(FToken::with::<$st>());
        // for i in $st.split(","){

        // }




        filter

    }};
}

#[derive(Debug)]
pub struct Filter(pub Vec<FToken>);


impl Filter{
    pub fn new(tokens: Vec<FToken>) -> Self{
        Filter(tokens)
    }

    pub fn search(&self) -> Vec<Entity>{
        
        let mut found = Vec::new();

        for entity in self.smallest(){

            let mut every_token_satisfied = true;

            for token in self.0.iter(){
                if token.satisfied(&entity){
                    continue;

                }
                every_token_satisfied = false;
                break;
            }

            if every_token_satisfied{
                found.push(entity);
            }
        }

        found

        
    
    }
    fn smallest(&self) -> HashSet<Entity>{

        let mut min = None;
        let mut target_index = usize::MAX;

        for index in 0..self.0.len(){

            let mut compare = |len: usize|{
                
                if let Some(min_unwraped) = min{
                    if len < min_unwraped{
                        target_index = index;
                        min = Some(len);
                    }
                } else {
                    target_index = index;
                    min = Some(len);
                }

            };

            let token = &self.0[index];
                if !token.get_is_not(){
                    compare(token.get_len());
                }
            
        }
        
        if min.is_some(){
            return self.0[target_index].get_entities().unwrap().clone();
        } else {
            HashSet::new()
        }
        
    }
}

#[derive(Debug)]
pub enum FToken{
    Scope(FScope),

    Has{
        is_not: bool,
        entities: HashSet<Entity>,
    },
    Added{
        is_not: bool,
        entities: HashSet<Entity>,
    },
    Removed{
        is_not: bool,
        entities: HashSet<Entity>,
    },
    Changed{
        is_not: bool,
        entities: HashSet<Entity>,
    },
    Node{
        is_not: bool,
    },
    Name{
        is_not: bool,
    },
    Or(Vec<FToken>, Vec<FToken>),
}

impl FToken{

    pub fn with<T: ComponentInterface>() -> Self{
        let entities = HashSet::from_iter(T::read().entities());

        FToken::Has { is_not: false, entities }
    }

    pub fn has<T: ComponentInterface>(is_not: bool) -> Self{
        let entities = HashSet::from_iter(T::read().entities());

        FToken::Has { is_not, entities }
    }

    pub fn added<T: ComponentInterface>(_is_not: bool) -> Self{
        //let entities = T::read().added_entities();

        //FToken::Added { is_not, entities: entities.clone() }
        todo!()
    }

    pub fn removed<T: ComponentInterface>(_is_not: bool) -> Self{
        //let entities = T::read().removed_entities();

        //FToken::Removed { is_not, entities: entities.clone() }
        todo!()
    }


    fn get_entities(&self) -> Option<&HashSet<Entity>>{
        match self{
            FToken::Scope(_) => None,
            FToken::Has { is_not: _, entities } => Some(entities),
            FToken::Added { is_not: _, entities } => Some(entities),
            FToken::Removed { is_not: _, entities } => Some(entities),
            FToken::Changed { is_not: _, entities } => Some(entities),
            FToken::Node { is_not: _ } => todo!(),
            FToken::Name { is_not: _ } => todo!(),
            FToken::Or(_, _) => todo!(),
        }
    }

    fn get_len(&self) -> usize{
        match self{
            FToken::Scope(_) => usize::MAX,
            FToken::Has { is_not: _, entities } => entities.len(),
            FToken::Added { is_not: _, entities } => entities.len(),
            FToken::Removed { is_not: _, entities } => entities.len(),
            FToken::Changed { is_not: _, entities } => entities.len(),
            FToken::Node { is_not: _ } => todo!(),
            FToken::Name { is_not: _ } => todo!(),
            FToken::Or(_, _) => todo!(),
        }
    }

    fn get_is_not(&self) -> &bool{

        match self{
            FToken::Scope(_) => &false,
            FToken::Has { is_not, entities: _ } => is_not,
            FToken::Added { is_not, entities: _ } => is_not,
            FToken::Removed { is_not, entities: _ } => is_not,
            FToken::Changed { is_not, entities: _ } => is_not,
            FToken::Node { is_not: _ } => todo!(),
            FToken::Name { is_not: _ } => todo!(),
            FToken::Or(_, _) => todo!(),
        }
    }

    fn satisfied(&self, entity: &Entity) -> bool{

        let check = 
        |is_not, entities: &HashSet<Entity>| -> bool{
            if is_not != &entities.contains(entity){
                return true
            }
            false
        };

            return match self {
                FToken::Scope(scope) => scope.satisfied(entity),
                FToken::Has { is_not, entities } => check(is_not, entities),
                FToken::Added { is_not, entities } => check(is_not, entities),
                FToken::Removed { is_not, entities } => check(is_not, entities),
                FToken::Changed { is_not, entities } => check(is_not, entities),
                FToken::Node { is_not: _ } => todo!(),
                FToken::Name { is_not: _ } => todo!(),
                FToken::Or(_, _) => todo!(),
            };
        

        false
    }

    
}

impl Not for FToken{
    type Output = FToken;

    fn not(mut self) -> Self::Output {
        match &mut self{
            FToken::Scope(_) => todo!(),
            FToken::Has { is_not, entities: _ } => { *is_not = !*is_not},
            FToken::Added { is_not, entities: _  } => *is_not = !*is_not,
            FToken::Removed { is_not, entities: _  } => *is_not = !*is_not,
            FToken::Changed { is_not, entities: _  } => *is_not = !*is_not,
            FToken::Node { is_not } => *is_not = !*is_not,
            FToken::Name { is_not } => *is_not = !*is_not,
            FToken::Or(_, _) => todo!(),
        };
        self
    }
}
#[derive(Debug)]
pub enum FScope{
    Children(Vec<FToken>),
    Parent(Vec<FToken>),
    Child(Vec<FToken>),
    Current(Vec<FToken>),
}

impl FScope{
    pub fn satisfied(&self, entity: &Entity) -> bool{
        match self{
            FScope::Children(_) => todo!(),
            FScope::Parent(tokens) => {

                let mut every_token_satisfied = true;

                for token in tokens{
                    if let Some(parent) = entity.get_parent(){
                        if !token.satisfied(&parent){
                            every_token_satisfied = false;
                        }
                    }

                }
                
                every_token_satisfied
            },
            FScope::Child(_) => todo!(),
            FScope::Current(_) => todo!(),
        }
    }
}


