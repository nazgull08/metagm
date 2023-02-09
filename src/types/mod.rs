use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Vertex{
    pub id      : u32,
    pub payload : u32,
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Edge{
    pub id         : u32,
    pub payload    : u32,
    pub vertex_start : u32,
    pub vertex_end   : u32,
    pub oriented   : bool
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MetaVertex{
    pub id : i32,
    pub payload : i32,
    pub metagraph : Option<MetaGraph>
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MetaEdge{
    pub id : i32,
    pub payload      : u32,
    pub vertex_start : u32,
    pub vertex_end   : u32,
    pub metagraph : Option<MetaGraph>,
    pub oriented   : bool
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MetaGraph{
    pub id : i32,
    pub vertecies : Vec<Vertex>,
    pub meta_vertecies : Vec<MetaVertex>,
    pub edges : Vec<Edge>,
    pub meta_edges : Vec<MetaEdge>,
}
