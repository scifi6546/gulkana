use serde::{Serialize, Deserialize};
#[derive(Copy,Clone,PartialEq,Eq,Serialize,Deserialize)]
pub struct OptStruct<A:Clone,B:Clone>{
    pub a:Option<A>,
    pub b:Option<B>,
}
pub fn new_optstruct_a<A,B>(input:A)->OptStruct<A,B>
    where
        A:std::clone::Clone,
        B:std::clone::Clone
    {
    return OptStruct{
        a:Some(input),
        b:None,
    }
}
pub fn new_optstruct_b<A,B>(input:B)->OptStruct<A,B>
    where
        A:std::clone::Clone,
        B:std::clone::Clone
    {
    return OptStruct{
        a:None,
        b:Some(input),
    }
}
impl<A:std::clone::Clone,B:std::clone::Clone> OptStruct<A,B>{
    pub fn a(&self)->Option<&A>{

        return self.a.as_ref();  
    }
    pub fn b(&self)->Option<&B>{
        return self.b.as_ref();
    }
    
}

