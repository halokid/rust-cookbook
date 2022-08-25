use std::collections::HashMap;

pub fn comm() {
  let mut my_gems = HashMap::new();

  my_gems.insert("red gem", 1);
  my_gems.insert("blue gem", 2);
  my_gems.insert("stone pick up by the river", 10);

  // ----------------------------------------------------
  let teams_list = vec![
    ("hello".to_string(), 100),
    ("rook".to_string(), 100),
    ("mink".to_string(), 100),
  ];

  let mut teams_map = HashMap::new();
  for team in &teams_list {
    teams_map.insert(&team.0, &team.1);
  }
  println!("tems_map 1 -->>> {:?}", teams_map);

  // -----------------------------------------------------
  let teams_list_cl = teams_list.clone();
  let teams_mapx: HashMap<_, _> = teams_list_cl.into_iter().collect();
  println!("tems_map 2 -->>> {:?}", teams_map);
  for (key, value) in &teams_mapx {
    println!("{}: {}", key, value);
  }

  let team_number = teams_mapx.get("hello");
  match team_number {
    None => {
      println!("can not find team_number of hello");
    }
    Some(team_number) => {
      println!("find team_number of hello {}", team_number);
    }
  }

  // ----------------------------------------------------
  let name = String::from("Sunface");
  let age = 18;

  let mut handsome_boys = HashMap::new();
  handsome_boys.insert(&name, age);

  // std::mem::drop(name);
  println!("因为过于无耻，{:?}已经被除名", handsome_boys);
  println!("还有，他的真实年龄远远不止{}岁", age);

  // ---------------------------------------------------
  let mut scores = HashMap::new();

  scores.insert("Blue", 10);

  // 覆盖已有的值
  let old = scores.insert("Blue", 20);
  assert_eq!(old, Some(10));

  // 查询新插入的值
  let new = scores.get("Blue");
  assert_eq!(new, Some(&20));
  // assert_eq!(new, Some(20));

  // 查询Yellow对应的值，若不存在则插入新值
  let v = scores.entry("Yellow").or_insert(5);
  assert_eq!(*v, 5); // 不存在，插入5

  // 查询Yellow对应的值，若不存在则插入新值
  let v = scores.entry("Yellow").or_insert(50);
  assert_eq!(*v, 5); // 已经存在，因此50没有插入

  println!("scores -->>> {:?}", scores);

  // ----------------------------------------------
  // todo: get quantity of the words, use hashMap
  let text = "hello world wonderful world";

  let mut map = HashMap::new();
// 根据空格来切分字符串(英文单词都是通过空格切分)
  for word in text.split_whitespace() {
    let count = map.entry(word).or_insert(0);
    *count += 1;
  }

  println!("{:?}", map);
}





