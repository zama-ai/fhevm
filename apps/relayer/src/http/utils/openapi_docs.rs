/// Chain Id
///
/// It does support an ID as an integer or a 0x prefixed hex string
#[derive(serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
#[serde(untagged)]
pub enum ChainId {
    #[schema(examples("0xaa36a7", "11155111"))]
    String(String),
    #[schema(example = 11155111)]
    Int(u64),
}
