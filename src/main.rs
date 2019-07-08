use crate::element::element::Element;
use crate::extension::conf::Conf;
use crate::extension::extension::Extension;
use crate::key::Key;

mod key;
mod extension;
mod element;
mod file;
mod files;

fn main() {
    let element_text = Element::new(Key::new("content_elements", "text"))
        .icon("text");

    let element_image = Element::new(Key::new("content_elements", "image"))
        .icon("image");

    let conf = Conf::new()
        .key("content_elements")
        .title("Content Elements")
        .description("Content Elements Extension");

    let extension = Extension::new(conf)
        .add_element(element_text)
        .add_element(element_image);

    extension.build();
}
