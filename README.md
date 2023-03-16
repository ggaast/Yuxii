Yuxii is an ECS-based 3D game engine.

Main purpose is to be very simple to use and learn.

Warning! Nothing is done yet.

# *Getting started:*

## Nodes

In Yuxii Node is a logic and a scene at once.

Lets see an Example:

```rust
/* file: src/my_node/mod.rs */

#[node]
pub struct MyNode;

#[node]
impl MyNode{
  fn init(&self){ }
  fn frame(&self) { }
  fn scene() -> Scene { }
}
```

Alright, we just created a node.

`init`   is called when we start the program

`frame`  is called each frame asynchronically . You can put there systems to execute.

`scene` is for spawning entities and their modifiers

But without connecting this node to main program it cant do any logic

So lets fix it and *plug* our node logic in the world:
```rust
/* file: src/main.rs */
fn main(){
    MyNode.plug();

    // We can also spawn this node content:
    MyNode::spawn();

}
```
### Structure of the node looks like this:
    NodeName (Folder)
        mod.rs
        resources.rs
        scene.ron
        systems.rs

todo!();