use std::collections::HashMap;
pub fn number_of_pairs(nums: Vec<i32>) -> Vec<i32> {
    let mut mp: HashMap<i32,i32> = HashMap::new();
    for x in nums{
        mp.entry(x).and_modify(|x| *x+=1).or_insert(1);
    }
    let mut cnt = 0;
    let mut cnt2 = 0;
    for x in mp.values(){
        if x&1 == 1{
            cnt/=2;
            cnt2+=1;
        }
    }
    return vec![cnt, cnt2];
}

    pub fn minimum_operations(nums: Vec<i32>) -> i32 {
        // 创建数组let mut arr  = [0;101];
        let mut mp :HashMap<i32, i32> = HashMap::new();
        for x in nums{
            mp.entry(x).or_insert(1);
        }
        return mp.len() as i32;
    }

    pub fn count_asterisks(s: String) -> i32 {
        let mut i = 0;
        let mut cnt = 0;
        for c in s.bytes(){
            if i & 1 == 0{
                if c == b'*'{
                   cnt+=1; 
                }else if c == b'|'{
                    i+=1;
                }
            }else{
                if c == b'*'{
                    i+=1;
                }
            }
        }
        cnt
    }


    pub fn check_x_matrix(grid: Vec<Vec<i32>>) -> bool {
        let row = grid.len();
        let col = grid[0].len();
        for i in 0..row{
            for j in 0..col{
                if i == j || i + j == col-1{
                    if grid[i][j] == 0{
                        println!("i = {}, j = {}", i, j);
                        return false;
                    }
                }else{
                    if grid[i][j] != 0{
                        println!("i = {}, j = {}", i, j);
                        return false;
                    }
                } 
            }
        }
        return true;
    }