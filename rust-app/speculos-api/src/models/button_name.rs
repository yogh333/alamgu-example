/*
 * Speculos API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */


/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ButtonName {
    #[serde(rename = "left")]
    Left,
    #[serde(rename = "right")]
    Right,
    #[serde(rename = "both")]
    Both,

}

impl ToString for ButtonName {
    fn to_string(&self) -> String {
        match self {
            Self::Left => String::from("left"),
            Self::Right => String::from("right"),
            Self::Both => String::from("both"),
        }
    }
}



