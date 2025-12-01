use boa_engine::{Context, Source};

pub mod url;

const JS_SOURCE: &str = r#"
    const deepDishRust = new URL("hi", "https://deep.dish.rust")
    deepDishRust.host
"#;

fn main() {
    let mut context = Context::default();
    url::Url::register(None, &mut context).unwrap();
    
    // We now have JavaScript context that can use the `URL` class

    // Evaluate our source code.
    let result = context.eval(Source::from_bytes(JS_SOURCE)).unwrap();
    
    println!("{}", result.display());
}
