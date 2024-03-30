
    pub fn result_array(nums: Vec<i32>) -> Vec<i32> {
        let mut a: Vec<i32> = Vec::new();
        let mut b: Vec<i32> = Vec::new();
        let mut i = 0;
        for x in &nums{
            if i == 0{
                a.push(*x);
            }else if i == 1{
                b.push(*x);
            }else if a.last() > b.last(){
                a.push(*x)
            }else{
                b.push(*x);
            }
            i+=1;

        }
        a.extend(b.iter().copied());
        a
    }
    fn main(){
        let mut nums = vec![5,4,3,8];
        println!("{:?}", result_array(nums));
    }
