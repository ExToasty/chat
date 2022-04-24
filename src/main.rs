use chat::initialize;

fn main() {
    let res = initialize();

    if let Err(err) = res {
        eprintln!("{:#?}", err)
    }
}