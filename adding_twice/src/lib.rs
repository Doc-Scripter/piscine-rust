pub fn twice<T:'static, F:Fn(T) -> T + 'static>(f: F) -> Box<dyn Fn(T) -> T>{
    Box::new(move |x| f(f(x)))
}

pub fn add_curry(x :i32)->Box<dyn Fn(i32)->i32>{
    Box::new(move|y|y+x)
}