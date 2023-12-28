// ProblemRemoveDuplicateItems.rs
// by Zqiu

// problem struct definition
// Example: Given sorted array [1, 1, 2, 3, 3], we need to remove dupliate 1 and 3
//          update it to [1, 2, 3, _, _], valid data length is 3

pub struct ProblemRemoveDuplicateItems {
    v: Vec<i32>, // sorted array as raw data of the problem
    name:String, // name of the problem describing the algorithm purpose
}
    fn show_array(v: &Vec<i32>){
        let vecter_iterator = v.iter();
        print!("[");
        for elem in vecter_iterator {
            print!("{} ", elem);
        }
        println!("]");
    }
impl ProblemRemoveDuplicateItems {


    // function to solving the RemoveDuplicateItems problem
    pub fn solve(&mut self) -> usize
    {
        println!("Solving problem [{}]", self.name);

        if (self.v.len()) < 2  {
            return self.v.len();
        }
        else {
            show_array(&self.v);
            let mut cursor : usize = 0;
            for _ind in 1..(self.v.len())
            {
                if self.v[cursor] != self.v[_ind] {
                    cursor = cursor+1;
                    self.v[cursor] = self.v[_ind];
                }
            }

            println!("After algorithm running, vector length is {} ", cursor+1);
            show_array(&self.v);
            return cursor+1;
        }
    }

    pub fn new(v: Vec<i32>, name: &str) -> Self {
        ProblemRemoveDuplicateItems {
            v: v,
            name: name.to_string(),
        }
    }

}
