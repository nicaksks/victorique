use victorique::logger::terminal::{Constructor, Logger};

fn main() {
    Logger.fatal("Hello, World!");
    Logger.error("Hello, World!");
    Logger.warn("Hello, World!");
    Logger.debug("Hello, World!");
    Logger.info("Hello, World!");
}