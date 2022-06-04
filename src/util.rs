extern crate slug;
use slug::slugify;

pub(crate) fn generate_slug(title: &String) -> String {
    slugify(&title)
}
