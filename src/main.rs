fn main() {
    let graph_drawer = GraphDrawer::new();
    graph_drawer.draw();
}

struct Node {
    text: String,
}

struct GraphDrawerBuffer {
    buffer: String,
}

struct GraphDrawer {}

impl GraphDrawer {
    fn new() -> Self {
        Self {}
    }

    pub fn draw(&self) {
        let mut buffer = GraphDrawerBuffer {
            buffer: String::from(""),
        };
        buffer.buffer.push_str("hello");
        println!("{}", buffer.buffer);
    }

    fn draw_box(&self, node: Node) {}
}
