struct Dimensions {
    content: Rect,

    padding: EdgeSizes,
    border: EdgeSizes,
    margin: EdgeSizes,
}

struct Rect {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
}

struct EdgeSizes {
    left: f32,
    right: f32,
    top: f32,
    bottom: f32,
}

struct LayoutBox<'a> {
    dimensions: Dimensions,
    box_types: BoxType<'a>,
    children: Vec<LayoutBox<'a>>,
}

enum BoxType<'a> {
    BlockNode(&'a StyleNode<'a>),
    InlineNode(&'a StyleNode<'a>),
    AnonymousBlock,
}
