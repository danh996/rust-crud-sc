use crate::*;

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone, Debug)]
#[serde(crate = "near_sdk::serde")]

pub struct Blog {
    pub author_id: AccountId,
    pub title: String,
    pub slug: String,
    pub view_number: i32,
    pub content: String,
}
