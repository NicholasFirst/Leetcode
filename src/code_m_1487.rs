struct Solution;

impl Solution {

    // https://leetcode.cn/problems/making-file-names-unique/
    #[allow(dead_code)]
    pub fn get_folder_names(names: Vec<String>) -> Vec<String> {
        use std::collections::HashMap;
        let mut map: HashMap<String, i32> = HashMap::new();
        names.into_iter().map(|name| {
            let mut s = name.clone();
            while map.contains_key(&s) {
                s = format!("{}({})", name, map[&name]);
                *map.get_mut(&name).unwrap() += 1;
            }
            map.insert(s.clone(), 1);
            s
        }).collect()
    }
}


#[cfg(test)]
mod test{
    use super::Solution;

    #[test]
    fn test() {
        let names = vec!["pes".to_string(),"fifa".to_string(),"gta".to_string(),"pes(2019)".to_string()];
        let res = Solution::get_folder_names(names);
        assert_eq!(vec!["pes".to_string(),"fifa".to_string(),"gta".to_string(),"pes(2019)".to_string()], res);
    }


    #[test]
    fn test1() {
        let names = vec!["gta".to_string(),"gta(1)".to_string(),"gta".to_string(),"avalon".to_string()];
        let res = Solution::get_folder_names(names);
        assert_eq!(vec!["gta".to_string(),"gta(1)".to_string(),"gta(2)".to_string(),"avalon".to_string()], res);
    }

    #[test]
    fn test2() {
        let names = vec!["kaido".to_string(),"kaido(1)".to_string(),"kaido".to_string(),"kaido(1)".to_string()];
        let res = Solution::get_folder_names(names);
        assert_eq!(vec!["kaido".to_string(),"kaido(1)".to_string(),"kaido(2)".to_string(),"kaido(1)(1)".to_string()], res);
    }
}