// さらにこのmodをこのファイルをcallしているところで使いたい場合 pubをつける。
pub mod sub_a;
pub mod sub_b;

// defaultで privateなので pubを付けて publicにしないといけない
pub fn run() {
    println!("Here is vars modules!!!!!");
    sub_a::func_a();
    sub_b::func_b();
}
