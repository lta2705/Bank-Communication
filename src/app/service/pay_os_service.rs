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

    }
}
