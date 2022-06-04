extern crate slug;
use crate::blog::Blog;
use slug::slugify;
use std::collections::HashMap;

pub(crate) fn generate_slug(title: &String) -> String {
    slugify(&title)
}

pub(crate) fn get_id_for_blog(blogs: &HashMap<String, Blog>) -> String {
    let blog_lenght = blogs.keys().len();

    if blogs.contains_key(&blog_lenght.to_string()) {
        (blog_lenght + 1).to_string()
    } else {
        blog_lenght.to_string()
    }
}
