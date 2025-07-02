struct Node { // node of a DOM tree
    // data common to all nodes:
    children: Vec<Node>,

    // data specific to each node type:
    node_type: NodeType,
}

enum NodeType { // the type of node, which determines how it should be rendered
    Text(String),
    Element(ElementData),
}

struct ElementData { // tag name and the attributes of that specific tag
    tag_name: String,
    attrs: AttrMap,
}

type AttrMap = HashMap<String, String>; // attribute to value mapping

fn text(data: String) -> Node { // text nodes
    Node { children: Vec::new(), node_type: NodeType::Text(data) }
}

pub fn elem(tag_name: String, attrs: AttrMap, children: Vec<Node>) -> Node { // element nodes
    Node {
        children,
        node_type: NodeType::Element(ElementData { tag_name, attrs })
    }
}
