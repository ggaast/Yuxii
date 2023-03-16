use crate::*;
/*

    Scene is a isolated out of World entity descriptor.
    You can create a scene via file, in editor or directly in code.
    Scene is a collected in single struct representation of a single Entity with all Components and Children

    By default each Node::spawn() call, it will parse each scene file. But if you build with --scene-backed flag...

*/

pub struct SEntity;


pub struct SNode{
    node: Box<dyn Node>,
    modifiers: Vec<Modifier>,
    removed_modifiers: Vec<Modifier>,
    is_export: bool,
}


impl SNode{
    pub fn new<T: Node + Default + 'static>() -> SNode{
        SNode { node: Box::<T>::default(), modifiers: vec![], is_export: false, removed_modifiers: vec![] }
    }
    pub fn add_modifier(&mut self, modifier: Modifier){
        self.modifiers.push(modifier);
    }
    fn to_modifier(self) -> Modifier{
        Modifier::Node(self.is_export, self.node, self.modifiers, self.removed_modifiers)
    }
}


#[derive(serde::Serialize, Deserialize, Debug)]
pub struct Scene{

    entity: Option<Entity>,

    name: Option<String>,

    modifiers: Vec<Modifier>, 

}
#[derive(Debug, Serialize, Deserialize)]
pub enum Modifier{
    Scene(/* is exported */ bool, Scene), // Can be shown as default modifier in scene // True - yes // False - no
    Component(/* is exported */ bool, Box<dyn ComponentInterface>),
    Bundle(/* is exported */ bool, Box<dyn BundleMarker>),
    Node(/* is exported */ bool, Box<dyn Node>, /* Added/Changed Modifiers */ Vec<Modifier>, /* Removed Modifiers */Vec<Modifier>)
}

pub struct ExportedModifier(pub Modifier);


use std::fs::{self, File};

use ron::{ser::{PrettyConfig, to_string_pretty}, de::from_reader};
use serde::{Deserialize, Serialize};



impl Scene{

    pub fn new() -> Scene{
        Scene {  entity: None, modifiers: Vec::new(), name: None }
    }

    pub fn sort(&mut self){
        todo!()
        //self.modifiers.sort_by(|a, b| b.0.cmp(&a.0));
    }
    pub fn name(&mut self, name: &str){
        self.name = Some(name.to_owned());
    }
    pub fn push_modifier(&mut self, modifier: Modifier){
        self.modifiers.push(modifier);
    }
    pub fn push_component(&mut self, component: impl ComponentInterface, is_export: bool){
        self.modifiers.push(Modifier::Component(is_export, Box::new(component)));
    }
    pub fn push_bundle(&mut self, bundle: impl BundleMarker + 'static, is_export: bool){
        self.modifiers.push(Modifier::Bundle(is_export ,Box::new(bundle)));
    }
    pub fn push_child(&mut self, child: Scene, is_export: bool){
        self.modifiers.push(Modifier::Scene(is_export, child));
    }
    pub fn spawn(&mut self) -> Entity{
        let entity = World::spawn();
        self.construct(entity);
        entity
    }
   
    pub fn expand(&mut self){
        todo!()
    }
    pub fn expand_bundles(&mut self){
        todo!()
    }
    pub fn expand_nodes(&mut self){
        todo!()
    }
    pub fn remove_entities(&mut self){
        todo!()
    }

    fn construct(&mut self, entity: Entity){

        for modifier in self.modifiers.iter_mut(){

            match modifier{
                Modifier::Scene(_, scene) => {
                    let _ = entity;scene.spawn();
                },
                Modifier::Component(_, component) => {
                    component.add_entity(entity);
                },
                Modifier::Bundle(_, bundle) => {
                    for component in bundle.to_components(){
                        component.add_entity(entity);
                    }
                },
                Modifier::Node(_, _node, _modifiers, _removed) => {
                    //let instance = node.as_mut().spawn_method();
                },
            }
        }
    }
    pub fn save(&self, file_path: &'static str){

        let pretty = PrettyConfig::new()
        .depth_limit(2)
        .separate_tuple_members(true)
        .enumerate_arrays(false)
        .struct_names(true)
        .compact_arrays(false);
        let s = to_string_pretty(self, pretty).expect("Serialization failed");

        fs::write(file_path, s).expect("Unable to write file");
    }

    pub fn load(file_path: &'static str) -> Scene{
        let f = File::open(file_path).expect("Failed opening file");
        match from_reader(f) {
            Ok(x) => x,
            Err(e) => {
                println!("Failed to load config: {}", e);
    
                std::process::exit(1);
            }
        }
    }
    fn attach(){

    }
}


impl Display for Scene{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let out = format_scene("".to_owned(), self, "".to_string(), true, "".to_string());

        
        write!(f, "\n{}", out)
    }
}

fn format_scene(spacing: String, scene: &Scene, mut entity_prefix: String, _is_root: bool, export_scene: String) -> String{

    let mut out = entity_prefix + &export_scene + &match scene.name.clone(){
        Some(name) => format!("{export_scene}{name}  \n"),
        None => "Entity ∘\n".to_string(),
        //_ => format!("{export_scene} Entity ∘ \n"), 
    };

    for (index, modifier) in scene.modifiers.iter().enumerate(){
        let mut clone = spacing.clone();
        if index == (scene.modifiers.len() - 1){
            entity_prefix = "└── ".to_string();
            clone += "    ";
        } else {
            entity_prefix = "├── ".to_string();
            clone += "│    ";
        }
        
        match modifier{
            Modifier::Scene(is_export, scene) => {

                    let export_scene = match is_export{
                        true => { 
                            clone += "  " ;
                            "⋆ "
                        },
                        false => "",
                    }.to_owned();

                    let fn_out = format_scene(clone.clone(), scene, entity_prefix, false, export_scene).to_owned();

                        out += &format!("{}{}", spacing, fn_out);
                    
                    
                
            },
            Modifier::Component(is_export, component) => {

                    if !is_export{
                        out += &format!("{}{}{:?}\n", spacing, entity_prefix, component.as_ref());
                    } else {
                        out += &format!("{}{}⋆ {:?} \n", spacing, entity_prefix, component.as_ref());
                    }
                
            },
            Modifier::Bundle(is_export, bundle) => {

                    if !is_export{
                        out += &format!("{spacing}{entity_prefix}{} ≣\n", bundle.as_ref().name());
                    } else {
                        out += &format!("{spacing}{entity_prefix}⋆ {} ≣ \n", bundle.as_ref().name());
                        clone += " ";
                    }

                    out += &format_bundle(clone, "".to_string() ,bundle.as_ref());
                
            },
            Modifier::Node(is_export, node, _modifiers, _removed) => {

                    
                    if !is_export{
                        out += &format!("{spacing}{entity_prefix}{} ⋄\n", node.as_ref().name());
                    } else {
                        out += &format!("{spacing}{entity_prefix}⋆ {} ⋄ \n", node.as_ref().name());

                    }

            },
        }
    }
    out
}

fn format_bundle(spacing: String, mut entity_prefix: String, bundle: &dyn BundleMarker) -> String{
    let mut out = "".to_owned();

    let components = bundle.to_components();

    for (index, component) in components.iter().enumerate(){

        if index == (components.len() - 1){
            entity_prefix = "└── ".to_string();
        } else {
            entity_prefix = "├── ".to_string();
        }
        out += &format!("{spacing}{entity_prefix}{component:?}\n");
    }
    //out += &format!("{}{}{:?}\n", spacing, entity_prefix, bundle.as_ref());
    out
}

impl Shl<Scene> for Entity {
    type Output = Self;

    fn shl(self, _rhs: Scene) -> Self::Output {
        Scene::attach();
        //self.add(rhs)
        todo!()
    }
}

impl Shl<&str> for Scene {
    type Output = Self;

    fn shl(mut self, rhs: &str) -> Self::Output {

        self.name(rhs);
        self
    }
}


impl Shl<ExportedModifier> for Scene {
    type Output = Self;

    fn shl(mut self, rhs: ExportedModifier) -> Self::Output {
        self.push_modifier(rhs.0);
        self
    }
}

impl Shl<ExportedModifier> for SEntity {
    type Output = Scene;

    fn shl(self, rhs: ExportedModifier) -> Self::Output {
        let mut scene = Scene::new();
        scene.push_modifier(rhs.0);
        scene
    }
}

impl Shl<SNode> for Scene {
    type Output = Self;

    fn shl(mut self, rhs: SNode) -> Self::Output {
        self.push_modifier(rhs.to_modifier());
        self
    }
}

impl Shl for Scene {
    type Output = Self;

    fn shl(mut self, rhs: Scene) -> Self::Output {
        self.push_child(rhs, false);
        self
    }
}

impl Not for Scene {
    type Output = ExportedModifier;

    fn not(self) -> Self::Output {
        ExportedModifier(Modifier::Scene(true, self))
    }
}

impl Not for SNode {
    type Output = Self;

    fn not(mut self) -> Self::Output {
        self.is_export = !self.is_export;
        self
    }
}
// impl serde::Serialize for Scene {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: serde::Serializer,
//     {
//         //serializer.serialize_i32(*self)
//         for modifier in self.modifiers.iter(){

//         }
//         todo!()
//     }
// }
