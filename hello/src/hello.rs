struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    
}

    pub fn move_zeroes(nums: &mut Vec<i32>) {
            let mut i = 0;
            for x in nums.iter(){
                if x != 0{
                    nums[i] = *x;
                    i+=1;
                }
            }
            nums
    }
