pub struct Entity {}
pub struct IComponent {}


pub trait IMatcher {
    fn getIndices() -> Vec<i32>;
    fn matches(entity: &Entity) -> bool;
}


pub trait EntityChanged {
    fn entityChanged(entity: &Entity, index: i32, component: &IComponent);
}
