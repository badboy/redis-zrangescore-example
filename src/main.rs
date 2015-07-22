extern crate redis;
use redis::Commands;
fn main() {
    real_main().unwrap();
}

fn real_main() -> redis::RedisResult<isize> {
    let client = try!(redis::Client::open("redis://127.0.0.1/"));
    let con = try!(client.get_connection());
    // throw away the result, just make sure it does not fail
    let _ : () = try!(con.del("myzset"));
    let _ : () = try!(con.zadd("myzset", "one", 1));
    let _ : () = try!(con.zadd("myzset", "two", 2));
    let _ : () = try!(con.zadd("myzset", "three", 3));

    let zset : Vec<String> = try!(con.zrangebyscore_withscores("myzset", 0, 3));
    println!("Only Strings: {:?}", zset);

    let zset : Vec<(String,u32)> = try!(con.zrangebyscore_withscores("myzset", 0, 3));
    println!("Parsed Tuple: {:?}", zset);

    let res : redis::RedisResult<Vec<String>> = con.zrangebyscore_withscores("myzset", 0, 3);
    match res {
        Ok(val) => println!("Explicit check: {:?}", val),
        Err(_) => panic!("An error occured")
    }

    let zset : Vec<String> = try!(con.zrangebyscore_withscores("non-existent-set", 0, 3));
    println!("Empty String set: {:?}", zset);

    Ok(1)
}
