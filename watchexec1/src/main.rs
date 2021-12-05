use anyhow::anyhow;
use watchexec::{
    config::{Config, ConfigBuilder},
    error::Result,
    pathop::PathOp,
    run::{watch, ExecHandler, Handler},
};

fn main() -> anyhow::Result<()> {
    let config = ConfigBuilder::default()
        // .clear_screen(true)
        // .run_initially(true)
        // .paths(vec!["./README.md".into()])
        .paths(vec![".".into()])
        .cmd(vec!["date; seq 1 10".into()])
        .build()
        .map_err(|_| anyhow!("fail"))?;

    let handler = MyHandler(ExecHandler::new(config)?);
    Ok(watch(&handler).map_err(|_| anyhow!(""))?)
}

struct MyHandler(ExecHandler);

impl Handler for MyHandler {
    fn args(&self) -> Config {
        self.0.args()
    }

    fn on_manual(&self) -> Result<bool> {
        println!("on manual");
        self.0.on_manual()
    }

    fn on_update(&self, ops: &[PathOp]) -> Result<bool> {
        println!("on update {:?}", ops);
        self.0.on_update(ops)
    }
}
