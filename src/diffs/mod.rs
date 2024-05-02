use crate::tree_compare::DiffCheckCoord;

fn signed(num: i32) -> String {
    if num > 0 {
        format!("+{}", num)
    } else {
        num.to_string()
    }
}

pub fn create_diffs(diffs: Vec<DiffCheckCoord>, left_file: &str, right_file: &str) -> Vec<String> {
    if diffs.len() == 0 {
        return vec!["\n\t\t\tNo structural diff found between files.\n".to_string()];
    }
    let mut diff_strings: Vec<String> = Vec::new();
    for diff in diffs {
        let left_first_row_num:i32 = left_file[0..diff.start_byte_left].split('\n').count() as i32 -1;
        let right_first_row_num = right_file[0..diff.start_byte_right].split('\n').count() as i32 -1;
        let left_diff = &left_file[diff.start_byte_left..diff.end_byte_left]
            .split('\n')
            .collect::<Vec<&str>>();
        let right_diff = &right_file[diff.start_byte_right..diff.end_byte_right]
            .split('\n')
            .collect::<Vec<&str>>();
        let mut current_diff = difflib::unified_diff(
            &left_diff,
            &right_diff,
            "before",
            "after",
            "",
            "",
            3,
        );
        // for line in &current_diff {
        //     println!("{};;;", line);
        // }
        let numbers = current_diff[2].split(" ").collect::<Vec<&str>>();
        let left_nums = numbers[1].split(",").collect::<Vec<&str>>();
        let right_nums = numbers[2].split(",").collect::<Vec<&str>>();

        let left_start : i32 = if let Ok(l) = left_nums[0].parse::<i32>() {
            if l > 0 {
                l + left_first_row_num
            }
            else {
                l - left_first_row_num
            }
        } else { 1 };

        let right_start = if let Ok(right_start) = right_nums[0].parse::<i32>() {
            if right_start > 0 {
                right_start + right_first_row_num
            }
            else {
                right_start - right_first_row_num
            }
        } else { 1 };

        let left_finish = if left_nums.len() > 1 {
            format!("{},{}", signed(left_start), left_nums[1])
        } else {
            signed(left_start)
        };

        let right_finish = if right_nums.len() > 1 {
            format!("{},{}", signed(right_start), right_nums[1])
        } else {
            signed(right_start)
        };

        current_diff[2] = format!("@@ {} {} @@\n", left_finish, right_finish);
        for i in 3..current_diff.len() {
            current_diff[i] = format!("{}{}", current_diff[i], "\n");
        }
        //@@ -1 +1 @@
        diff_strings.extend(current_diff);
        diff_strings.push("\n##########################\n\n".to_string());
    }
    diff_strings
}
