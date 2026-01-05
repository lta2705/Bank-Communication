use crate::{dto::vietqr_req_dto::VietQrReqDto, models::vietqr_req::VietQrReq};



pub struct VietQrService;

impl VietQrService {
    pub fn new() -> Self {
        Self
    }
    
    pub async fn create_qr(&self, payload: VietQrReqDto) -> VietQrReqDto {
        // business logicg 
        let model: VietQrReq = payload.into();
        
    }
    }
}
