use crate::*;

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone, Debug)]
#[serde(crate = "near_sdk::serde")]

pub struct Blog {
    pub author_id: AccountId,
    pub category_id: i32,
    pub title: String,
    pub slug: String,
    pub view_number: i32,
    pub content: String,
}

impl Blog {
    pub fn add_blog(title: String, content: String, category_id: i32) -> Blog {
        let blog = Blog {
            author_id: env::signer_account_id(),
            category_id,
            slug: generate_slug(&title),
            view_number: 0,
            content,
            title: title,
        };

        blog
    }
}
