use serde::{Deserialize, Serialize};

arg_enum! {
    #[allow(non_camel_case_types)]
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize, Hash)]
    pub enum Status {
        OPEN,
        DOING,
        CLOSED,
    }
}