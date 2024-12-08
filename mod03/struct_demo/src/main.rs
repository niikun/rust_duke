
struct Person{
    name: String,
    email:String,
    active:bool,
}
impl Person{
    fn new(name:&str,email:&str)->Self{
        Person{
            name: String::from(name),
            email:String::from(email),
            active:true,
        }
    }
    fn deactivate(&mut Self){
        self.active = false;
    }
}

fn main() {
    let mut niikun = Person::new("niikun","niikun@email.com");
    println!("name: {}, email: {}, active: {}", niikun.name, niikun.email, niikun.active);
    niikun.deactivate();
    println!("name: {}, email: {}, active: {}", niikun.name, niikun.email, niikun.active);  

}
