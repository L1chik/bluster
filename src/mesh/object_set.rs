use std::ops::{Index, IndexMut};
use std::ptr::hash;

use crate::mesh::object::SceneObject;
use crate::mesh::object_parameters::ObjectHandle;
use crate::data::space::Space;


pub struct ObjectSet {
    pub objects: Space<SceneObject>,
    pub removed_objects: Vec<ObjectHandle>,
    pub changed_objects: Vec<ObjectHandle>,
}

impl ObjectSet {
    pub fn new() -> Self {
        Self {
            objects: Space::new(),
            removed_objects: Vec::new(),
            changed_objects: Vec::new(),
        }
    }

    pub fn insert(&mut self, obj: impl Into<SceneObject>) -> ObjectHandle {
        let mut obj = obj.into();

        obj.parent = None;
        let handle = ObjectHandle(self.objects.insert(obj));

        self.changed_objects.push(handle);

        println!("Some handl");
        handle
    }

    pub fn get(&self, handle: ObjectHandle) -> Option<&SceneObject> {
        self.objects.get(handle.0)
    }

    pub fn get_mut(&mut self, handle: ObjectHandle) -> Option<&mut SceneObject> {
        let result = self.objects.get_mut(handle.0)?;
        Some(result)
    }

    pub fn iter(&self) -> impl ExactSizeIterator<Item = (ObjectHandle, &SceneObject)> {
        self.objects.iter().map(|(h, o)| (ObjectHandle(h), o))
    }
}

impl Index<ObjectHandle> for ObjectSet {
    type Output = SceneObject;

    fn index(&self, index: ObjectHandle) -> & Self::Output {
        &self.objects[index.0]
    }
}

impl IndexMut<ObjectHandle> for ObjectSet {
    fn index_mut(&mut self, index: ObjectHandle) -> &mut Self::Output {
        let object = &mut self.objects[index.0];

        object
    }
}



