use std::collections::HashMap;

// This annotation prevents Clippy from warning us that `School` has a
// `fn new()` with no arguments, but doesn't implement the `Default` trait.
//
// Normally, it's good practice to just do what Clippy tells you, but in this
// case, we want to keep things relatively simple. The `Default` trait is not the point
// of this exercise.
#[allow(clippy::new_without_default)]
pub struct School {
    grade_hash: HashMap<u32, Vec<String>>,
}
// 面向用例编程
impl School {
    pub fn new() -> School {
        School{
            grade_hash: HashMap::new(),
        }
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        if self.grade_hash.contains_key(&grade) {
            if let Some(vec) = self.grade_hash.get_mut(&grade){
                vec.push(student.to_string());
            } 
        }else{
            let mut v:Vec<String> = vec![];
            v.push(student.to_string());
            self.grade_hash.insert(grade, v);
        }
    }

    pub fn grades(&self) -> Vec<u32> {
        let mut res: Vec<u32> = vec![];
        for p in self.grade_hash.keys(){
            res.push(*p);
        }
        res.sort();
        res
    }

    // If `grade` returned a reference, `School` would be forced to keep a `Vec<String>`
    // internally to lend out. By returning an owned vector of owned `String`s instead,
    // the internal structure can be completely arbitrary. The tradeoff is that some data
    // must be copied each time `grade` is called.
    // 返回某个年级有哪些学生
    pub fn grade(&self, grade: u32) -> Vec<String> {
        if self.grade_hash.contains_key(&grade) {
            if let Some(vec) = self.grade_hash.get(&grade){
                let mut t = vec.clone();
                t.sort();
                return t;
            } 
            
        }
            vec![]
        
    }
}