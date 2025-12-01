fn main(){
    // in genral we use {} as a placeholder will be automatically replaced with any arguments we provide
    println!("{} days", 31);
    // positional arguments can be used multiple times. 
    // Specifying an integer inside the curly brackets{} selects which argument to place there.
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");
    // as can named arguments be used
    println!("{subject} {verb} {object}",
        object="the lazy dog", subject="the quick brown fox", verb="jumps over");   
        
    // Differnet formating can be specified by appending a colon (:) 
    // after the placeholder and then the format specifier
    println!("Base 10:  {}", 69420);
    println!("Base 2:   {:b}", 69420);
    println!("Base 8:   {:o}", 69420);
    println!("Base 16:  {:x}", 69420);
    println!("Base 16:  {:X}", 69420);
}