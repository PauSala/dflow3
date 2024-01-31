use serde::{Deserialize, Serialize};

use crate::modules::query::model::query_builder::abstract_query::AbstractQueryDto;

#[allow(non_snake_case)] //Is a DTO and we don't want to map this props betwen front/back
#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct PanelLayoutDto {
    pub(crate) x: usize,
    pub(crate) y: usize,
    pub(crate) w: usize,
    pub(crate) h: usize,
    pub(crate) minH: usize,
    pub(crate) minW: usize,
}
#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct ChartProps{
    chart_type: String
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) enum PanelContentType {
    Chart(ChartProps),
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct PanelPropsDto {
    pub(crate) name: String,
    pub(crate) content_type: PanelContentType,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct JsonPanel {
    pub(crate) layout: PanelLayoutDto,
    pub(crate) props: PanelPropsDto,
    pub(crate) query: Option<AbstractQueryDto>
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct PanelDto {
    pub(crate) panel_id: String,
    pub(crate) user_id: String,
    pub(crate) model_id: String,
    pub(crate) panel: JsonPanel
}


