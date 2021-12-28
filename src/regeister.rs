pub struct Regeister {
    stack: Vec<String>,
    near_stack: Vec<String>,
}

impl Regeister {
    pub fn init() -> Self {
        Self {
            stack: vec!["t9".to_string(),"t8".to_string(),"t7".to_string(),"t6".to_string(),"t5".to_string(),"t4".to_string(),"t3".to_string(),"t2".to_string(),"t1".to_string(),"t0".to_string()],
            near_stack: vec![]
        }
    }
    pub fn eat(&mut self) -> String {
        let a = self.stack.pop().unwrap();
        let b = a.clone();
        self.near_stack.push(a);
        b
    }

    pub fn near(&mut self) -> String{
        self.near_stack.pop().unwrap()
    }

    pub fn put_near(& mut self, str: String) {
        self.near_stack.push(str);
    }    

    // pub fn take<'a,'b>(&'a mut self) -> &'b str {
    //     self.stack[self.stack.len() - 1]
    // }
}