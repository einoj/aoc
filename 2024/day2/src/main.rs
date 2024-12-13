use std::io::{self,BufRead};

fn checksafe(nums: Vec<i32>, dampen: bool) -> bool {
    let mut growing: bool = true;
    if nums[0] > nums[1] {
        growing = false;
    }
    for (i, curr) in nums.iter().enumerate() {
        if i == nums.len()-1 {
            return true;
        }
        let next = nums[i+1];
        if *curr == next {
            if ! dampen {
                return false;
            } 
            let mut nums2 = nums.clone();
            nums2.remove(i);
            let mut nums3 = nums.clone();
            nums3.remove(i+1);
            if i > 0 {
                let mut nums4 = nums.clone();
                nums4.remove(i-1);
                if checksafe(nums4, false) {
                    return true;
                }
            }
            if checksafe(nums2, false) {
                return true;
            }
            return checksafe(nums3, false);
        }
        if (*curr > next) && growing {
            if ! dampen {
                return false;
            }
            let mut nums2 = nums.clone();
            nums2.remove(i);
            let mut nums3 = nums.clone();
            nums3.remove(i+1);
            if i > 0 {
                let mut nums4 = nums.clone();
                nums4.remove(i-1);
                if checksafe(nums4, false) {
                    return true;
                }
            }
            if checksafe(nums2, false) {
                return true;
            }
            return checksafe(nums3, false);
        }
        if growing && (next-curr > 3) {
            if ! dampen {
                return false;
            }
            let mut nums2 = nums.clone();
            nums2.remove(i);
            let mut nums3 = nums.clone();
            nums3.remove(i+1);
            if i > 0 {
                let mut nums4 = nums.clone();
                nums4.remove(i-1);
                if checksafe(nums4, false) {
                    return true;
                }
            }
            if i > 0 {
                let mut nums4 = nums.clone();
                nums4.remove(i-1);
                if checksafe(nums4, false) {
                    return true;
                }
            }
            if checksafe(nums2, false) {
                return true;
            }
            return checksafe(nums3, false);
        }
        if (*curr < next) && !growing{
            if ! dampen {
                return false;
            }
            let mut nums2 = nums.clone();
            nums2.remove(i);
            let mut nums3 = nums.clone();
            nums3.remove(i+1);
            if i > 0 {
                let mut nums4 = nums.clone();
                nums4.remove(i-1);
                if checksafe(nums4, false) {
                    return true;
                }
            }
            if checksafe(nums2, false) {
                return true;
            }
            return checksafe(nums3, false);
        }
        if (!growing) && (*curr-next > 3) {
            if ! dampen {
                return false;
            }
            let mut nums2 = nums.clone();
            nums2.remove(i);
            let mut nums3 = nums.clone();
            nums3.remove(i+1);
            if i > 0 {
                let mut nums4 = nums.clone();
                nums4.remove(i-1);
                if checksafe(nums4, false) {
                    return true;
                }
            }
            if checksafe(nums2, false) {
                return true;
            }
            return checksafe(nums3, false);
        }
    }
    return true;
}

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines();
    let mut sum = 0;
    for line in lines {
        let sline = line.unwrap();
        let nums = sline.split_whitespace().filter_map(|s| s.parse::<i32>().ok()).collect::<Vec<i32>>();
        if checksafe(nums, true) {
            sum+=1; 
        }
    }
    println!("Sum = {}", sum);
}
