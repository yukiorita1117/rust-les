mod vars;

fn main() {
    println!("Hello, world!");
    // vars::の：：はvars内の階層をたぐってる。   vars.run() 的な。
    vars::run();
    vars::sub_a::func_a();
    vars::sub_b::func_b();
}
