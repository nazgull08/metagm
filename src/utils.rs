use crate::types::{MetaGraph, MetaVertex, MetaEdge};


pub fn example_metagraph() -> MetaGraph<i32>{
let a = MetaVertex{ id: 0, payload: 10, metagraph: None};
    let b = MetaVertex { id: 1, payload: 20, metagraph: None };
    let e1 = MetaEdge {
        id: 0,
        payload: "a",
        vertex_start: a.id,
        vertex_end: b.id,
        oriented: false,
        metagraph: None,
    };
}
