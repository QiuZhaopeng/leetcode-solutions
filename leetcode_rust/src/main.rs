mod problems;

use problems::ProblemRemoveDuplicateItems::ProblemRemoveDuplicateItems;

fn main() {
    let data_vec:Vec<i32> =  vec![1,1,2,2,3,4,5,6,6];
    let mut pb: ProblemRemoveDuplicateItems = ProblemRemoveDuplicateItems::new(data_vec, "ProblemRemoveDuplicateItems");
    pb.solve();
}
