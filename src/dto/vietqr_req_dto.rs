use serde::{Serialize, Deserialize};

use crate::models::payos_qr_req::VietQrReq;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VietQrReqDto {
    pub account_no: String,
    pub account_name: String,
    pub acq_id: i32,
    pub amount: i32,
    pub add_info: String,
    pub format: String,
    pub template: String,
}

impl From<VietQrReqDto> for VietQrReq {
    fn from(dto: VietQrReqDto) -> Self {
        Self {
            account_no: dto.account_no,
            account_name: dto.account_name,
            acq_id: dto.acq_id,
            amount: dto.amount,
            add_info: dto.add_info,
            format: dto.format,
            template: dto.template,
        }
    }
}

impl From<VietQrReq> for VietQrReqDto {
    fn from(model: VietQrReq) -> Self {
        Self {
            account_no: model.account_no,
            account_name: model.account_name,
            acq_id: model.acq_id,
            amount: model.amount,
            add_info: model.add_info,
            format: model.format,
            template: model.template,
        }
    }
}