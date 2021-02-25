/*
状态模式
 */


use std::collections::HashMap;

// todo: 1. 定义状态类型
#[derive(Debug, Eq, PartialEq, Hash)]
enum StateDice {      // 状态筛子
  PowerOn,
  StopDice,
  PowerOff,
}

// todo: 2. 定义状态接口
trait State {
  fn on_press_button(&self, _: &mut StateContext);
}

// todo: 3. 实现具体的状态实体类
struct StatePowerOn;
impl State for StatePowerOn {
  fn on_press_button(&self, context: &mut StateContext)
  {
    // Something to do for turning on the dice.
    println!("Power on and Shake the dice.");

    context.set_state(StateDice::StopDice);
  }
}

struct StateStop;
impl State for StateStop {
  fn on_press_button(&self, context: &mut StateContext)
  {
    // Something to do for turning on the dice.
    println!("Stopping the dice.");

    context.set_dice_number(4);

    context.set_state(StateDice::PowerOff);
  }
}

struct StatePowerOff;
impl State for StatePowerOff {
  fn on_press_button(&self, context: &mut StateContext)
  {
    // Something to do for turning on the dice.
    println!("Power off.");

    context.set_state(StateDice::PowerOn);
  }
}


// todo: 4. 定义具体的状态管理类
#[derive(Debug)]
struct StateContext {
  number: Option<u8>,
  current_state: StateDice,
}

impl StateContext {
  fn new() -> StateContext
  {
    StateContext {
      number: None,
      current_state: StateDice::PowerOn,
    }
  }

  fn set_state(&mut self, s: StateDice)
  {
    self.current_state = s;
  }

  fn set_dice_number(&mut self, n :u8)
  {
    self.number = Some(n)
  }

  fn press_button<'a>(&mut self, hmap: &HashMap<StateDice, Box<dyn State + 'a>>)
  {
    let b = hmap.get(&self.current_state).unwrap();
    b.on_press_button(self);
  }
}

fn main() {
  let mut hmap = HashMap::new();
  hmap.insert(StateDice::PowerOn,  Box::new(StatePowerOn)    as Box<dyn State>);
  hmap.insert(StateDice::StopDice, Box::new(StateStop)       as Box<dyn State>);
  hmap.insert(StateDice::PowerOff, Box::new(StatePowerOff)   as Box<dyn State>);
  let hmap = &hmap;

  let mut context = StateContext::new();
  context.press_button(hmap);
  println!("{:?}", context);
  context.press_button(hmap);
  println!("{:?}", context);
  context.press_button(hmap);
  println!("{:?}", context);
}





















