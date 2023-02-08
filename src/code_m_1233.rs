struct Solution;

impl Solution {

    // https://leetcode.cn/problems/remove-sub-folders-from-the-filesystem/
    #[allow(dead_code)]
    pub fn remove_subfolders(folder: Vec<String>) -> Vec<String> {
        let mut folder = folder;
        folder.sort_unstable_by(|a,b| a.len().cmp(&b.len()));
    
        use std::collections::HashSet;

        let mut set = HashSet::new();

        for path in folder {
            let byt = path.as_bytes();
            let mut flg = false;
            for f in 0..path.len() {
                if byt[f] == b'/' && set.contains(path.get(0..f).unwrap()) {
                    flg = true;
                    break;
                }
            }
            if !flg {
                set.insert(path);
            }    
        }
        set.into_iter().collect()
    }
}

#[cfg(test)]
mod test{
    use super::Solution;

    #[test]
    fn test() {
        let folder = vec!["/a".to_string(),"/a/b".to_string(),"/c/d".to_string(),"/c/d/e".to_string(),"/c/f".to_string()];
        let res = Solution::remove_subfolders(folder);
        assert!(res.contains(&"/a".to_string()));
        assert!(res.contains(&"/c/d".to_string()));
        assert!(res.contains(&"/c/f".to_string()));
    }

    #[test]
    fn test1() {
        let mut folder = vec!["/z/b/c".to_string(), "/a/z".to_string(), "/a/c/d".to_string(), "/a/c/d".to_string()];
        folder.sort();
        dbg!(folder);
    }
}