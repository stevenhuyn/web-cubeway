use cubeway::{run, setup};
use pollster::FutureExt;

fn main() {
    setup();
    run(10).block_on();
}
