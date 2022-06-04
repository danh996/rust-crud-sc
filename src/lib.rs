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
pub struct News {
    blogs: HashMap<String, Blog>,
    categories: HashMap<String, Blog>,
}

impl Default for News {
    fn default() -> Self {
        Self {
            blogs: HashMap::new(),
            categories: HashMap::new(),
        }
    }
}

#[near_bindgen]
impl News {
    fn add_blog(&mut self, title: String, content: String, category_id: i32) {
        let blog = Blog::add_blog(title, content, category_id);

        let id = get_id_for_blog(&self.blogs);

        self.blogs.insert(id, blog);
    }

    pub fn get_blog_by_id(self, blog_id: String) -> Blog {
        self.blogs.get(&blog_id).unwrap().clone()
    }

    pub fn get_all_blogs(self) -> HashMap<String, Blog> {
        self.blogs
    }

    pub fn delete_blog(&mut self, blog_id: String) {
        self.blogs.remove(&blog_id);
    }

    pub fn edit_blog(&mut self, blog_id: String, content: String) {
        let blog = self.blogs.get(&blog_id);

        match blog {
            Some(blog) => {
                let new_blog = Blog {
                    content: content,
                    ..blog.clone()
                };

                self.blogs.insert(blog_id, new_blog);
            }
            None => (),
        }
    }

    pub fn update_blog_view(&mut self, blog_id: String) {
        let blog = self.blogs.get(&blog_id);

        match blog {
            Some(blog) => {
                let new_blog = Blog {
                    view_number: blog.clone().view_number + 1,
                    ..blog.clone()
                };

                self.blogs.insert(blog_id, new_blog);
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
        let mut contract = News::default();

        contract.add_blog("test blog".to_string(), "Test blog content".to_string(), 1);
        println!("value of blog is {:?}", contract.blogs);

        let blog = contract.get_blog_by_id("0".to_string());

        assert_eq!(blog.title, "test blog".to_string());
        assert_eq!(blog.content, "Test blog content".to_string());
        assert_eq!(blog.slug, "test-blog".to_string());
        assert_eq!(blog.view_number, 0);
    }
}
