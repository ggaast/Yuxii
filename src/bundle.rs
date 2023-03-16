use crate::{ComponentInterface};

#[typetag::serde(tag = "type")]
pub trait BundleMarker: ComponentInterface{
    fn to_components(&self) -> Vec<Box<dyn ComponentInterface>>;
    fn name(&self) -> String;
 }


