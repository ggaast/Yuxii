
#[derive(Debug, Default)]
pub struct StaticScenes(pub Vec<StaticScene>);

#[derive(Debug)]
pub struct StaticScene{

    pub entity: Entity,

    pub name: Option<String>,

    pub components: Vec<String>,

    pub children: Vec<StaticScene>
}

use std::fmt::Display;

use crate::Entity;

impl StaticScene{
    pub fn new(entity: Entity) -> StaticScene{
        StaticScene { entity, components: Vec::new(), children: Vec::new(), name: None }
    }
}

impl Display for StaticScene{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut vec = Vec::new();
        let out = format_static_scene(&mut vec, self, "".to_string());

        
        write!(f, "\n{}", out)
    }
}

fn format_static_scene(levels: &mut Vec<bool>, scene: &StaticScene, mut entity_prefix: String) -> String{

    let mut spacing = "".to_owned();
    

    for byte in levels.clone().iter(){
        if *byte{
            spacing += "│   ";
        } else {
            spacing += "    ";
        }
    }

    let mut out = entity_prefix + &match scene.name.clone(){
        Some(name) => format!("{} [{}]\n",  name, scene.entity.id),
        None => format!("{}\n", scene.entity),
    };

    for (index, component) in scene.components.iter().enumerate(){
        if index == (scene.components.len() - 1) && scene.children.is_empty(){
            out += &format!("{}└── {}\n", spacing, component);
        } else {
            out += &format!("{}├── {}\n", spacing, component);
        }
        
    }

    for (index, child) in scene.children.iter().enumerate(){

        let mut clone = levels.clone();

        if index == (scene.children.len() - 1){
            clone.push(false);
            entity_prefix = "└── ".to_string();
        } else {
            clone.push(true);
            entity_prefix = "├── ".to_string();
        }
        

        let fn_out = format_static_scene(&mut clone, child, entity_prefix).to_owned();
        out += &format!("{}{}", spacing, fn_out);
    }



    out
}

impl Display for StaticScenes{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {

        let mut out = "".to_owned();

        for static_scene in self.0.iter(){
            out += &format!("{}", static_scene);
        }
        write!(f, "\n{}", out)
    }
}
