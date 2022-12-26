pub fn permute<T: Clone>(nums: Vec<T>) -> Vec<Vec<T>> {
    let mut vec = Vec::new();
    if nums.len() == 1 {
        vec.push(nums);
    } else {
        for (i, _item) in nums.iter().enumerate() {
            let left: &T = &nums[i];
            let mut v1 = Vec::new();

            v1.push(&nums[0..i]);
            v1.push(&nums[i + 1..]);

            let right = v1.concat();
            let arr = permute(right);

            for (j, _item2) in arr.iter().enumerate() {
                let mut vec2 = Vec::new();
                vec2.push(left.clone());

                for item3 in &arr[j] {
                    vec2.push(item3.clone());
                }

                vec.push(vec2);
            }
        }
    }

    vec
}

pub fn compare_vec<T: PartialEq>(a: &Vec<T>, b: &Vec<T>) -> bool {
    if a.len() != b.len() {
        return false;
    }

    for item in a.iter() {
        if b.contains(item) {
            continue;
        } else {
            return false;
        }
    }

    for item in b.iter() {
        if a.contains(item) {
            continue;
        } else {
            return false;
        }
    }

    true
}

pub fn compare_arr<T: PartialEq>(a: &[T], b: &[T]) -> bool {
    if a.len() != b.len() {
        return false;
    }

    for item in a.iter() {
        if b.contains(item) {
            continue;
        } else {
            return false;
        }
    }

    for item in b.iter() {
        if a.contains(item) {
            continue;
        } else {
            return false;
        }
    }

    true
}
