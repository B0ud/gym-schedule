#[derive(Deserialize, Serialize, Debug, Copy, Clone)]
pub struct PaginationQuery {
    pub limit: Option<u32>,
    pub offset: Option<u32>,
}
