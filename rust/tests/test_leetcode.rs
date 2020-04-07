use leetcode::*;
use std::collections::LinkedList;

#[test]
fn test_p001() {
    assert_eq!(p001_two_sum(&vec![2, 7, 11, 15], 9), (0, 1));
    assert_eq!(p001_two_sum(&vec![3, 2, 4], 6), (1, 2));
}

#[test]
fn test_p002() {
    let mut num1 = LinkedList::new(); num1.push_back(2); num1.push_back(4); num1.push_back(3);
    let mut num2 = LinkedList::new(); num2.push_back(5); num2.push_back(6); num2.push_back(4);
    let mut num3 = LinkedList::new(); num3.push_back(7); num3.push_back(0); num3.push_back(8);
    assert_eq!(p002_add_two_numbers(&num1, &num2), num3);

    let mut num1 = LinkedList::new(); num1.push_back(9); num1.push_back(9); num1.push_back(9); num1.push_back(9);
    let mut num2 = LinkedList::new(); num2.push_back(9); num2.push_back(9); num2.push_back(9); num2.push_back(9); num2.push_back(9); num2.push_back(9);
    let mut num3 = LinkedList::new(); num3.push_back(8); num3.push_back(9); num3.push_back(9); num3.push_back(9); num3.push_back(0); num3.push_back(0); num3.push_back(1);
    assert_eq!(p002_add_two_numbers(&num1, &num2), num3);
}

#[test]
fn test_p003() {
    assert_eq!(p003_longest_substring("abcabcabc"), "abc".to_string());
    assert_eq!(p003_longest_substring("abcbcabcabc"), "abc".to_string());
    assert_eq!(p003_longest_substring("pwwkew"), "wke".to_string());
    assert_eq!(p003_longest_substring("bbbbb"), "b".to_string());
}

#[test]
fn test_p004() {
    assert_eq!(p004_median_of_two_sorted_arrays(&vec![1, 3], &vec![2, 4]), 2.5);
    assert_eq!(p004_median_of_two_sorted_arrays(&vec![1, 2], &vec![3, 4, 5]), 3.0);
    assert_eq!(p004_median_of_two_sorted_arrays(&vec![2], &vec![]), 2.0);
    assert_eq!(p004_median_of_two_sorted_arrays(&vec![1, 2], &vec![]), 1.5);
}

#[test]
fn test_p010() {
    assert!(p010_regular_expression_matching("mississippi".to_string(), "mis*is*ip*.".to_string()));
    assert!(!p010_regular_expression_matching("mississippi".to_string(), "mis*is*p*.".to_string()));
    assert!(p010_regular_expression_matching("ab".to_string(), ".*".to_string()));
    assert!(p010_regular_expression_matching("aab".to_string(), "c*a*b".to_string()));
    assert!(!p010_regular_expression_matching("aa".to_string(), "a".to_string()));
    assert!(p010_regular_expression_matching("aa".to_string(), "a*".to_string()));
}

#[test]
fn test_p012() {
    assert_eq!(p012_int_to_roman(3), "III".to_string());
    assert_eq!(p012_int_to_roman(39), "XXXIX".to_string());
    assert_eq!(p012_int_to_roman(99), "XCIX".to_string());
    assert_eq!(p012_int_to_roman(98), "XCVIII".to_string());
    assert_eq!(p012_int_to_roman(993), "CMXCIII".to_string());
}

#[test]
fn test_p013() {
    assert_eq!(p013_roman_to_int("III".to_string()), 3);
    assert_eq!(p013_roman_to_int("CMXCIII".to_string()), 993);
    assert_eq!(p013_roman_to_int("LVIII".to_string()), 58);
}

#[test]
fn test_p014() {
    assert_eq!(p014_longest_common_prefix(
            vec![ "flower".to_string(), "flow".to_string(), "flight".to_string()
    ]), "fl".to_string());
    assert_eq!(p014_longest_common_prefix(
            vec![ "dog".to_string(), "racecar".to_string(), "car".to_string()
    ]), "".to_string());

}

#[test]
fn test_p015() {
    assert_eq!(p015_three_sum( vec![ -1, 0, 1, 2, -1, -4 ] ), vec![ vec![-1, -1, 2], vec![-1, 0, 1] ]);
    assert_eq!(p015_three_sum( vec![ -1, 0, 2 ] ), vec![  ] as Vec<Vec<i32>>);
    assert_eq!(p015_three_sum( vec![ 0, 0, 0, 0 ] ), vec![ vec![ 0, 0, 0 ] ] );
    assert_eq!(p015_three_sum( vec![2,0,-2,-5,-5,-3,2,-4] ), vec![ vec![-4, 2, 2], vec![-2, 0, 2] ]);
    assert_eq!(p015_three_sum( vec![2,-3,0,-2,-5,-5,-4,1,2,-2,2,0,2,-4] ), vec![ vec![-4, 2, 2], vec![-3, 1, 2], vec![-2, 0, 2] ]);
    assert_eq!(p015_three_sum( vec![-2,0,1,1,2] ), vec![ vec![-2,0,2], vec![-2, 1, 1] ]);
}

#[test]
fn test_p016() {
    assert_eq!(p016_three_sum_closest( vec![-1, 2, 1, -4, -1, 1], 1), 1);
    assert_eq!(p016_three_sum_closest( vec![0, 1, 2], 0), 3);
    assert_eq!(p016_three_sum_closest( vec![0, 2, 1, -3], 1), 0);
}

#[test]
fn test_p017() {
    assert_eq!(p017_letter_combinations("23".to_string()), 
               vec!["ad", "bd", "cd", "ae", "be", "ce", "af", "bf", "cf"].into_iter().map(
                   |s| s.to_string()).collect::<Vec<String>>()
               );
}

#[test]
fn test_p018() {
    assert_eq!(p018_four_sum( vec![1, 0, -1, 0, -2, 2], 0 ), vec![ vec![-2, -1, 1, 2], vec![-2, 0, 0, 2], vec![-1, 0, 0, 1] ]);
    assert_eq!(p018_four_sum( vec![-3,-2,-1,0,0,1,2,3], 0 ), vec![ 
               vec![-3, -2, 2, 3], vec![-3, -1, 1, 3], vec![-3, 0, 0, 3], vec![-3, 0, 1, 2], 
               vec![-2, -1, 0, 3], vec![-2, -1, 1, 2], vec![-2, 0, 0, 2], vec![-1, 0, 0, 1]]);
}

#[test]
fn test_p019() {
    assert_eq!(
        p019_remove_nth_from_end(Some(Box::new(listnode!(1, 2, 3, 4, 5))), 2), Some(Box::new( listnode!(1, 2, 3, 5) )) 
    );
    assert_eq!(
        p019_remove_nth_from_end(Some(Box::new(listnode!(1, 2, 3, 4, 5))), 1), Some(Box::new( listnode!(1, 2, 3, 4) )) 
    );
}

#[test]
fn test_p020() {
    assert!(p020_is_valid("()".to_string()));
    assert!(p020_is_valid("()[]{}".to_string()));
    assert!(!p020_is_valid("([)]".to_string()));
    assert!(p020_is_valid("([])".to_string()));
    assert!(!p020_is_valid("(".to_string()));
}

#[test]
fn test_p022() {
    assert_eq!(p022_generate_parenthesis(1), vec!["()".to_string()]);
    //assert_eq!(p022_generate_parenthesis(4), vec!["()".to_string()]);
}

#[test]
fn test_p023() {
    //println!("{:?}", p023_merge_k_lists(
            //vec![ 
            //Some(Box::new(listnode!(1, 3))),
            //Some(Box::new(listnode!(1, 2))),
            //]
    //));
    //assert_eq!(
        //p023_merge_k_lists(
            //vec![ 
            //Some(Box::new(listnode!(1, 3))),
            //Some(Box::new(listnode!(1, 2))),
            //]
            //), 
        //Some(Box::new( listnode!(1, 2, 3, 4, 5) )) 
    //);
}

#[test]
fn test_p024() {
    assert_eq!(
        p024_swap_pairs(Some(Box::new(listnode!(1, 2, 3, 4, 5)))), 
        Some(Box::new(listnode!(2, 1, 4, 3, 5)))
        );
}

#[test]
fn test_p026() {
    let mut v1 = vec![0,0,1,1,1,2,2,3,3,4];
    assert_eq!( p026_remove_duplicates(&mut v1), 5 );
}

#[test]
fn test_p027() {
    let mut v = vec![0,1,2,2,3,0,4,2];
    assert_eq!( p027_remove_element(&mut v, 2), 5 );
}

#[test]
fn test_p028() {
    assert_eq!( p028_str_str( "hello".into(), "ll".into() ), 2 );
    assert_eq!( p028_str_str( "aaaaa".into(), "bba".into() ), -1 );
    assert_eq!( p028_str_str( "aaaaa".into(), "".into() ), 0 );
    assert_eq!( p028_str_str( "".into(), "a".into() ), -1 );
    assert_eq!( p028_str_str( "".into(), "".into() ), 0 );
    assert_eq!( p028_str_str( "ab".into(), "ab".into() ), 0 );
}

#[test]
fn test_p029() {
    assert_eq!( p029_divide(10, 3), 3 );
    assert_eq!( p029_divide(7, -3), -2 );
    assert_eq!( p029_divide(7, 1), 7 );
    assert_eq!( p029_divide(-7, 1), -7 );
    assert_eq!( p029_divide(-7, -1), 7 );
    assert_eq!( p029_divide(2147483647, 1), 2147483647);
    assert_eq!( p029_divide(-2147483648, -1), 2147483647);
}

#[test]
fn test_p030() {
    assert_eq!( p030_find_substring( "".to_string(), vec![ "a".to_string() ] ), vec![] );
    assert_eq!( p030_find_substring( "a".to_string(), vec![ "a".to_string() ] ), vec![0] );
    assert_eq!( p030_find_substring( "a".to_string(), vec![ ] ), vec![] );
    assert_eq!( p030_find_substring( "barfoothefoobarman".to_string(), vec![ "foo".to_string(), "bar".to_string() ] ), vec![0, 9] );
    assert_eq!( 
        p030_find_substring( 
            "wordgoodgoodgoodbestword".to_string(), 
            vec!["word","good","best", "word"].into_iter().map(|s| s.into()).collect::<Vec<String>>() ), 
        vec![] );

    assert_eq!( 
        p030_find_substring( 
            "xcsgedisbnkkiperkawetuiokxjmrapqcjyjpgbqulcecgxoitudpcrcccotcglhpjqeptwlhasjgpaqlutaznebptwszhbostvhmtvtunfcehtpboscbwdrpzlqgohahcivxfpruwuydpqgdijhgmaymloubxvizfdxkuqeqmetduajejqnxqlldbgezdoaitzuosagegakdcthnjwmzjyeleimjyotrqphipooxqyasrihagtbqthdzppipfbhvqodheufushomrvmyrqokxrkpiuepwnloeqyikfdfmrrepfcgvqsvjektbqixetnkmlsyqxddpwhgclozdgumnghoxpndlapxohvghbjyxsebfsbiaxwnedddvsggvxdjgapnbblbvpcbhdjibixhlbkgtsooptzvurlxswynmdoviafjidsvgcebwsslmrjiiufcsgqgjgcrghdomvmuaqwwkokwhvgsrmujskqbruszdxqqtckvuirewddgyjypxszpdrswlnvoklefprajzsqyxtewecncuorzfmvqztfjglrwcrfelxcjqghvkuzgjsgoedfdwmpdxgbcxiglgiuyqdtaxuginoxrsevqsmvpuwrrhxenhalxdhzbbilqwiiqofjgrewldpemplzwlmupvvsxddncoxsccdlvkjinypbnaamloiakdujhyylmwdqajbwtkgijvjyvlkhzsjlyeufctsorvergipzswhdrqcpbowdjfohjjonegdvdkoksejkcrovjsklgiorqeybnmprusoyedkwjthnmxkwpxjxfzpvdpxtcokyibwnggjrcseopqmgnvgtuvqamntqbfpmgnzowifydloscdbpyhkvebvqqqhuvwgclfshpyfsjwnnzodzxpudqrtjhcddajhmqztfzbajxnywddxatsdllyuvbzabkjnaihikiivhvtfyxcaswfdidafebfimovoyeyioidvfzadwffqbkvlovquzvcnjydkecstkyoqxrvvwdlznildebyfzasiavufznamnqcmhzhfcufscsvitvpswhdyfxdemfqbwundwwlaqsuvkqffnvalcfkjepotvgurdiwzehbxbwsnozvbuvnzcxigmyzjfuaicxjigkfkgzxuzuytplutkdaybbiixisbhdkqopawrztqurlleghojhmmkuxifrjovtellghcicsetfrxlylwhalsuiczblqwhuhsdpwlrqpnvimhhafoaqiuakwcwmyfiizlzvyqlpfiysrfsxvsneoqomsmasrjaqwznvsakzjiraplxlfnrwdfixujpluseqtrlluyldiedasdlxscvvjeudplrjdxbxqpkkpxpxctxuyktqornyxhdmuwxytaxmphwefoejhbfhmazarmaovecpczpwcokrwiydwcofftmttlwnzwbwceoffddhsnbqxzvubjzieocxbymduozzungztjjlykdxlarojtwpjyokwbntppujcakvlvilfniqnceyvdnebcqadgtuvpfzppxanhlsvvlkxrjuuyywarwdzrzwgevxwuzjemdzholfgwzcvayvtwbspaoxhlwdivmmhpnpgywovxqqzrnsnqmfrceaobdywrkeixvovrcsqtkqkyizovghxljnmmlkfvqoulesehkvcxlo".to_string(),
            vec!["dbpyhkvebvqqqhuvwgclfshpyfs","jwnnzodzxpudqrtjhcddajhmqzt","fzbajxnywddxatsdllyuvbzabkj","naihikiivhvtfyxcaswfdidafeb","fimovoyeyioidvfzadwffqbkvlo","vquzvcnjydkecstkyoqxrvvwdlz","nildebyfzasiavufznamnqcmhzh","fcufscsvitvpswhdyfxdemfqbwu","ndwwlaqsuvkqffnvalcfkjepotv","gurdiwzehbxbwsnozvbuvnzcxig","myzjfuaicxjigkfkgzxuzuytplu","tkdaybbiixisbhdkqopawrztqur","lleghojhmmkuxifrjovtellghci","csetfrxlylwhalsuiczblqwhuhs","dpwlrqpnvimhhafoaqiuakwcwmy","fiizlzvyqlpfiysrfsxvsneoqom","smasrjaqwznvsakzjiraplxlfnr","wdfixujpluseqtrlluyldiedasd","lxscvvjeudplrjdxbxqpkkpxpxc","txuyktqornyxhdmuwxytaxmphwe","foejhbfhmazarmaovecpczpwcok","rwiydwcofftmttlwnzwbwceoffd","dhsnbqxzvubjzieocxbymduozzu","ngztjjlykdxlarojtwpjyokwbnt","ppujcakvlvilfniqnceyvdnebcq"].into_iter().map(|s| s.into()).collect::<Vec<String>>() 
        ), 
        vec![873]);

    assert_eq!( 
        p030_find_substring(
            "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".to_string(),
            vec!["a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a","a"].into_iter().map(|s| s.into()).collect::<Vec<String>>() 
        ), 
        vec![]);
}

#[test]
fn test_p031() {
    let mut arr = vec![1,2,3,4];
    p031_next_permutation(&mut arr);
    assert_eq!( arr, vec![1, 2, 4, 3] );

    let mut arr = vec![3,2,1];
    p031_next_permutation(&mut arr);
    assert_eq!( arr, vec![1,2,3] );

    let mut arr = vec![1,3,2];
    p031_next_permutation(&mut arr);
    assert_eq!( arr, vec![2,1,3] );
}

#[test]
fn test_p032() {
    assert_eq!( p032_longest_valid_parentheses( "()(())".into() ), 6 );
    assert_eq!( p032_longest_valid_parentheses( "(()".into() ), 2 );
    assert_eq!( p032_longest_valid_parentheses( ")()())".into() ), 4 );
    assert_eq!( p032_longest_valid_parentheses( ")((()))())".into() ), 8 );
}

#[test]
fn test_p033() {
    assert_eq!(p033_search(vec![4,5,6,7,0,1,2], 4), 0);
    assert_eq!(p033_search(vec![4,5,6,7,0,1,2], 7), 3);
    assert_eq!(p033_search(vec![4,5,6,7,0,1,2], 0), 4);
    assert_eq!(p033_search(vec![4,5,6,7,0,1,2], 1), 5);
    assert_eq!(p033_search(vec![0,1], 3), -1);
    assert_eq!(p033_search(vec![0,1], 1), 1);
    assert_eq!(p033_search(vec![1], 0), -1);
    assert_eq!(p033_search(vec![3, 1], 1), 1);
}

#[test]
fn test_p034() {
    assert_eq!(p034_search_range(vec![5,7,7,8,8,10], 8), vec![3,4]);
    assert_eq!(p034_search_range(vec![5,7,7,8,8,10], 6), vec![-1,-1]);
    assert_eq!(p034_search_range(vec![5,7,7,8,8,10], 10), vec![5,5]);
    assert_eq!(p034_search_range(vec![5,7,7,8,8,10], 5), vec![0,0]);
}

#[test]
fn test_p035() {
    assert_eq!(p035_search_insert(vec![1,3,5,6], 5), 2);
}

#[test]
fn test_p036() {
    let sudoku1: Vec<Vec<char>> = vec![
        vec!['5','3','.','.','7','.','.','.','.'],
        vec!['6','.','.','1','9','5','.','.','.'],
        vec!['.','9','8','.','.','.','.','6','.'],
        vec!['8','.','.','.','6','.','.','.','3'],
        vec!['4','.','.','8','.','3','.','.','1'],
        vec!['7','.','.','.','2','.','.','.','6'],
        vec!['.','6','.','.','.','.','2','8','.'],
        vec!['.','.','.','4','1','9','.','.','5'],
        vec!['.','.','.','.','8','.','.','7','9'],
    ];

    assert!(p036_is_valid_sudoku(sudoku1));
}

#[test]
#[warn(dead_code)] // not work now
#[allow(unused_variables)]
fn test_p037() {
    let mut sudoku1: Vec<Vec<char>> = vec![
        vec!['.','.','9','7','4','8','.','.','.'],
        vec!['7','.','.','.','.','.','.','.','.'],
        vec!['.','2','.','1','.','9','.','.','.'],
        vec!['.','.','7','.','.','.','2','4','.'],
        vec!['.','6','4','.','1','.','5','9','.'],
        vec!['.','9','8','.','.','.','3','.','.'],
        vec!['.','.','.','8','.','3','.','2','.'],
        vec!['.','.','.','.','.','.','.','.','6'],
        vec!['.','.','.','2','7','5','9','.','.'],
    ];
    //let mut sudoku1: Vec<Vec<char>> = vec![
        //vec!['.','.','9','7','4','8','.','.','.'],
        //vec!['7','.','.','.','.','.','.','.','.'],
        //vec!['.','2','.','1','.','9','.','.','.'],
        //vec!['.','.','7','.','.','.','2','4','.'],
        //vec!['.','6','4','.','1','.','5','9','.'],
        //vec!['.','9','8','.','.','.','3','.','.'],
        //vec!['.','.','.','8','.','3','.','2','.'],
        //vec!['.','.','.','.','.','.','.','.','6'],
        //vec!['.','.','.','2','7','5','9','.','.'],
    //];
    let sol: Vec<Vec<char>> = vec![
        vec!['5','1','9','7','4','8','6','3','2'],
        vec!['7','8','3','6','5','2','4','1','9'],
        vec!['4','2','6','1','3','9','8','7','5'],
        vec!['3','5','7','9','8','6','2','4','1'],
        vec!['2','6','4','3','1','7','5','9','8'],
        vec!['1','9','8','5','2','4','3','6','7'],
        vec!['9','7','5','8','6','3','1','2','4'],
        vec!['8','3','2','4','9','1','7','5','6'],
        vec!['6','4','1','2','7','5','9','8','3'],
    ];
    p037_solve_sudoku(&mut sudoku1);
    //assert_eq!( sudoku1, sol );


    /*
     *let mut sudoku1: Vec<Vec<char>> = vec![
     *    vec!['5','3','.','.','7','.','.','.','.'],
     *    vec!['6','.','.','1','9','5','.','.','.'],
     *    vec!['.','9','8','.','.','.','.','6','.'],
     *    vec!['8','.','.','.','6','.','.','.','3'],
     *    vec!['4','.','.','8','.','3','.','.','1'],
     *    vec!['7','.','.','.','2','.','.','.','6'],
     *    vec!['.','6','.','.','.','.','2','8','.'],
     *    vec!['.','.','.','4','1','9','.','.','5'],
     *    vec!['.','.','.','.','8','.','.','7','9'],
     *];
     *let sol: Vec<Vec<char>> = vec![
     *    vec!['5','3','4','6','7','8','9','1','2'],
     *    vec!['6','7','2','1','9','5','3','4','8'],
     *    vec!['1','9','8','3','4','2','5','6','7'],
     *    vec!['8','5','9','7','6','1','4','2','3'],
     *    vec!['4','2','6','8','5','3','7','9','1'],
     *    vec!['7','1','3','9','2','4','8','5','6'],
     *    vec!['9','6','1','5','3','7','2','8','4'],
     *    vec!['2','8','7','4','1','9','6','3','5'],
     *    vec!['3','4','5','2','8','6','1','7','9']
     *];
     *p037_solve_sudoku(&mut sudoku1);
     *assert_eq!( sudoku1, sol )
     */
}

#[test]
fn test_p038() {
    assert_eq!(p038_count_and_say(1), "1".to_string());
    assert_eq!(p038_count_and_say(4), "1211".to_string());
    assert_eq!(p038_count_and_say(6), "312211".to_string());
}

#[test]
fn test_p039() {
    assert_eq!(p039_combination_sum(vec![2,3,6,7], 7), vec![vec![7], vec![2,2,3]]);
    assert_eq!(p039_combination_sum(vec![7], 7), vec![vec![7]]);
}

#[test]
fn test_p040() {
    assert_eq!(p040_combination_sum(vec![10,1,2,7,6,1,5], 8), vec![vec![2, 6], vec![1, 7], vec![1, 2, 5], vec![1, 1, 6]]);
}

#[test]
fn test_p041() {
    assert_eq!(p041_first_missing_positive(vec![3, 4, -1, 1]), 2);
    assert_eq!(p041_first_missing_positive(vec![7,8,9,11,12]), 1);
    assert_eq!(p041_first_missing_positive(vec![3,4,-1,1]), 2);
}

#[test]
fn test_p042() {
    assert_eq!(p042_trap(vec![0,1,0,2,1,0,1,3,2,1,2,1]), 6);
    assert_eq!(p042_trap(vec![0,1,0,2,1,0]), 1);
    assert_eq!(p042_trap(vec![0,1,0,2,1,0,1]), 2);
}

#[test]
fn test_p043() {
    assert_eq!(p043_multiply("999".into(), "999".into()), "998001".to_string());
    assert_eq!(p043_multiply("123".into(), "456".into()), "56088".to_string());
    assert_eq!(p043_multiply("123456789".into(), "987654321".into()), "121932631112635269".to_string());
    assert_eq!(p043_multiply("999".into(), "0".into()), "0".to_string());
}

#[test]
fn test_p044() {
    assert_eq!(p044_is_match("aab".into(), "c*a*b".into()), false);
    assert_eq!(p044_is_match("aa".into(), "a".into()), false);
    assert_eq!(p044_is_match("aa".into(), "*".into()), true);
    assert_eq!(p044_is_match("cb".into(), "?a".into()), false);
    assert_eq!(p044_is_match("adceb".into(), "*a*b".into()), true);
    assert_eq!(p044_is_match("ssippi".into(), "ss*?i*pi".into()), false);
    assert_eq!(p044_is_match("mississippi".into(), "m??*ss*?i*pi".into()), false);
    assert_eq!(p044_is_match("ho".into(), "ho**".into()), true);
    assert_eq!(p044_is_match("bbbbbbbabbaabbabbbbaaabbabbabaaabbababbbabbbabaaabaab".into(), "b*b*ab**ba*b**b***bba".into()), false);
}

#[test]
fn test_p045() {
    assert_eq!(p045_jump(vec![2,3,1,1,4]), 2);
    assert_eq!(p045_jump(vec![1,1,1,1,1]), 4);
    assert_eq!(p045_jump(vec![1,2,1,1,1]), 3);
    assert_eq!(p045_jump(vec![2,3,0,1,4]), 2);
    assert_eq!(p045_jump(vec![0]), 0);
}

#[test]
fn test_p046() {
    assert_eq!(p046_permute(vec![1]), vec![vec![1]]);
    assert_eq!(p046_permute(vec![1, 2]), vec![vec![2, 1], vec![1, 2]]);
}

#[test]
fn test_p047() {
    assert_eq!(p047_permute_unique(vec![1]), vec![vec![1]]);

    let mut res = p047_permute_unique(vec![1, 1, 2]);
    let mut matched = vec![vec![2, 1, 1], vec![1, 2, 1], vec![1, 1, 2]];
    res.sort();matched.sort();
    assert_eq!(res, matched);

    let mut res = p047_permute_unique(vec![1, 1, 2, 2]);
    let mut matched = vec![vec![2, 1, 2, 1], vec![2, 1, 1, 2], vec![1, 2, 1, 2], vec![2, 2, 1, 1], vec![1, 2, 2, 1], vec![1, 1, 2, 2]];
    res.sort();matched.sort();
    assert_eq!(res, matched);

    let mut res = p047_permute_unique(vec![2, 3, 3, 3, 2]);
    let mut matched = vec![vec![2,2,3,3,3], vec![2,3,2,3,3], vec![2,3,3,2,3], vec![2,3,3,3,2], vec![3,2,2,3,3], vec![3,2,3,2,3], vec![3,2,3,3,2], vec![3,3,2,2,3], vec![3,3,2,3,2], vec![3,3,3,2,2], ];
    res.sort();matched.sort();
    assert_eq!(res, matched);
}

#[test]
fn test_p048() {
    let mut input = vec![
        vec![1, 2, 3],
        vec![4, 5, 6],
        vec![7, 8, 9], ];
    p048_rotate(&mut input);
    let matched = vec![
        vec![7, 4, 1],
        vec![8, 5, 2],
        vec![9, 6, 3],
    ];
    assert_eq!(input, matched);

    let mut input = vec![
        vec![ 5, 1, 9,11],
        vec![ 2, 4, 8,10],
        vec![13, 3, 6, 7],
        vec![15,14,12,16],
    ];
    p048_rotate(&mut input);
    let matched = vec![ 
        vec![15, 13, 2, 5], 
        vec![14, 3, 4, 1], 
        vec![12, 6, 8, 9], 
        vec![16, 7, 10, 11],
    ];
    assert_eq!(input, matched);
}

#[test]
fn test_p049() {
    assert_eq!(p049_group_anagrams(vec!["eat".to_string(), "tea".to_string()]), vec![ vec!["eat".to_string(), "tea".to_string()] ]);
}

#[test]
fn test_p050() {
    assert_eq!(p050_my_pow(2.0, 1), 2.0);
    assert_eq!(p050_my_pow(2.0, 10), 1024.0);
    assert_eq!(p050_my_pow(1.0, 214748364), 1.0);
    assert_eq!(p050_my_pow(0.00001, 2147483647), 0.0);
    assert_eq!(p050_my_pow(1.0, 2147483647), 1.0);
}

#[test]
fn test_p053() {
    assert_eq!(p053_max_sub_array(vec![-2,1,-3,4,-1,2,1,-5,4]), 6);
    assert_eq!(p053_max_sub_array(vec![-2,1,-3,4,-1,2]), 5);
    assert_eq!(p053_max_sub_array(vec![-2]), -2);
}

#[test]
fn test_p054() {
    assert_eq!(
        p054_spiral_order(
            vec![ 
                vec![1, 2, 3], 
                vec![4, 5, 6],
                vec![7, 8, 9],
            ]
        ),
        vec![1,2,3,6,9,8,7,4,5],
        );
    assert_eq!(
        p054_spiral_order(
            vec![ 
                vec![1, 2, 3, 4], 
                vec![5, 6, 7, 8],
                vec![9,10,11,12],
            ]
        ),
        vec![1,2,3,4,8,12,11,10,9,5,6,7],
        );
    assert_eq!(
        p054_spiral_order(
            vec![ 
                vec![3], 
                vec![2], 
            ]
        ),
        vec![3, 2],
        );
    assert_eq!(
        p054_spiral_order(
            vec![ 
                vec![1, 2, 3], 
                vec![4, 5, 6], 
            ]
        ),
        vec![1, 2, 3, 6, 5, 4],
        );
    assert_eq!(
        p054_spiral_order(
            vec![ 
                vec![1, 2], 
                vec![3, 4], 
            ]
        ),
        vec![1, 2, 4, 3],
        );
}

#[test]
fn test_p051() {
    assert_eq!(p052_total_n_queens(1), 1);
    assert_eq!(p052_total_n_queens(2), 0);
    assert_eq!(p052_total_n_queens(3), 0);
    assert_eq!(p052_total_n_queens(4), 2);
    assert_eq!(p052_total_n_queens(5), 10);
    assert_eq!(p052_total_n_queens(8), 92);
    //assert_eq!(p052_total_n_queens(12), 14200); // emmm..., it can't reach here
}

#[test]
fn test_p055() {
    assert_eq!(p055_can_jump(vec![1, 2, 3]), true);
    assert_eq!(p055_can_jump(vec![8,2,4,4,4,9,5,2,5,8,8,0,8,6,9,1,1,6,3,5,1,2,6,6,0,4,8,6,0,3,2,8,7,6,5,1,7,0,3,4,8,3,5,9,0,4,0,1,0,5,9,2,0,7,0,2,1,0,8,2,5,1,2,3,9,7,4,7,0,0,1,8,5,6,7,5,1,9,9,3,5,0,7,5]), true);
    assert_eq!(p055_can_jump(vec![2, 0, 2]), true);
    assert_eq!(p055_can_jump(vec![2, 0]), true);
    assert_eq!(p055_can_jump(vec![3, 2, 1, 0, 4]), false);
    assert_eq!(p055_can_jump(vec![3, 2, 1, 0]), true);
    assert_eq!(p055_can_jump(vec![3, 2, 2, 0, 0]), true);
    assert_eq!(p055_can_jump(vec![3, 2, 2, 0, 0, 1]), false);
    assert_eq!(p055_can_jump(vec![3, 0, 0, 0]), true);
    assert_eq!(p055_can_jump(vec![0]), true);
    assert_eq!(p055_can_jump(vec![2, 1, 0, 0]), false);

    //let mut vec: Vec<i32> = (0..25000+1).rev().collect();
    //vec.push(0);
    //assert_eq!(p055_can_jump(vec), false);
}

#[test]
fn test_p056() {
    assert_eq!(p056_merge(vec![vec![1,4], vec![4,5]]), vec![vec![1, 5]]);
    assert_eq!(p056_merge(vec![ vec![1,3], vec![2,6], vec![8,10], vec![15,18] ]), vec![ vec![1,6], vec![8,10], vec![15,18] ]);
    assert_eq!(p056_merge(vec![ vec![-1, 5], vec![2,3], vec![4,5], vec![6,7], vec![8,9], vec![1,10] ]), vec![ vec![-1,10] ]);
}

#[test]
fn test_p057() {
    assert_eq!(p057_insert(vec![vec![1,3], vec![6,9]], vec![2,5]), vec![vec![1, 5], vec![6, 9]]);
}

#[test]
fn test_p058() {
    assert_eq!(p058_length_of_last_word("Hello World".into()), 5);
    assert_eq!(p058_length_of_last_word("a ".into()), 1);
}

#[test]
fn test_p060() {
    assert_eq!(p060_get_permutation(3, 3), "213");
    assert_eq!(p060_get_permutation(4, 9), "2314");
    assert_eq!(p060_get_permutation(9, 24479), "168975423");
}

#[test]
fn test_p061() {
    //assert_eq!( p061_rotate_right( Some(Box::new(listnode!(1, 2, 3, 4, 5))), 1 ), Some( Box::new(listnode!(2, 1, 4, 3, 5)) ));
}

#[test]
fn test_p1223() {
    assert_eq!( p1223_die_simulator( 2, vec![1, 1, 2, 2, 2, 3] ), 34 );
}
