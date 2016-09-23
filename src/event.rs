use std::collections::HashMap;

pub type EntityChanged = fn(Entity, i32, ) -> i32;
pub type ComponentReplaced = fn(i32) -> i32;
pub type EntityReleased = fn(i32) -> i32;
pub type GroupChanged = fn(i32) -> i32;
pub type PoolChanged = fn(i32) -> i32;
pub type PoolGroupChanged = fn(i32) -> i32;


pub struct Event<T> {
    named_listeners: HashMap<String, T>,
    anonymous_listener: Vec<T>,
}

impl<T> Event<T> {
    pub fn addListener(&mut self, methodName: &str, namedEventHandlerMethod: T) {
        if !self.named_listeners.contains_key(methodName) {
            self.named_listeners.insert(methodName.to_string(), namedEventHandlerMethod);
        }
    }

    pub fn removeListener( & mut self, methodName: &str) {
        if self.named_listeners.contains_key(methodName) {
            self.named_listeners.remove(methodName);
        }
    }

    pub fn addAnonymousListener(&mut self, namedEventHandlerMethod: T) {
         self.anonymous_listener.push(namedEventHandlerMethod);

    }

    pub fn listeners(&self) -> Vec<&T> {
        let mut all_listeners =  Vec::new();
        for lta in &self.anonymous_listener {
            all_listeners.push(lta);
        }
        for (_, ltn) in &self.named_listeners {
            all_listeners.push(ltn);
        }
       // all_listeners.extend(self.anonymous_listener.as_slice())
        all_listeners
    }

    pub fn clear(&mut self) {
        self.named_listeners.clear();
        self.anonymous_listener.clear();
    }
}