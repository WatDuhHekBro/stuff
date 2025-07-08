mod template_builder;

use reqwest::Client;
use serde::{Deserialize, Serialize};
use template_builder::TEMPLATES;
use tera::Context;

#[derive(Serialize, Deserialize, Debug)]
struct GitHubUser {
    login: String,
    id: i32,
}

fn main() {
    // Tera bundled example
    let mut context = Context::new();
    context.insert("var", "asdf");
    let a = TEMPLATES.render("sub/asdf.html", &context);
    println!("{a:?}\n");

    // Server-Side Request (should be put in an async handler)
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            // A user agent is required to make any requests to GitHub.
            let a = Client::builder().user_agent("ZBound Auth").build().unwrap();
            let b = a
                .get("https://api.github.com/users/watduhhekbro")
                .send()
                .await
                .unwrap()
                .json::<GitHubUser>()
                .await
                .unwrap();
            println!("{b:?}");
        });
}
