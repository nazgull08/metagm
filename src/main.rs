use metagm::types::{Vertex, MetaEdge};

fn main() {
    let a = Vertex{id:0, payload:10};
    let b = Vertex{id:1, payload:20};
    let e1 = MetaEdge{id:0,
                                payload: 15,
                                vertex_start: a.id,
                                vertex_end: b.id,
                                oriented: false,
                                metagraph: None};
    println!("==============");
    println!("{:?}",e1);
}
