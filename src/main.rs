fn main() {
    let mut database:Vec<Banking>=Vec::new();
    register("Akif".to_string(), "4524525453".to_string(), &mut database);

}
#[derive(Debug,Clone)]
struct Banking{
    name:String,
    tc:String
}
 
 fn register(name:String,tc:String,database :&mut Vec<Banking>){
    let banking:Banking=Banking{
        name:name,
        tc:tc

    };
    database.push(banking.clone());
    print!("bankamıza kayıt oldunuz{:?}",database)

 }
 


