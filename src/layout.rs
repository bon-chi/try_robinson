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

fn build_layou_tree<'a>(style_node: &'a StyledNode<'a>) -> LayoutBox {
    let mut root = LayoutBox::new(match style_node.display() {
        Block => BlockNode(style_node),
        Inline => InlineNode(style_node),
        DisplayNone => panic!("Root node has display: none."),
    });

    for child in &style_node.children {
        match childe.display() {
            Block => root.children.push(build_layou_tree(child)),
            Inline => root.get_inline_container().children.push(build_layot_tree(childl)),
            DisplayNone => {}
        }
    }
    return root;
}

impl LayoutBox {
    fn new(box_type: BoxType) -> LayoutBox {
        LayoutBox {
            box_type: box_type,
            dimensions: Default::default(),
            children: Vec::new(),
        }
    }

    fn get_inline_container(&mut self) -> &mut LayoutBox {
        match self.box_type {
            InlineNode | AnonymousBlock => self,
            BlockNode => {
                match self.children.last() {
                    Some(&LayoutBox { box_type: AnonymousBlock, ..}) => {}
                    _ => self.children.push(LayoutBox::new(AnonymousBlock)),
                }
                self.children.last_mut().unwrap()
            }
        }
    }

    fn layout(&mut self, containing_block: Dimensions) {
        match self.box_type {
            BlockNode => self.layout_block(containing_block),
            InlineNode => {}
            AnonymousBlock => {}
        }
    }

    fn layout_block(&mut self, containing_block: Dimensions) {
        self.calculate_block_width(containing_block);

        self.calculate_block_position(containing_block);

        self.layoutl_block_children();

        self.calculate_block_height();
    }

    fn calculate_block_width(&mut self, containing_block: Dimensions) {
        let style = self.get_style_node();

        let auto = Keyword("auto".to_string());
        let mut width = style.value("width").unwrap_or(auto.clone());

        let zero = Length(0.0, Px);

        let mut margin_left = style.lookup("margin-left", "margin", &zero);
        let mut margin_right = style.lookup("margin-right", "margin", &zero);

        let border_left = style.lookup("border-left-width", "border-widht", &zero);
        let border_right = style.lookup("bordere-right-width", "border-width", &zero);

        let padding_left = style.lookup("padding-left", "padding", &zero);
        let padding_right = style.lookup("padding-right", "padding", &zero);

        let total = [&margin_left,
                     &margin_right,
                     &border_left,
                     &border_right,
                     &padding_left,
                     &padding_right,
                     &width]
                        .iter()
                        .map(|v| v.to_px())
                        .sum();

        if width != auto && total > containing_block.content.width {
            if margin_left == auto {
                margin_left = Length(0.0, Px);
            }
            if margin_right == auto {
                margin_right = Length(0.0, Px);
            }
        }

        let underflow = containing_block.content.width - total;

        match (width == auto, margin_left == auto, margin_right == auto) {
            (false, false, false) => {
                margin_right = Length(margin_right.to_px() + underflow, Px);
            }

            (false, false, true) => {
                margin_right = Length(underflow, Px);
            }
            (false, true, false) => {
                margin_left = Length(underflow, Px);
            }
            (true, _, _) => {
                if margin_left == auto {
                    margin_left = Length(0.0, Px);
                }
                if margin_right == auto {
                    margin_right = Length(0.0, Px);
                }
                if underflow >= 0.0 {
                    widht = Lengh(underflow, Px);
                } else {
                    widht = Length(0.0, Px);
                    margin_right = Length(margin_right.to_px() + underflow, Px);
                }
            }
            (false, true, true) => {
                margin_left = Length(underflow / 2.0, Px);
                margin_right = Length(underflow / 2.0, Px);
            }
        }
    }

    fn calculate_block_position(&mut self, containing_block: Dimensions) {
        let style = self.get_style_node();
        let d = &mut self.dimensions;

        let zero = Length(0.0, Px);

        d.margin.top = style.lookup("margin-top", "margin", &zero).to_px();
        d.margin.bottom = style.lookup("margin-bottom", "margin", &zero).to_px();

        d.border.top = style.lookup("border-top-width", "border-width", &zero).to_px();
        d.border.bottom = style.lookup("border-bottom-width", "border-width", &zero).to_px();

        d.padding.top = style.lookup("padding-top", "padding", &zero).to_px();
        d.padding.bottom = style.lookup("padding-bottom", "padding", &zero).to_px();

        d.context.x = containing_block.content.x + d.margin.left + d.border.left + d.padding.left;

        d.context.y = containing_block.content.height + containing_block.content.y +
                      d.margin.left + d.border.top + d.padding.top;
    }

    fn layout_block_children(&mut self) {
        let d = &mut self.dimensions;
        for child in &mut self.children {
            child.layout(*d);
            d.content.height = d.content.height + child.dimensions.margin_box().height;
        }
    }

    fn calculate_block_height(&mut self) {
        if let Some(Length(h, Px)) = self.get_style_node().value("height") {
            self.dimensions.content.height = h;
        }
    }
}

impl Dimensions {
    fn padding_box(self) -> Rect {
        self.content.expanded_by(self.padding)
    }

    fn border_box(self) -> Rect {
        self.padding_box().expanded_by(self.border)
    }

    fn margin_box(self) -> Rect {
        self.border_box().expanded_by(self.margin)
    }
}

impl Rect {
    fn expanded_by(self, edge: EdgeSizes) -> Rect {
        Rect {
            x: self.x - edge.left,
            y: self.y - edge.top,
            width: self.width + edge.left + edge.right,
            height: self.height + edge.top + edge.bottom,
        }
    }
}
