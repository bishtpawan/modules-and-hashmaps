fn main() {
    crate::first::one();
    first::inside_first_module::two();

}
mod first{
    pub fn one(){
        println!("This is function of module first");
    }
    pub fn xyz(){
        println!("This is called by inner module with the help of super keyword");
    }
    pub mod inside_first_module{
        pub fn two(){
            println!("This is fucntion of module which is declared inside the first module  ");
            super::xyz();
        }

    }
}
