use bluster::mesh::ObjectSet;
use bluster::pipeline::query_pipeline::QueryPipeline;

pub struct SynergyState {
    pub objects: ObjectSet,
    pub pipeline: QueryPipeline,
}