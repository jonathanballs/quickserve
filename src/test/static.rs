static mut counter:i32 = 0;
fn main()
{
    add();
    unsafe
    {
        println!("{:?}",counter);
    }
}

fn add()
{
    unsafe
    {
    counter = counter + 1;
    }
}
