//just the same sorters as sorters.rs but with vectors

///yeet_sort constantly compares two consecutive elements, if the first one is greater than the other.
/// It yeets it to the back.
pub fn yeet_sort(_list: &mut Vec<isize>) -> Vec<Vec<isize>> {
    // my custom made sort. I came up with the idea my self but dont know if it already exists.
    let mut index = 0;
    let mut return_vec: Vec<Vec<isize>> = vec![];
    return_vec.push(_list.to_vec());
    while index < _list.len() - 1 {
        if _list.get(index).unwrap() > _list.get(index + 1).unwrap() {
            // the element thats infront is higher so we yeet it (_list[index]) to the back.
            let temp = _list.remove(index);
            _list.push(temp);
            index = 0;
            return_vec.push(_list.to_vec());
        } else {
            index += 1;
        }
    }
    return_vec
}
pub fn insert_sort(_list: &mut Vec<isize>) -> Vec<Vec<isize>> {
    let mut return_vec: Vec<Vec<isize>> = vec![];
    return_vec.push(_list.to_vec());
    for i in 1.._list.len() {
        let x = _list.remove(i);
        let mut j: isize = i as isize - 1;
        while (j >= 0) && (*_list.get(j as usize).unwrap() > x) {
            j -= 1;
        }
        _list.insert((j + 1) as usize, x);
        if j < i as isize - 1 {
            return_vec.push(_list.to_vec());
        }
    }
    return_vec
}

pub fn selection_sort(_list: &mut Vec<isize>) -> Vec<Vec<isize>> {
    let mut return_vec: Vec<Vec<isize>> = vec![];
    return_vec.push(_list.to_vec());
    for i in 0..(_list.len() - 1) {
        let mut lowest_index = i;
        for j in (i + 1).._list.len() {
            if _list.get(j).unwrap() < _list.get(lowest_index).unwrap() {
                lowest_index = j;
            }
        }
        if lowest_index != i {
            let from_switch = _list.remove(lowest_index);
            let to_switch = _list.remove(i);
            _list.insert(i, from_switch);
            _list.insert(lowest_index, to_switch);
        }
        return_vec.push(_list.to_vec());
    }
    return_vec
}

pub fn merge_sort(_list: &mut Vec<isize>) ->Vec<Vec<isize>> {
    let mut return_vec: Vec<Vec<isize>> = vec![];
    
    if _list.len() != 1 {
        let n = _list.len() / 2;
        let mut second_half = _list.split_off(n);
        let result = merge_sort(_list);
        for mut i in result{
            i.extend(second_half.as_slice());
            return_vec.push(i.to_vec());
        }
        let result2 = merge_sort(&mut second_half);
        for i in result2{
            let mut temp_vec : Vec<isize> = vec![];
            temp_vec.extend(_list.as_slice());
            temp_vec.extend(i.as_slice());
            return_vec.push(temp_vec.to_vec());
        }
        // I couldn't solve problem with borrowing twice so i got "inspired" by
        // https://www.hackertouch.com/merge-sort-in-rust.html
        // previous attempt commented out, but onlt in sorters.rs
        //merge(&mut _list, &left_side, &right_side);
        //---
        //return_vec.extend(_list.iter());

        let temp = merge(&_list, &second_half);
        *_list = temp.to_vec();
        return_vec.push(_list.to_vec());
    }
    return_vec
}

pub fn merge(_list_a: &Vec<isize>, _list_b: &Vec<isize>) -> Vec<isize> {
    let mut left_counter = 0;
    let mut right_counter = 0;
    let mut return_vec = vec![];
    while (left_counter < _list_a.len()) && (right_counter < _list_b.len()) {
        return_vec.push(if _list_a.get(left_counter).unwrap() < _list_b.get(right_counter).unwrap() {
            left_counter += 1;
            *_list_a.get(left_counter - 1).unwrap()
        } else {
            right_counter += 1;
            *_list_b.get(right_counter - 1).unwrap()
        })
    }
    for i in left_counter.._list_a.len() {
        return_vec.push(*_list_a.get(i).unwrap());
    }
    for i in right_counter.._list_b.len() {
        return_vec.push(*_list_b.get(i).unwrap());
    }
    return_vec
}
