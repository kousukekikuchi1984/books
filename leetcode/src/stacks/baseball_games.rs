impl Solution {
    pub fn cal_points(operations: Vec<String>) -> i32 {
        /* x: integer
         * +: sum previous two scores and append it
         * D: double the previous score
         * C: delete the last scores
         * */
        let mut queue: Vec<i32> = vec![];
        for operation in operations {
            let queue_length = queue.len();
            match operation.as_str() {
                "+" => {
                    let sum = queue[queue_length - 1] + queue[queue_length - 2];
                    queue.push(sum);
                }
                "D" => {
                    let multiply = queue[queue_length - 1] * 2;
                    queue.push(multiply)
                }
                "C" => {
                    queue.pop();
                }
                _ => queue.push(operation.parse::<i32>().unwrap()),
            }
        }
        queue.iter().sum()
    }
}
