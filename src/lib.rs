// Use a procedural macro to generate bindings for the world we specified in
// `host.wit`
wit_bindgen::generate!("host");

// Define a custom type and implement the generated `Host` trait for it which
// represents implementing all the necesssary exported interfaces for this
// component.
struct MyHost;

impl Host for MyHost {
    
    fn myfunction1() -> String {
         let x = name();
         return x.to_string();
         //return String::from("hello world");
    }
    fn myfunction2(Value :String) -> String {
        return String::from("hello world2");
   }
}

export_host!(MyHost);
