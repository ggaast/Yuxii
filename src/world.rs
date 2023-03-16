use std::{fmt::{Display, Debug}};

use hashbrown::{HashMap, HashSet};

use static_init::dynamic;
use idalloc::Slab;


use crate::{Entity, Scene, static_scene::{StaticScene, StaticScenes}, Node, ComponentInterface};


#[dynamic]
pub static mut WORLD: World = World::default();


// todo: implement Slab for Entity.
pub struct World {
    pub id_alloc: Slab<u16>,
    pub ent_hierarchie: HashMap<Entity, EntityHierarchie>,
    pub node_roots: HashMap<Entity, Box<dyn Node>>,
}

impl Default for World {
    fn default() -> Self {
        World {
            id_alloc: idalloc::Slab::<u16>::new(),
            ent_hierarchie: HashMap::new(),
            node_roots: HashMap::new()
        }
    }
}
#[allow(dead_code)]
impl World {

    pub fn read() -> static_init::lazy::lesser_locked_lazy::ReadGuard<'static, World>{
        WORLD.read()
    }

    pub(crate) fn write() -> static_init::lazy::lesser_locked_lazy::WriteGuard<'static, World>{
        WORLD.write()
    }

    pub fn spawn() -> Entity {
        let mut world = WORLD.write();
        let id = world.id_alloc.next();
        let ent = Entity::new_with_id(id);
        world.ent_hierarchie.insert(ent, EntityHierarchie::empty());
        ent
    }
    pub fn despawn(_entity: Entity){
        todo!()
    }
    pub fn entities() -> DisplayWrapper{
        let world = WORLD.read();
        DisplayWrapper{
            map: world.ent_hierarchie.clone()
        }
        
    }
    pub fn to_static_scenes() -> StaticScenes{

        let mut scenes: StaticScenes = StaticScenes::default();
        

        for (entity, hierarchie) in WORLD.read().ent_hierarchie.iter(){
            if hierarchie.parent.is_none(){
                
                let scene = Self::construct_static_scene(*entity);


                scenes.0.push(scene);
            }
        }    
        scenes
    }

    pub fn to_scene() -> Scene{
        let mut scenes_map: HashMap<Entity, Scene> = HashMap::new();

        let _world = WORLD.read();

        for (entity, hierarchie) in WORLD.read().ent_hierarchie.iter(){
            if hierarchie.parent.is_none(){

                scenes_map.insert(*entity, entity.to_scene());
                //let scene = Self::construct_static_scene(*entity);
                // let mut scene = Scene::new();

                // scene.entity = Some(*entity);
                // if let Some(node_boxed) = world.node_roots.get(entity){
                //     //let node = node_boxed.as_mut();
                //     //let boxed = Box::new(node);
                //     //scene.node_root = Some(boxed);
                // }
            

                //scenes.0.push(scene);
            }
        }   


        todo!()
    }


    fn construct_static_scene(entity: Entity) -> StaticScene{

        

        // for component_boxed in TABLE_ACCESSERS.read().iter(){
            
        //     if let Some((component, is_name)) = component_boxed.as_ref().get_fmt(entity){
        //         if is_name{
        //             scene.name = Some(component);
        //         } else{
        //             scene.components.push(component);
        //         }
                
        //     }
        // }

        // for child_entity in entity.children(){
        //     scene.children.push(Self::construct_static_scene(child_entity))
        // }

        StaticScene::new(entity)
    }

    pub(crate) fn add_child(parent: Entity, child: Entity) -> Entity{
        let mut world = WORLD.write();

        if let Some([parent_desc, child_desc]) = world.ent_hierarchie.get_many_mut([&parent, &child]){
            if child_desc.parent.is_none(){
                parent_desc.children.insert(child);
                child_desc.parent = Some(parent);
            }
            else {
                println!("Warning! Entity can not be added as a child, because it is already a child");
            }
        }
        parent
    }

    pub fn get_entity_parent(&self, entity: &Entity) -> Option<Entity>{
        if let Some(entity) = self.ent_hierarchie.get(entity){
            return entity.parent;
        }
        None
    }

    pub fn remove_child(_child: Entity){
        todo!()
    }
}

#[derive(Debug, Clone)]
pub struct EntityHierarchie{
    pub children: HashSet<Entity>,
    pub parent: Option<Entity>,
    //pub node_root: Option<Box<dyn Node>>
}
impl EntityHierarchie{
    pub fn empty() -> EntityHierarchie{
        EntityHierarchie{
            children: HashSet::new(),
            parent: None,
            //node_root: todo!(),
        }
    }
}


// use crate::*;

// #[derive(Component, Default, Clone, Debug, serde::Serialize)]
// struct  Vel;

// #[dynamic]
// pub static mut TABLE_ACCESSERS: Vec<dyn crate::ComponentInterface<Vely> > = Vec::new();



#[dynamic]
pub static mut TABLE_ACCESSERS: Vec<Box<dyn ComponentInterface>> = Vec::new();

#[derive(Debug)]
pub struct DisplayWrapper{
    pub map: HashMap<Entity, EntityHierarchie>
}

impl Display for DisplayWrapper{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        
        let mut out = "".to_owned(); 
        for (root, entity_h) in self.map.iter(){
            if entity_h.parent.is_none(){
                
                out += &format!("{}{}\n", root, display_children(entity_h, &self.map, "".to_owned()));
            }
        }
        write!(f, "\n{}", out)
    }
}

fn display_children(entity_h_out: &EntityHierarchie, map: &HashMap<Entity, EntityHierarchie>, mut out: String) -> String{

    if entity_h_out.children.is_empty(){
        return "".to_owned();
    }

    for child in entity_h_out.children.iter(){
            out += &format!(" {}{} ", child, display_children(map.get(child).unwrap(), map, out.clone()));
    }
    format!("[{}]",out)
}

