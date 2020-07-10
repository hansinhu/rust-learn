use std::collections::HashMap;

fn main() {
    // 1.所有键都必须具有相同的类型，并且所有值都必须具有相同的类型
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // 2.使用迭代器和collect 元组向量上的方法 创建 HashMap
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    // 对于键和值类型的参数，我们使用下划线，Rust可以根据向量中数据的类型推断哈希映射包含的类型
    let scores2: HashMap<_, _> = teams
        .into_iter()
        .zip(initial_scores.into_iter())
        .collect();
    
    // 3.所有权：对于实现Copy特征的类型，例如i32，将值复制到哈希图中。对于拥有的值，如String，这些值将被移动，并且哈希映射将成为这些值的所有者
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // 在下面尝试访问field_name，不存在

    // 4.取值 get
    let t_name = String::from("Blue");
    let  t_score = scores2.get(&t_name);
    println!("Bule is score is {:?}", t_score);

    // 5.遍历
    for (key, val) in &scores2 {
      println!("{}: {}", key, val);
    }

    // 6.更新值：覆盖（新值优先）
    scores.insert(String::from("Blue"), 33);
    println!("{:?}", scores);

    // 7.更新值：不存在时才插入（旧值优先）
    scores.entry(String::from("Blue")).or_insert(88);

    println!("{:?}", scores);

    // 计算单词出现的次数
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for world in text.split_whitespace() {
      let count = map.entry(world).or_insert(0);
      *count += 1;
    }

    println!("{:?}", map);
}
