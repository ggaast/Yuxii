
use async_trait::async_trait;
use crate::{yuxii::Yuxii, Entity, world::{self}, Scene};


#[typetag::serde(tag = "type")]
#[async_trait]
pub trait Node: Send + Sync + core::fmt::Debug {

    fn init(&self) { }

    async fn frame(&self) { }

    fn exit(&self) { }

    fn plug(self) where Self: 'static + Sized {
        self.init();
        Yuxii::write().plug_node_without_init(self);
    }
    fn scene() -> Scene where Self: Sized{
        // Scene::attach("scene.ron", entity);
        todo!()
        
    }
    fn name(&self) -> String {
        let path = std::any::type_name::<Self>().to_string();

        let mut last = "";

        for s in path.split("::"){
            last = s;
        }
        last.to_string()
    }
}

pub trait Spawner: Node + Default{
    fn spawn() -> Entity where Self: 'static + Sized{

        let entity = Self::scene().spawn();
        
        world::WORLD.write().node_roots.insert(entity, Box::new(Self::new()));
        entity
        
    }

    fn spawn_method(&self) -> Entity where Self: 'static + Sized{

        let entity = Self::scene().spawn();
        
        world::WORLD.write().node_roots.insert(entity, Box::new(Self::new()));
        entity
        
    }

    fn new() -> Self where Self: Sized{
        Self::default()
    }
}