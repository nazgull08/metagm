use crate::types::{MetaGraph, MetaVertex, MetaEdge, PayloadWrapper};


pub fn example_metagraph() -> MetaGraph<PayloadWrapper>{
let a = MetaVertex{ id: 0, payload: PayloadWrapper::Pi32(2), metagraph: None};
    let b = MetaVertex { id: 1, payload:PayloadWrapper::Pi32(5), metagraph: None };
    let e1 = MetaEdge {
        id: 0,
        payload:PayloadWrapper::PString("small edge".to_owned()) ,
        vertex_start: a.id,
        vertex_end: b.id,
        oriented: false,
        metagraph: None,
    };
    let mvs = vec![a,b];
    let mes = vec![e1];
    return MetaGraph{ id: 0, vertecies: vec![], meta_vertecies: mvs, edges: vec![], meta_edges: mes};
}
