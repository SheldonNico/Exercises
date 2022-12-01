pub fn p01() {
    let contents = r#"1000
2000
3000

4000

5000
6000

7000
8000
9000

10000"#;
    let contents = std::fs::read_to_string("./assets/adv2022/adv01.txt").unwrap();

    let calories: Vec<Vec<usize>> = contents.trim().split("\n\n").map(
        |lines| lines.split("\n").map(|cc| cc.parse().unwrap()).collect()
    ).collect();
    let mut calories_max: Vec<usize> = calories.into_iter().map(|cc| cc.into_iter().sum::<usize>()).collect();
    eprintln!("max calories: {:?}", calories_max.iter().max());

    calories_max.sort();
    eprintln!("top 3 calories: {:?}", calories_max.iter().rev().take(3).sum::<usize>());
}
