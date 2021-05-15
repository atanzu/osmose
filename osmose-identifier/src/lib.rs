use osmose_generated::generated_proto::osmose::Identifier as ProtoIdentifier;


#[derive(Debug, Hash, Clone)]
pub struct Identifier {
    name: String,
    id: u64,
}


impl Identifier {
    pub fn new() -> Self {
        Identifier { name: "".to_owned(), id: u64::from(std::process::id()) }
    }

    pub fn from_given(name: &str, id: u64) -> Self {
        Identifier { name: name.to_owned(), id: id }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_id(&self) -> u64 {
        self.id
    }
}


impl From<&ProtoIdentifier> for Identifier {
    fn from(proto_id: &ProtoIdentifier) -> Self {
        Identifier{
            name: proto_id.get_name().to_owned(),
            id: proto_id.get_id()
        }
    }
}


impl From<&Identifier> for ProtoIdentifier {
    fn from(id: &Identifier) -> Self {
        let mut proto_id = ProtoIdentifier::new();
        proto_id.set_name(id.get_name().to_owned());
        proto_id.set_id(id.get_id());
        proto_id
    }
}


impl PartialEq for Identifier {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.id == other.id
    }
}


impl Eq for Identifier {}

