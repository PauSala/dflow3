use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct PanelConfigDto {
    panels: Vec<String>
} 


#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct DashboardDto {
    pub(crate) id: String,
    pub(crate) user_id: String,
    pub(crate) model_id: String,
    pub(crate) name: String,
    pub(crate) config: PanelConfigDto
}
