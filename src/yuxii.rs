use core::time;
use std::thread;


use static_init::dynamic;

use crate::default_res::EngineConfig;
use crate::{Node, default_res::Time};
use crate::table::ResourceInterface;




#[dynamic]
pub static mut YUXII: Yuxii = Yuxii::new();

pub struct Yuxii{
    nodes: Vec<Box<dyn Node>>,
}
#[allow(dead_code)]
impl Yuxii{
    fn new() -> Yuxii{
        Yuxii{
            nodes: Vec::new(),
        }
    }
    async fn frame(&mut self){

        for nodes_boxed in self.nodes.iter_mut(){
            let node = nodes_boxed.as_mut();
  
            node.frame().await;
        }
    }

    pub async fn run(){
        use std::time::Instant;
        let mut yuxii = Yuxii::write();
        for i in 0..2000{
            
            let now = Instant::now();

            yuxii.frame().await;

            let engine_config_lck = EngineConfig::read();


            if let Some(fps) = engine_config_lck.limit_fps{
                if fps > 0{
                    let time = time::Duration::from_millis(1000/(fps as u64));

                    thread::sleep(time);
                }
            }

            
            let mut tl = Time::write();
            for u in 0..100000 {
                
                tl.delta_time = u as f32 * i as f32;
            }


            let elapsed = now.elapsed();
            
            tl.delta_time = elapsed.as_secs_f32();
            tl.delta_time_micros = elapsed.as_micros();
            tl.delta_time_nanos = elapsed.as_nanos();
            tl.delta_time_millis = elapsed.as_millis();

            // dbg!(elapsed.as_nanos());

            // dbg!(tl.delta_time_micros, tl.delta_time_millis, tl.delta_time_nanos );

            //panic!();



        }
    }
    pub(crate) fn read() -> static_init::lazy::lesser_locked_lazy::ReadGuard<'static, Yuxii>{
        YUXII.read()
    }
    pub(crate) fn write() -> static_init::lazy::lesser_locked_lazy::WriteGuard<'static, Yuxii>{
        YUXII.write()
    }
    pub(crate) fn plug_node_without_init<T: Node + 'static + Send>(&mut self, node: T) -> &mut Yuxii{
        //plugin.init();
        self.nodes.push(Box::new(node));
        self
    }
    pub fn plug_node<T: Node + 'static + Send>(&mut self, node: T) -> &mut Yuxii{
        node.init();
        self.nodes.push(Box::new(node));
        self
    }
    
}

