Yuxii is an ECS-based 3D game engine.

Main purpose is to be very simple to use and learn.

Warning! Nothing is done yet.

# *Getting started:*

### Nodes

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

    // Run
    Yuxii::run();

}
```
### Structure of the node looks like this:
    NodeName (Folder)
        mod.rs
        resources.rs
        scene.ron
        systems.rs

### Systems

You can easily add system from any part of your project, but after `Yuxii::run()`

```rust
#[system]
fn my_system(){ /* do something */ }
```

and add it:
```rust
fn frame(&self){
    my_system();
}
```

### Queries

Queries are very powerfull in this engine (should be, not yet)
Lets get into example:
```rust
#[system]
fn my_system(){
    // This query will be removed by macro and will not appear in docs
    /// *GetMut, &GetRef, Added+, &OptionRef?
    {
        get_mut.field = "oh, hey";
    }

    // And we have even better parenting
    /// Parent:: *GetMut
    {
        parent_get_mut.field = "looks easy";

    }
    // Custom naming
    /// *[my_vel]Velocity
    {
        my_vel;
    }
}
```

## Resources

### Components
```rust
#[derive(Component)]
pub struct MyComponent{
    /// def 9
    pub field: i32,
    /// def "Hello World"
    pub private: String
}
```

It will automatically implement Default according to docs attributes.

Add component:
```rust
let entity = World::spawn();

entity << MyComponent{ field: 65, ..default() };
```
### Globals

```rust
#[derive(Global)]
pub struct MyGlobal{
    /// def "It is global" 
    pub info: String
}
```
globals instances are automatically adding to the world at their definition. It means, you can access it after its definition.

```rust
fn system(){
    let lock = MyGlobal::read();

    log << "MyGlobal info: " << lock.info;
}
```

### Signals


todo!();