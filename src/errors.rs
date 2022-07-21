use thiserror::Error;

#[derive(Error, Debug)]
pub enum ObjError {
    #[error("Invlaid OBJ file: {0}")]
    Gltf(#[from] obj::ObjError),

    #[error("Unknown vertex format")]
    UnknownVertexFormat
}