use yuxii_macros::Component;
use serde::{Serialize, Deserialize};
use crate::*;


#[derive(Component, Clone, Serialize, Deserialize ,Default, Debug)]
pub struct Name(pub String);

