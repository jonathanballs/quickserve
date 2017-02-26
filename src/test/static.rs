static mut COUNTER:i32 = 0;
fn main()
{
    add();
    unsafe
    {
        println!("{:?}",COUNTER);
    }
}

fn add()
{
    unsafe
    {
    COUNTER = COUNTER + 1;
    }
}
