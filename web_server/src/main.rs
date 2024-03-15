use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

fn main() {
    println!("Hello, world!");
    let nums: Vec<i32> = vec![1, 2, 7, 4];
    let nums2: Vec<i32> = vec![5, -4, 1, -2, -2, -2, 4];

    nums_game(&nums);
    magic_tower(&nums2);
    let listener: TcpListener = TcpListener::bind("127.0.0.1:7878").unwrap();
    for i_stream in listener.incoming() {
        let i_stream = i_stream.unwrap();
        handle_connection(i_stream);
    }
    println!("------啦啦啦-----");
}

fn handle_connection(mut i_stream: TcpStream) {
    let buf_reader = BufReader::new(&mut i_stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    let status_line = "HTTP/1.1 200 OK";
    let contents = fs::read_to_string("index.html").unwrap();
    let length = contents.len();
    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
    i_stream.write_all(response.as_bytes()).unwrap();

    println!("Request: ${:#?}", http_request);
}

fn nums_game(nums: &Vec<i32>) -> Vec<i32> {
    //最终造作数
    let mut keys_array: Vec<i32> = Vec::new();
    //最终操作数取余
    let mut result_array: Vec<i32> = Vec::new();
    //最终操作的结果
    let mut cur_array: Vec<i32> = Vec::new();

    for (i, val) in nums.iter().enumerate() {
        let mut result_num: i32 = 0;

        //i为0操作数为0
        if i == 0 {
            keys_array.push(result_num);
            result_array.push(result_num);
            cur_array.push(*val);
            continue;
        }

        //i = 1 比较两个值求操作数
        if i == 1 {
            if nums[i - 1] >= *val {
                result_num = nums[i - 1] - val + 1;
                cur_array.push(*val + result_num);
            } else {
                result_num = val - nums[i - 1] - 1;
                cur_array.push(*val - result_num);
            }
            result_num += keys_array[i - 1];
            keys_array.push(result_num);
            result_array.push(result_num % 1000000007);
            continue;
        }

        if cur_array[i - 1] >= *val {
            result_num = cur_array[i - 1] - val + 1;
            cur_array.push(*val + result_num);
        } else {
            result_num = val - cur_array[i - 1] - 1;
            cur_array.push(*val - result_num);
        }

        result_num += keys_array[i - 1];
        keys_array.push(result_num);
        result_array.push(result_num % 1000000007);
    }

    result_array
}

fn magic_tower(nums: &Vec<i32>) -> i32 {
    let mut lower: i32 = 0;
    let mut _default_num: i32 = 1;
    let mut push_num: i32 = 0;
    for (i, val) in nums.iter().enumerate() {
        if i + 1 < nums.len() {
            if _default_num + val + nums[i + 1] >= 1 {
                _default_num += *val;
            } else if *val > 0 {
                _default_num += *val;
            } else {
                push_num += *val;
                lower += 1
            }
        } else {
            if _default_num + val + push_num < 1 {
                lower = -1;
            }
        }
    }
    println!("-----------__{}", lower);
    lower
}

#[cfg(test)]
mod test {
    use super::magic_tower;
    #[test]
    fn test_qc() {
        let nums: Vec<i32> = vec![100, 100, 100, -250, -60, -140, -50, -50, 100, 150];
        magic_tower(&nums);
    }
}
