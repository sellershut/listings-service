use serde::Deserialize;
use time::OffsetDateTime;

#[derive(Debug, Deserialize, Clone)]
pub struct Listing {
    id: String,
    user_ap_id: String,
    local: bool,
    title: String,
    description: String,
    expires_at: Option<OffsetDateTime>,
    created_at: OffsetDateTime,
    updated_at: OffsetDateTime,
    ap_id: String,
    active: bool,
    quantity: i32,
    price: f32,
    liked_by: Vec<String>,
    category_ap_id: String,
    status: String,
}

impl From<Listing> for sellershut_core::listings::Listing {
    fn from(value: Listing) -> Self {
        Self {
            id: value.id,
            user_ap_id: value.user_ap_id,
            local: value.local,
            title: value.title,
            description: value.description,
            expires_at: value.expires_at.map(Into::into),
            created_at: Some(value.created_at.into()),
            updated_at: Some(value.updated_at.into()),
            ap_id: value.ap_id,
            active: value.active,
            quantity: value.quantity,
            status: todo!(),
            price: todo!(),
            liked_by: value.liked_by,
            category_ap_id: value.category_ap_id,
        }
    }
}
