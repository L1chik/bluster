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

        handle
    }

    pub fn get(&self, handle: ObjectHandle) -> Option<&SceneObject> {
        self.objects.get(handle.0)
    }
}



