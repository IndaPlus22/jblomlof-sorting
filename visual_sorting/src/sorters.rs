///yeet_sort constantly compares two consecutive elements, if the first one is greater than the other.
/// It yeets it to the back.
pub fn yeet_sort(_list: &mut [isize]) -> Vec<isize> {
    // my custom made sort. I came up with the idea my self but dont know if it already exists.
    let mut index = 0;
    let mut return_vec: Vec<isize> = vec![];
    return_vec.extend(_list.iter());
    while index < _list.len() - 1 {
        if _list[index] > _list[index + 1] {
            // the element thats infront is higher so we yeet it (_list[index]) to the back.
            let temp = _list[index];
            for j in index..(_list.len() - 1) {
                _list[j] = _list[j + 1];
            }
            _list[_list.len() - 1] = temp;
            index = 0;
            return_vec.extend(_list.iter());
        } else {
            index += 1;
        }
    }
    return_vec
}
pub fn insert_sort(_list: &mut [isize]) -> Vec<isize> {
    let mut return_vec: Vec<isize> = vec![];
    return_vec.extend(_list.iter());
    let mut update_flag = false;
    for i in 1.._list.len() {
        let x = _list[i];
        let mut j: isize = i as isize - 1;
        while (j >= 0) && (_list[j as usize] > x) {
            update_flag = true;
            _list[j as usize + 1] = _list[j as usize];
            j -= 1;
        }
        _list[(j + 1) as usize] = x;
        if update_flag {
            return_vec.extend(_list.iter());
        }
        update_flag = false;
    }
    return_vec
}

pub fn selection_sort(_list: &mut [isize]) -> Vec<isize> {
    let mut return_vec: Vec<isize> = vec![];
    return_vec.extend(_list.iter());
    for i in 0..(_list.len() - 1) {
        let mut lowest_index = i;
        for j in (i + 1).._list.len() {
            if _list[j] < _list[lowest_index] {
                lowest_index = j;
            }
        }
        if lowest_index != i {
            let temp = _list[lowest_index];
            _list[lowest_index] = _list[i];
            _list[i] = temp;
        }
        return_vec.extend(_list.iter());
    }
    return_vec
}

pub fn merge_sort(_list: &mut [isize]) -> Vec<isize> {
    let mut return_vec: Vec<isize> = vec![];
    
    if _list.len() != 1 {
        /*let mut left_side = &mut _list[0..(_list.len() / 2)];
        let mut right_side = &mut _list[(_list.len() / 2).._list.len()];*/
        let n = _list.len() / 2;
        let m = _list.len();
        let result = merge_sort(&mut _list[0..n]);
        for i in 0..(result.len() / n) {
            for j in 0..n {
                return_vec.push(*result.get(i * n + j).unwrap());
            }
            return_vec.extend(&_list[n..m]);
        }
        let result2 = merge_sort(&mut _list[n..m]);
        for i in 0..(result2.len() / (m - n)) {
            return_vec.extend(&_list[0..n]);
            for j in 0..(m-n) {
                return_vec.push(*result2.get(i * (m - n) + j).unwrap());
            }
        }
        // I couldn't solve problem with borrowing twice so i got "inspired" by
        // https://www.hackertouch.com/merge-sort-in-rust.html
        // previous attempt commented out
        //merge(&mut _list, &left_side, &right_side);
        //---
        //return_vec.extend(_list.iter());

        let temp = merge(&_list[0..n], &_list[n..m]);
        _list.copy_from_slice(&temp);
        return_vec.extend(_list.iter());
    }
    return_vec
}

pub fn merge(_list_a: &[isize], _list_b: &[isize]) -> Vec<isize> {
    let mut left_counter = 0;
    let mut right_counter = 0;
    let mut return_vec = vec![];
    while (left_counter < _list_a.len()) && (right_counter < _list_b.len()) {
        return_vec.push(if _list_a[left_counter] < _list_b[right_counter] {
            left_counter += 1;
            _list_a[left_counter - 1]
        } else {
            right_counter += 1;
            _list_b[right_counter - 1]
        })
    }
    for i in left_counter.._list_a.len() {
        return_vec.push(_list_a[i]);
    }
    for i in right_counter.._list_b.len() {
        return_vec.push(_list_b[i]);
    }
    return_vec
}

/* pub fn merge(_complete_list : &mut [isize], _list_a : &[isize], _list_b : &[isize]) {
    let mut left_counter  = 0;
    let mut right_counter = 0;
    let mut complete_counter = 0;
    while (left_counter < _list_a.len()) && (right_counter < _list_b.len()) {
        if _list_a[left_counter] < _list_b[right_counter] {
            _complete_list[complete_counter] = _list_a[left_counter];
            left_counter += 1;
        } else {
            _complete_list[complete_counter] = _list_b[right_counter];
            right_counter += 1;
        }
        complete_counter += 1;
    }
    for i in left_counter.._list_a.len() {
        _complete_list[complete_counter] = _list_a[i];
        complete_counter += 1;
    }
    for i in right_counter.._list_b.len() {
        _complete_list[complete_counter] = _list_b[i];
        complete_counter += 1;
    }
} */
