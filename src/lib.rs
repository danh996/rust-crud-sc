use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, near_bindgen, AccountId};
use std::collections::HashMap;

use crate::blog::*;
use crate::util::*;

mod blog;
mod util;
#[derive(BorshDeserialize, BorshSerialize)]
#[near_bindgen]
pub struct ListBlog {
    list_blog: HashMap<String, Blog>,
}

impl Default for ListBlog {
    fn default() -> Self {
        Self {
            list_blog: HashMap::new(),
        }
    }
}

#[near_bindgen]
impl ListBlog {
    pub fn add_blog(&mut self, title: String, content: String) {
        let blog = Blog {
            author_id: env::signer_account_id(),
            slug: generate_slug(&title),
            view_number: 0,
            content,
            title: title,
        };

        let list_blog_lenght = self.list_blog.keys().len();

        self.list_blog
            .insert((list_blog_lenght + 1).to_string(), blog);
    }

    pub fn get_blog_by_id(self, blog_id: String) -> Blog {
        self.list_blog.get(&blog_id).unwrap().clone()
    }

    pub fn get_all_blogs(self) -> HashMap<String, Blog> {
        self.list_blog
    }

    pub fn delete_blog(&mut self, blog_id: String) {
        self.list_blog.remove(&blog_id);
    }

    pub fn edit_blog(&mut self, blog_id: String, content: String) {
        let blog = self.list_blog.get(&blog_id);

        match blog {
            Some(blog) => {
                let new_blog = Blog {
                    content: content,
                    ..blog.clone()
                };

                self.list_blog.insert(blog_id, new_blog);
            }
            None => (),
        }
    }

    pub fn update_blog_view(&mut self, blog_id: String) {
        let blog = self.list_blog.get(&blog_id);

        match blog {
            Some(blog) => {
                let new_blog = Blog {
                    view_number: blog.clone().view_number + 1,
                    ..blog.clone()
                };

                self.list_blog.insert(blog_id, new_blog);
            }
            None => (),
        }
    }
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use super::*;
    use near_sdk::test_utils::{accounts, VMContextBuilder};
    use near_sdk::{testing_env, MockedBlockchain};

    fn get_context(is_view: bool) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder
            .current_account_id(accounts(0))
            .signer_account_id(accounts(0))
            .predecessor_account_id(accounts(0))
            .is_view(is_view);

        builder
    }

    #[test]
    fn test_create_blog() {
        let context = get_context(false);
        testing_env!(context.build());
        let mut contract = ListBlog::default();

        contract.add_blog("test blog".to_string(), "Test blog content".to_string());

        let blog = contract.get_blog_by_id("1".to_string());

        // println!("value of blog is {:?}", contract.list_blog);

        assert_eq!(blog.title, "test blog".to_string());
        assert_eq!(blog.content, "Test blog content".to_string());
        assert_eq!(blog.slug, "test-blog".to_string());
        assert_eq!(blog.view_number, 0);
    }
}
