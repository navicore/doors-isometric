#[derive(Debug, Clone, Eq, PartialEq)]
pub struct IntegrationResource {
    pub name: String,
    pub kind: String,
    pub parent: Option<Box<Self>>,
    pub children: Vec<Self>,
}

impl IntegrationResource {
    pub fn new(name: String, kind: String, parent: Option<Self>, children: Vec<Self>) -> Self {
        Self {
            name,
            kind,
            parent: parent.map(Box::new),
            children,
        }
    }
}
