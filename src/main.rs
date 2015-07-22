extern crate redis;
use redis::Commands;
fn main() {
    real_main().unwrap();
}

fn real_main() -> redis::RedisResult<isize> {
    let client = try!(redis::Client::open("redis://127.0.0.1/"));
    let con = try!(client.get_connection());
    let _ : () = try!(con.del("non-existent-set"));

    let zset : Vec<String> = try!(con.zrangebyscore_withscores("non-existent-set", 0, 3));
    println!("Empty String set: {:?}", zset);

    let zset : Vec<(String,u32)> = try!(con.zrangebyscore_withscores("non-existent-set", 0, 3));
    println!("Empty Tuple set: {:?}", zset);

    Ok(1)
}
