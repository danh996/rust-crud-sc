use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, near_bindgen, AccountId};
use std::collections::HashMap;

use crate::blog::*;

mod blog;

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
            title,
            slug: String::from("le-cong-danh"),
            view_number: 0,
            content,
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
