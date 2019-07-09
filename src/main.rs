#[macro_use]
extern crate clap;

use clap::{App, Arg, SubCommand};

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
    let matches = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .subcommand(SubCommand::with_name("extension")
            .about("Add a new extension")
            .arg(Arg::with_name("key")
                .short("k")
                .long("key")
                .help("Sets the extension key")
                .required(true)
                .takes_value(true)
            )
            .arg(Arg::with_name("title")
                .short("t")
                .long("title")
                .help("Sets the extension title")
                .required(true)
                .takes_value(true)
            )
            .arg(Arg::with_name("description")
                .short("d")
                .long("description")
                .help("Sets the extension description")
                .required(true)
                .takes_value(true)
            )
        )
        .subcommand(SubCommand::with_name("element")
            .about("Add a new element")
            .arg(Arg::with_name("key")
                .short("k")
                .long("key")
                .help("Sets the element key")
                .required(true)
                .takes_value(true)
            )
            .arg(Arg::with_name("title")
                .short("t")
                .long("title")
                .help("Sets the element title")
                .required(true)
                .takes_value(true)
            )
            .arg(Arg::with_name("description")
                .short("d")
                .long("description")
                .help("Sets the element description")
                .required(true)
                .takes_value(true)
            )
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("extension") {
        if matches.is_present("key") {
            let key = matches.value_of("key").unwrap();
            let title = matches.value_of("title").unwrap();
            let description = matches.value_of("description").unwrap();
            let conf = Conf::new()
                .key(&key)
                .title(&title)
                .description(&description);
            let extension = Extension::new(conf).build();
            println!("Added new extension {:?} ", key);
        }
    }

    if let Some(matches) = matches.subcommand_matches("element") {
        if matches.is_present("key") {
            let key = matches.value_of("key").unwrap();
            let title = matches.value_of("title").unwrap();
            let description = matches.value_of("description").unwrap();
            let conf = Conf::new()
                .key(&key)
                .title(&title)
                .description(&description);
            let extension = Extension::new(conf).build();
            println!("Added new element {:?} ", key);
        }
    }

    /*

    let element_text = Element::new(Key::new(&extension_key, "text"))
        .icon("text");

    let element_image = Element::new(Key::new(&extension_key, "image"))
        .icon("image");

    let extension = Extension::new(conf)
        .add_element(element_text)
        .add_element(element_image);

    extension.build();

    */
}
