use block::Block;

mod app;
mod block;
mod util;

fn main() {
    let mut app = app::App::new();
    app.genesis();

    add_block("test number 1", &mut app);
    add_block("test number 2", &mut app);

    println!("{:#?}", app.blocks);
}

fn add_block(data: &str, app: &mut app::App) {
    if let Some(latest) = app.blocks.last() {
        let id = latest.id + 1;
        let hash = latest.hash.clone();

        let new_block = Block::new(id, hash, data.to_owned());

        app.try_add_block(new_block)
    }
}
