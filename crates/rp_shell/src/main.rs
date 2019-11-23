use futures::executor::block_on;

fn main() -> anyhow::Result<()> {
    block_on(rp_shell::run())
}
