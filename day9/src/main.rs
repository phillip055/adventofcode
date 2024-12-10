use std::fs;

fn parse_input(input: &str) -> Vec<Option<char>> {
    let mut result = Vec::new();
    let mut is_file_size = true;
    let mut file_id = 0;

    for c in input.chars() {
        let count = c.to_digit(10).unwrap() as usize;
        if is_file_size {
            result.extend((0..count).map(|_| Some(char::from_u32(file_id as u32).unwrap())));
            file_id += 1;
        } else {
            result.extend((0..count).map(|_| None));
        }
        is_file_size = !is_file_size;
    }
    result
}

fn defragment(mut files: Vec<Option<char>>) -> Vec<Option<char>> {
    let mut left = 0;
    let mut right = files.len() - 1;

    while left < right {
        while files[left] != None {
            left += 1;
        }
        while files[right] == None {
            right -= 1;
        }
        if left < right {
            files.swap(right, left);
        }
        left += 1;
        right -= 1;
    }
    return files;
}

fn find_first_slot(files: Vec<Option<char>>, size: usize, start: usize, end: usize) -> Option<usize> {
    for i in start..end {
        if i + size > files.len() {
            break;
        }
        if files[i..i+size].iter().all(|c| c.is_none()) {
            return Some(i);
        }
    }
    return None;
}

fn insert_file(mut files: Vec<Option<char>>, file_id: char, start: usize, size: usize) -> Vec<Option<char>> {
    for i in start..start + size {
        files[i] = Some(file_id);
    }
    files
}

fn defragment_group(mut files: Vec<Option<char>>) -> Vec<Option<char>> {
    let mut end = (files.len() - 1) as u32;
    while end > 0 {
        if files[end as usize].is_none() {
            end -= 1;
            continue;
        }
        let file_id = files[end as usize].unwrap();
        let mut start = end;
        while start > 0 && files[start as usize].is_some() && files[start as usize].unwrap() == file_id {
            start -= 1;
        }
        let size = end - start;
        let slot = find_first_slot(files.clone(), size as usize, 0, start as usize);        
        if slot.is_none() {
            while end > 0 && files[end as usize].is_some() && files[end as usize].unwrap() == file_id {
                end -= 1;
            }
            continue;
        }
        files = insert_file(files, file_id, slot.unwrap(), size as usize);

        for i in start+1..end+1 {
            files[i as usize] = None
        }

        end = start;
    }
    return files
}

fn file_checksum(files: Vec<Option<char>>) -> u64 {
    let mut result = 0;
    for (i, c) in files.iter().enumerate() {
        match c {
            Some(c) => {
                result += i as u64 * *c as u64;
            }
            None => {}
        }
    }
    result
}

fn main() {
    let input = fs::read_to_string("input/sample.txt").unwrap();
    let files = parse_input(&input);
    let defragmented = defragment(files.clone());
    let checksum = file_checksum(defragmented);
    println!("{:?}", checksum);
    let defragmented_group = defragment_group(files.clone());

    let checksum = file_checksum(defragmented_group);
    println!("{:?}", checksum);
}
