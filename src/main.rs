mod args;
mod commands;
mod config;
mod wasm_fetch;
mod cosmos;

use std::collections::HashMap;

use clap::Parser;

use crate::args::{Cli, Command};
use crate::commands::init;
fn main() {
    let args = Cli::parse();
    match args.command {
        Command::Init(options) => init(options),
    }
}


fn accounts() -> HashMap<String, String> {
    HashMap::from([
      ("validator".to_string(),
        "satisfy adjust timber high purchase tuition stool faith fine install that you unaware feed domain license impose boss human eager hat rent enjoy dawn".to_string()),
      ("test1".to_string(),
          "notice oak worry limit wrap speak medal online prefer cluster roof addict wrist behave treat actual wasp year salad speed social layer crew genius".to_string()),
      ("test2".to_string(),
          "quality vacuum heart guard buzz spike sight swarm shove special gym robust assume sudden deposit grid alcohol choice devote leader tilt noodle tide penalty".to_string()),
      ("test3".to_string(),
          "symbol force gallery make bulk round subway violin worry mixture penalty kingdom boring survey tool fringe patrol sausage hard admit remember broken alien absorb".to_string()),
      ("test4".to_string(),
          "bounce success option birth apple portion aunt rural episode solution hockey pencil lend session cause hedgehog slender journey system canvas decorate razor catch empty".to_string()),
      ("test5".to_string(),
          "second render cat sing soup reward cluster island bench diet lumber grocery repeat balcony perfect diesel stumble piano distance caught occur example ozone loyal".to_string()),
      ("test6".to_string(),
          "spatial forest elevator battle also spoon fun skirt flight initial nasty transfer glory palm drama gossip remove fan joke shove label dune debate quick".to_string()),
      ("test7".to_string(),
          "noble width taxi input there patrol clown public spell aunt wish punch moment will misery eight excess arena pen turtle minimum grain vague inmate".to_string()),
      ("test8".to_string(),
          "cream sport mango believe inhale text fish rely elegant below earth april wall rug ritual blossom cherry detail length blind digital proof identify ride".to_string()),
      ("test9".to_string(),
          "index light average senior silent limit usual local involve delay update rack cause inmate wall render magnet common feature laundry exact casual resource hundred".to_string()),
      ("test10".to_string(),
          "prefer forget visit mistake mixture feel eyebrow autumn shop pair address airport diesel street pass vague innocent poem method awful require hurry unhappy shoulder".to_string()),
  ])
}