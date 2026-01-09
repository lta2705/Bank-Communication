use crate::{
    app::error::AppError, dto::vietqr_req_dto::VietQrReqDto, dto::vietqr_resp_dto::VietQrRespDto,
};

pub struct VietQrService;

impl VietQrService {
    pub fn new() -> Self {
        Self
    }

    pub async fn create_qr(&self, payload: VietQrReqDto) -> Result<VietQrRespDto, AppError> {
        // business logic (placeholder)
        let _model: crate::models::vietqr_req::VietQrReq = payload.into();

        // Build a sample response. Replace with real logic: persistence, external calls, validations, etc.
        let data = crate::models::vietqr_resp::Data {
            qr_code: "SAMPLE_CODE".to_string(),
            qr_data_url: "https://example.com/qr/SAMPLE".to_string(),
        };

        let resp = crate::models::vietqr_resp::VietQrResp {
            code: "00".to_string(),
            desc: "Success".to_string(),
            data,
        };

        Ok(resp.into())
    }
}
