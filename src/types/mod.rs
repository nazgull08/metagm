use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Vertex<T> {
    pub id: u32,
    pub payload: T,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Edge<T> {
    pub id: u32,
    pub payload: T,
    pub vertex_start: u32,
    pub vertex_end: u32,
    pub oriented: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MetaVertex<T> {
    pub id: u32,
    pub payload: T,
    pub metagraph: Option<MetaGraph<T>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MetaEdge<T> {
    pub id: u32,
    pub payload: T,
    pub vertex_start: u32,
    pub vertex_end: u32,
    pub metagraph: Option<MetaGraph<T>>,
    pub oriented: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MetaGraph<T> {
    pub id: u32,
    pub vertecies: Vec<Vertex<T>>,
    pub meta_vertecies: Vec<MetaVertex<T>>,
    pub edges: Vec<Edge<T>>,
    pub meta_edges: Vec<MetaEdge<T>>,
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum PayloadWrapper {
    PString(String),
    Pi32(i32),
}
