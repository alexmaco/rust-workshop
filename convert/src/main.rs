use csv::Reader;

fn build_table_header(reader: &mut Reader<&[u8]>) -> Result<Node, csv::Error> {
    let mut thead = Node::from("thead");
    let mut title_row = Node::from("tr");

    // Task 1:
    //      - iterate csv headers
    //      - create cells for the html table header with <td>
    //      - set the csv header as text for the cell
    //      - add the new cell to title_row
    for head in reader.headers()? {
        title_row.add_child(Node::from("td").with_text(head));
    }

    thead.add_child(title_row); // now thead contains the <tr> that is title_row

    Ok(thead)
}

fn build_table_body(reader: &mut Reader<&[u8]>) -> Result<Node, csv::Error> {
    let mut tbody = Node::from("tbody");

    // Task 2:
    //      - iterate csv records
    //      - for each record create a row with <tr>
    //      - create a cell for each string with <td>
    //      - set the csv entry as the text for cell
    //      - **NEW** add the new <td> cell as a child for the <tr> row
    //      - add the new <tr> row as a child of the <tbody> node

    for record in reader.records() {
        let mut data_row = Node::from("tr");

        for cell in record?.iter() {
            data_row.add_child(Node::from("td").with_text(cell));
        }

        tbody.add_child(data_row);
    }

    Ok(tbody)
}

fn main() {
    let csv = "\
year,make,model,description
1948,Porsche,356,Luxury sports car
1967,Ford,Mustang fastback 1967,American car";

    let mut reader = csv::Reader::from_reader(csv.as_bytes());
    let mut table = Node::from("table");

    // Task 3:
    //      - call build_table_header with the csv reader
    //      - obtain the table header node
    //      - call build_table_body with the csv reader
    //      - obtain the table body node
    //      - add both nodes as children to the "table" node

    let thead = build_table_header(&mut reader).unwrap();
    let tbody = build_table_body(&mut reader).unwrap();
    table.add_child(thead);
    table.add_child(tbody);

    let html_doc = create_html_only_with_table(table);

    // Task 4:
    //      - read the api docs (or source) and find how to print html_doc as a string
    println!("{}", html_doc.to_string_pretty());
}

fn create_html_only_with_table(table: Node) -> Node {
    let body = Node::from("body").with_child(table);

    let style = Node::from("style").with_text(
        "
        thead {color:green;}
        tbody {color:blue;}
        tfoot {color:red;}

        table, th, td {
          border: 1px solid black;
        }",
    );

    let head = Node::from("head").with_child(style);
    Node::from("html").with_child(head).with_child(body)
}

use html_build::Node;

/// Create Nodes with `Node::from`
/// Output readable html with `Node::to_string_pretty`
pub mod html_build {

    //use std::collections::HashMap;

    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub struct Node {
        name: String,
        content: Option<Content>,
        // TODO
        //attr: HashMap<String, String>,
    }

    impl From<&str> for Node {
        fn from(s: &str) -> Self {
            Self {
                name: s.into(),
                content: None,
                // TODO
                //attr: HashMap::new(),
            }
        }
    }

    /// Getter methods
    impl Node {
        /// Get the name with which this node was created
        pub fn name(&self) -> &str {
            &self.name
        }

        /// If node contains text, returns it
        pub fn text(&self) -> Option<&str> {
            match &self.content {
                Some(Content::Text(s)) => Some(&s),
                _ => None,
            }
        }

        /// If node has children, returns an iterator over them
        pub fn children(&self) -> Option<impl Iterator<Item = &Node>> {
            match &self.content {
                Some(Content::Children(list)) => Some(list.iter()),
                _ => None,
            }
        }

        /// Creates a string with human-readable html
        pub fn to_string_pretty(&self) -> String {
            let mut buf = String::new();
            to_string_pretty_buf(self, 0, &mut buf);
            buf
        }
    }

    /// Builder methods
    impl Node {
        /// Constructs a new Node, with the given text
        /// If node had children, it does nothing
        pub fn with_text(mut self, text: &str) -> Self {
            self.set_text(text);
            self
        }

        /// Constructs a new Node, with the given child
        /// If node had text, it does nothing
        pub fn with_child(mut self, node: Node) -> Self {
            self.add_child(node);
            self
        }
    }

    /// Setter methods
    impl Node {
        /// Add the given child to the node
        /// If node already has text, it does nothing
        pub fn add_child(&mut self, node: Node) {
            match self.content.as_mut() {
                Some(Content::Children(list)) => list.push(node),
                None => self.content = Some(Content::Children(vec![node])),
                _ => {}
            }
        }

        /// Set the node to contain the given text
        /// If node already has children, it does nothing
        pub fn set_text(&mut self, text: &str) {
            match self.content.as_mut() {
                Some(Content::Text(s)) => *s = text.into(),
                None => self.content = Some(Content::Text(text.into())),
                _ => {}
            }
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    enum Content {
        Children(Vec<Node>),
        Text(String),
    }

    /// implementation auxiliary, to avoid alloc thrashing
    fn to_string_pretty_buf(node: &Node, indent: usize, buf: &mut String) {
        static TAB: &str = "    ";
        let left = TAB.repeat(indent);

        // open the tag and write name
        buf.push_str(&left);
        buf.push_str("<");
        buf.push_str(&node.name);

        let content = match &node.content {
            Some(c) => c,
            _ => {
                // empty tag, close immediately
                buf.push_str("/>\n");
                return;
            }
        };

        // end of opening tag
        buf.push_str(">\n");

        match content {
            Content::Children(list) => {
                for child in list {
                    to_string_pretty_buf(child, indent + 1, buf);
                }
            }
            Content::Text(text) => {
                buf.push_str(&left);
                buf.push_str(TAB);
                buf.push_str(&text);
                buf.push_str("\n");
            }
        }

        buf.push_str(&left);
        buf.push_str("</");
        buf.push_str(&node.name);
        buf.push_str(">\n");
    }
}
