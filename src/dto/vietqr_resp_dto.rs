use serde::{Deserialize, Serialize};

use crate::models::payos_qr_resp::{Data, VietQrResp};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DataDto {
    pub qr_code: String,
    pub qr_data_url: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VietQrRespDto {
    pub code: String,
    pub desc: String,
    pub data: DataDto,
}

impl From<DataDto> for Data {
    fn from(dto: DataDto) -> Self {
        Self {
            qr_code: dto.qr_code,
            qr_data_url: dto.qr_data_url,
        }
    }
}

impl From<Data> for DataDto {
    fn from(model: Data) -> Self {
        Self {
            qr_code: model.qr_code,
            qr_data_url: model.qr_data_url,
        }
    }
}

impl From<VietQrRespDto> for VietQrResp {
    fn from(dto: VietQrRespDto) -> Self {
        Self {
            code: dto.code,
            desc: dto.desc,
            data: dto.data.into(),
        }
    }
}

impl From<VietQrResp> for VietQrRespDto {
    fn from(model: VietQrResp) -> Self {
        Self {
            code: model.code,
            desc: model.desc,
            data: model.data.into(),
        }
    }
}
