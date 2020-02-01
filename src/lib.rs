use std::collections::BTreeMap;
use std::fmt;
//rust lint does not see that rand is used so to kill error
#[allow(unused_imports)]
use rand::prelude;
use serde::{Serialize, Deserialize};
mod opt_pair;
use opt_pair::{OptStruct,new_optstruct_a,new_optstruct_b};
/// # Gulkana
/// Gulkana is a lightweight key based database for string files.
/// The main struct is DataStructure

#[derive(PartialEq,Eq,Clone,Serialize,Deserialize)]
pub struct Link<Key:std::clone::Clone,TypeLabel:std::clone::Clone>{
    type_label:TypeLabel,
    children: Vec<Key>,
}
#[derive(PartialEq,Eq,Clone,Serialize,Deserialize)]
pub struct Node<Key:std::cmp::PartialEq+std::clone::Clone,Item:std::clone::Clone,LinkLabel:std::clone::Clone>
{
    item:OptStruct<Link<Key,LinkLabel>,Item>

}
impl <KeyType:std::cmp::PartialEq+std::clone::Clone,DataType:std::clone::Clone,LinkLabel:std::clone::Clone> Node<KeyType,DataType,LinkLabel>{
    pub fn get_item(&self)->Result<&DataType,DBOperationError>{
        let data = self.item.b();
        if data.is_some(){
            return Ok(data.unwrap());
        }else{
            return Err(DBOperationError::NodeNotData);
        }
    }
    pub fn get_item_mut(&mut self)->Result<&mut DataType,DBOperationError>{
        let data = self.item.b_mut();
        if data.is_some(){
            return Ok(data.unwrap());
        }else{
            return Err(DBOperationError::NodeNotData);
        }
    }
}
fn new_node<K:std::cmp::PartialEq+std::clone::Clone,
   I:std::clone::Clone,LinkLabel:std::clone::Clone>(input:I)->Node<K,I,LinkLabel>
    where
        K:std::clone::Clone,
        I:std::clone::Clone,
    {
    let foo =  Node{
        item:new_optstruct_b(input),
    };
    return foo;
}
fn new_node_link<K:std::cmp::PartialEq+std::clone::Clone,
   I:std::clone::Clone,LinkLabel:std::clone::Clone>(input:&std::vec::Vec<K>,link_type:LinkLabel)->Node<K,I,LinkLabel>
    where
        K:std::clone::Clone,
        I:std::clone::Clone,
    {
    let foo =  Node{
        item:new_optstruct_a(
            Link{ 
                children:input.clone(),
                type_label:link_type,
            }),
    };
    return foo;
}
pub enum SerializeError{
    Unknown,

}
pub enum DBOperationError{
    KeyAllreadyPresent,
    KeyNotFound,
    NodeNotLink,
    NodeNotData,
    
}
impl Into<String> for DBOperationError{
    fn into(self)->String{
        match self{
            #[allow(unused)]
            Self::KeyAllreadyPresent => "Key Allready Present".to_string(),
            Self::KeyNotFound => "Key Not found".to_string(),
            Self::NodeNotLink => "Node Not Link".to_string(),
            Self::NodeNotData => "Node Not Data".to_string(),
        }
    }
}
/// Struct usd to store data
/// Inorder to allow new fields in input struct to be added
/// make all fields Optional e.g.
/// ```
/// struct bar{
///     foo:Option<String>,
///     bar:Option<u32>,
/// }
/// ```
/// this way the data structure is compatible with old versions of the database.
#[derive(Clone,PartialEq,Eq,Deserialize,Serialize)]
pub struct DataStructure<KeyType:std::cmp::Ord+std::clone::Clone+Serialize,DataType:std::clone::Clone,LinkLabel:std::clone::Clone+Serialize>{
    tree:BTreeMap<KeyType,Node<KeyType,DataType,LinkLabel>>,
    
}
///Iterator over all data nodes
pub struct DataNodeIter<'a,KeyType:std::cmp::Ord+std::clone::Clone,
    DataType:std::clone::Clone,LinkLabel:std::clone::Clone>{
        iter:std::collections::btree_map::Iter<'a,KeyType, Node<KeyType,DataType,LinkLabel>>
    }
impl<'a,KeyType:std::cmp::Ord+std::clone::Clone,DataType:std::clone::Clone,LinkLabel:std::clone::Clone> Iterator 
    for DataNodeIter<'a,KeyType,DataType,LinkLabel>{
        type Item=(& 'a KeyType,&'a DataType);
        fn next(&mut self)->Option<Self::Item>{
            let data = self.iter.next();
            if data.is_none(){
                return None;
            }
            else{
                let (key,node_unwrapped) = data.unwrap();
                //getting data in node opt_pair;
                let data_opt = node_unwrapped.item.b();
                if data_opt.is_none(){
                    return self.next();
                }else{
                    return Some((key,data_opt.unwrap()));
                }
            }
        }
}
pub struct DataMutIter<'a,KeyType:std::cmp::Ord+std::clone::Clone,DataType:std::clone::Clone,LinkLabel:std::clone::Clone>{
    iter: std::collections::btree_map::IterMut<'a,KeyType,Node<KeyType,DataType,LinkLabel>>,
}
impl<'a,KeyType:std::cmp::Ord+std::clone::Clone,DataType:std::clone::Clone,LinkLabel:std::clone::Clone> Iterator 
    for DataMutIter<'a,KeyType,DataType,LinkLabel>{
        type Item=(&'a KeyType,&'a mut DataType);
        fn next(&mut self)->Option<Self::Item>{
            let data = self.iter.next();
            if data.is_none(){
                return None;
            }
            else{
                let (key,node_unwrapped) = data.unwrap();
                //getting data in node opt_pair;
                let data_opt = node_unwrapped.item.b_mut();
                if data_opt.is_none(){
                    return self.next();
                }else{
                    return Some((key,data_opt.unwrap()));
                }
            }
        }
}
pub struct DataLinkIter<'a,KeyType:std::cmp::Ord+std::clone::Clone+Serialize,DataType:std::clone::Clone,LinkLabel:std::clone::Clone+Serialize>{
        db:&'a DataStructure<KeyType,DataType,LinkLabel>,
        linked_keys: &'a std::vec::Vec<KeyType>,
        current_index: usize,
}
impl<'a,KeyType:std::cmp::Ord+std::clone::Clone+Serialize,
        DataType:std::clone::Clone,
        LinkLabel:std::clone::Clone+Serialize
    > Iterator for DataLinkIter<'a,KeyType,DataType,LinkLabel>{

    type Item=(&'a KeyType,&'a DataType);
    fn next(&mut self)->Option<Self::Item>{
        let opt = self.linked_keys.get(self.current_index);
        if opt.is_some(){
            let res = self.db.get(&opt.unwrap().clone()); 
            if res.is_ok(){

                let data= res.ok().unwrap();
                self.current_index+=1;
                return Some((&opt.unwrap(),data));
            }else{
                return None;
            }
        }else{
            return None;
        }
    }
}
/*
pub struct DataLinkIterMut<'a,KeyType:std::cmp::Ord+std::clone::Clone,
    DataType:std::clone::Clone>{
        db:&'a mut DataStructure<KeyType,DataType>,
        linked_keys: &'a std::vec::Vec<KeyType>,
        current_index: usize,
}
impl<'a,KeyType:std::cmp::Ord+std::clone::Clone,DataType:std::clone::Clone> Iterator
    for DataLinkIterMut<'a,KeyType,DataType>{
    type Item=(&'a KeyType,&'a mut DataType);
    fn next(&mut self)->Option<Self::Item>{
        let opt = self.linked_keys.get(self.current_index);
        if opt.is_some(){
            let res = self.db.get_mut(&opt.unwrap().clone()); 
            if res.is_ok(){

                let data= res.ok().unwrap();
                self.current_index+=1;
                return Some((&opt.unwrap(),data));
            }else{
                return None;
            }
        }else{
            return None;
        }
    }
}*/
impl<KeyType:std::cmp::Ord+std::clone::Clone+Serialize,DataType:std::clone::Clone,LinkLabel:std::clone::Clone+Serialize> DataStructure<KeyType,DataType,LinkLabel>{
    /// Inserts data into datastructure
    /// ```
    /// let mut ds = gulkana::new_datastructure::<u32,u32,u32>();
    /// ds.insert(&10,5);
    /// assert!(ds.insert(&10,20).is_err());
    /// ```
    pub fn insert(&mut self,key:&KeyType,data:DataType)->Result<(),DBOperationError>
    {
        return self.insert_node(key,new_node(data)); 
    }
    ///Used to insert a link into a datastructure
    ///```
    /// let mut ds = gulkana::new_datastructure::<u32,u32,u32>();
    /// ds.insert(&10,5);
    /// ds.insert_link(&9,&vec![10],0);
    /// let iter = ds.iter_links(&9).ok().unwrap();
    /// 
    /// for (i,j) in iter{
    ///     assert!(*j==5);
    /// }
    ///```
    pub fn insert_link(&mut self,key:&KeyType,children:&std::vec::Vec<KeyType>,link_type:LinkLabel)->
        Result<(),DBOperationError>{
        return self.insert_node(key,new_node_link(children,link_type));
        
    }
    ///Overwrites Links with vec shown
    ///```
    /// let mut ds = gulkana::new_datastructure::<u32,u32,u32>();
    /// ds.insert(&10,5);
    /// ds.insert(&11,6);
    /// ds.insert_link(&9,&vec![10],0);
    /// ds.overwrite_link(&9,&vec![11],0);
    /// let iter = ds.iter_links(&9).ok().unwrap();
    /// 
    /// for (_key,data) in iter{
    ///     assert!(*data==6);
    /// }
    /// ````
    pub fn overwrite_link(&mut self,key:&KeyType,children:&std::vec::Vec<KeyType>,link_type:LinkLabel)->
        Result<(),DBOperationError>{
        return self.overwrite_node(key,new_node_link(children,link_type));
    }

    fn insert_node(&mut self,key:&KeyType,data:Node<KeyType,DataType,LinkLabel>)->Result<(),DBOperationError>
        {
        if self.tree.contains_key(key)==false{
            self.tree.insert(key.clone(),data);
            return Ok(());
        }else{
            return Err(DBOperationError::KeyAllreadyPresent);
        }
    

    }
    fn overwrite_node(&mut self,key:&KeyType,
        data:Node<KeyType,DataType,LinkLabel>)->Result<(),DBOperationError>{
            self.tree.insert(key.clone(),data);
            return Ok(());

    }
    /// sets data in database
    /// ```
    /// let mut ds = gulkana::new_datastructure::<u32,u32,u32>();
    /// ds.insert(&10,3);
    /// ds.set_data(&10,&5);
    /// assert!(ds.get(&10).ok().unwrap()==&5);
    /// ```
    pub fn set_data(&mut self,key:&KeyType,
                          data:&DataType)->Result<(),DBOperationError>{
        self.overwrite_node(key,new_node(data.clone()))
         
    }
    fn iter(&self)->
        std::collections::btree_map::Iter<'_, KeyType, Node<KeyType,DataType,LinkLabel>>{
        self.tree.iter()
    }
    fn iter_mut(&mut self)->
    std::collections::btree_map::IterMut<'_, KeyType, Node<KeyType,DataType,LinkLabel>>{
        self.tree.iter_mut()
    }
    /// Used to iterate through data
    ///
    pub fn iter_data(&self)->DataNodeIter<KeyType,DataType,LinkLabel>{
        return DataNodeIter{
            iter:self.iter()
        };
    }
    /// Iterates through data mutably
    /// ```
    /// let mut ds = gulkana::new_datastructure::<u32,u32,u32>();
    /// ds.insert(&10,3);
    /// for (k,n) in ds.iter_data_mut(){
    ///     *n=5;
    /// }
    /// assert!(ds.get(&10).ok().unwrap()==&5);
    /// ```
    pub fn iter_data_mut(&mut self)->DataMutIter<KeyType,DataType,LinkLabel>{
        return DataMutIter{
            iter:self.iter_mut(),
        }
    }
    /// gets key from database
    /// ```
    ///
    /// let mut ds = gulkana::new_datastructure::<u32,u32,u32>();
    /// ds.insert(&10,5);
    /// let data = ds.get(&10);
    /// assert!(*data.ok().unwrap()==5); 
    /// ```
    pub fn get(&self,key:&KeyType)->Result<&DataType,DBOperationError>
        where
            KeyType : std::cmp::Ord,
    {
        let temp = self.tree.get(key);
        if temp.is_none(){

            return Err(DBOperationError::KeyNotFound);
        }else{
            return temp.unwrap().get_item();
        }
    }
    /// Gets data associated with key mutably
    /// ```
    /// let mut ds = gulkana::new_datastructure::<u32,u32,u32>();
    /// ds.insert(&10,5);
    /// let data = ds.get_mut(&10).ok().unwrap();
    /// *data=10;
    /// assert!(ds.get(&10).ok().unwrap()==&10); 
    /// ```
    pub fn get_mut(&mut self,key:&KeyType)->Result<& '_ mut DataType,DBOperationError>{
        let temp = self.tree.get_mut(key);
        if temp.is_none(){
            return Err(DBOperationError::KeyNotFound);
        }else{
            return temp.unwrap().get_item_mut();
        }

    }
    fn get_node(&self,key:&KeyType)->Result<&Node<KeyType,DataType,LinkLabel>,DBOperationError>{
        let item = self.tree.get(key);
        if item.is_some(){
            return Ok(item.unwrap());
        }else{
            return Err(DBOperationError::KeyNotFound);
        }
    }
    /// Gets linked nodes
    /// ```
    /// let mut ds = gulkana::new_datastructure::<u32,u32,u32>();
    /// ds.insert(&10,5);
    /// ds.insert(&11,6);
    /// ds.insert_link(&9,&vec![10],0);
    /// let v = ds.get_links(&9).ok().unwrap();
    /// assert!(v[0]==10);
    /// ````
    pub fn get_links(&self,key:&KeyType)->Result<&Vec<KeyType>,DBOperationError>{
        let data = self.get_node(key)?;
        let vec_temp = data.item.a();
        if vec_temp.is_some(){
            return Ok(&vec_temp.unwrap().children);
        }else{
            return Err(DBOperationError::NodeNotLink);
        }
    }
    /// Gets Link Type
    /// ```
    /// let mut ds = gulkana::new_datastructure::<u32,u32,u32>();
    /// ds.insert(&10,5);
    /// ds.insert(&11,6);
    /// ds.insert_link(&9,&vec![10],0);
    /// let v = ds.get_link_type(&9).ok().unwrap();
    /// assert!(v==0);
    /// ````
    pub fn get_link_type(&self,key:&KeyType)->Result<LinkLabel,DBOperationError>{
        let data = self.get_node(key)?;
        if data.item.a().is_some(){
            return Ok(data.item.a().unwrap().type_label.clone());

        }else{
            return Err(DBOperationError::NodeNotLink);
        }

    }
    /// Iterates through nodes attached to link
    ///
    pub fn iter_links(&self,key:&KeyType)->Result<DataLinkIter<KeyType,DataType,LinkLabel>,DBOperationError>{
        return Ok(DataLinkIter{
                db:self,
                linked_keys:self.get_links(key)?,
                current_index:0,
        });
                
    }
    /// Checks if database contains a given key
    /// ```
    /// let mut ds = gulkana::new_datastructure::<u32,u32,u32>();
    /// ds.insert(&10,5);
    /// assert!(ds.contains(&10));
    /// assert!(!ds.contains(&20));
    /// ```
    pub fn contains(&self,key:&KeyType)->bool{
        return self.tree.get(key).is_some();
    }
    /// Appends to link in database
    ///```
    /// let mut ds = gulkana::new_datastructure::<u32,u32,u32>();
    /// ds.insert(&10,5);
    /// ds.insert_link(&9,&vec![],0);
    /// ds.append_links(&9,&10);
    /// let iter = ds.iter_links(&9).ok().unwrap();
    /// 
    /// for (i,j) in iter{
    ///     assert!(*j==5);
    /// }
    /// ```
    pub fn append_links(&mut self,key:&KeyType,
        key_append:&KeyType)->Result<(),DBOperationError>{
        let data = self.get_node(key)?.clone();
        let link_vec_opt = data.item.a();
        if link_vec_opt.is_some(){
            let link = link_vec_opt.unwrap();
            let mut link_vec = link.children.clone();
            if !link_vec.contains(key_append){
                link_vec.push(key_append.clone());
                return self.overwrite_link(key,&link_vec,link.type_label.clone());
            }else{
                return Err(DBOperationError::KeyAllreadyPresent);
            }
        }else{
            return Err(DBOperationError::NodeNotLink);
        }

    }
    pub fn right_join(&self,right: &DataStructure<KeyType,DataType,LinkLabel>)->
        Result<DataStructure<KeyType,DataType,LinkLabel>,DBOperationError>
    {
        return right_join(self,right);
    }
    pub fn to_string(&self)->Result<std::string::String,SerializeError>
        where
            KeyType:Serialize,
            DataType:Serialize,
            LinkLabel:Serialize
    {
        let res = serde_json::to_string(&self);
        if res.is_ok(){
            return Ok(res.ok().unwrap());
        }else{
            match res.err().unwrap(){
                _ =>return Err(SerializeError::Unknown),
            }
        }
    }
    /// Gets number of elements in db
    /// ```
    /// let mut ds = gulkana::new_datastructure::<u32,u32,u32>();
    /// assert!(ds.len()==0);
    /// ds.insert(&20,20);
    /// assert!(ds.len()==1);
    /// ```
    pub fn len(&self)->usize{
        return self.tree.len()
    }
}
impl<K: std::cmp::Ord+std::fmt::Display+std::clone::Clone+Serialize,DataType:std::clone::Clone,
    I:std::clone::Clone+Serialize> fmt::Display for DataStructure<K,DataType,I>{
    fn fmt(&self, f: &mut fmt::Formatter)-> fmt::Result 
    {
        write!(f,"\n")?;
        for row in self.iter(){
            write!(f,"\tkey: {}\n",row.0)?;
        }
        return write!(f,"");

    }
}
pub enum ReadError{
    ParseError
}
/// Reads Database from a string. Can be used to write to a file
pub fn from_string<'a,K:std::cmp::PartialEq+std::cmp::Ord+std::clone::Clone+Serialize,DataType:std::clone::Clone+Serialize,LinkLabel:std::clone::Clone+Serialize>(data_in:&'a std::string::String)->Result<DataStructure<K,DataType,LinkLabel>,ReadError>
where
    K:Deserialize<'a>,
    DataType:Deserialize<'a>,
    LinkLabel:Deserialize<'a>,

{

        let res = serde_json::from_str(data_in);
        if res.is_ok(){
            return Ok(res.ok().unwrap());
        }else{
            return match res.err().unwrap(){
                _ => Err(ReadError::ParseError),
            }
        }
        
        
    }
pub fn right_join<K:std::cmp::Ord+std::clone::Clone+Serialize,DataType:std::clone::Clone,LinkLabel:std::clone::Clone+Serialize>(left:&DataStructure<K,DataType,LinkLabel>,
        right:&DataStructure<K,DataType,LinkLabel>)->Result<DataStructure<K,DataType,LinkLabel>,DBOperationError>
    {

    let mut left_iter = left.iter().peekable();
    let mut right_iter = right.iter().peekable();
    let mut db = new_datastructure::<K,DataType,LinkLabel>();


    loop{
        let left_opt = left_iter.peek();
        let right_opt = right_iter.peek();
        if left_opt.is_none(){
            return Ok(db);            
        }else{
            if right_opt.is_none(){
                db.insert_node(left_opt.unwrap().0,left_opt.unwrap().1.clone())?;
                left_iter.next();
            }else{
                let left_data = left_opt.unwrap();
                let right_data = right_opt.unwrap();
                let left_key = left_data.0;
                let right_key=right_data.0;
                //if keys are the same
                if left_key==right_key{
                    db.insert_node(left_key,left_data.1.clone())?;
                    left_iter.next();
                    right_iter.next();
                }else{
                    if left_key>right_key{
                        right_iter.next();
                    }else{
                        db.insert_node(left_key,left_data.1.clone())?;
                        left_iter.next();
                    }

                }
            }
        }
    }

}
pub fn new_datastructure<K:std::cmp::PartialEq+std::clone::Clone+std::cmp::Ord+Serialize,DataType:std::clone::Clone,LinkLabel:std::clone::Clone+Serialize>()->DataStructure<K,DataType,LinkLabel>
    {
    return DataStructure{
        tree:BTreeMap::new(),
    }
}


#[cfg(test)]
mod tests{
    use super::*;
    type Label = u32;

    #[test]
    #[allow(unused_must_use)]
    fn test_insert(){
        let mut arr:Vec<u32> =Vec::new();
        arr.push(2);
        arr.push(3);
        arr.push(4);
        for _i in 1..10000{
            arr.push(prelude::random());
        }

        let mut ds = new_datastructure::<u32,u32,Label>();
        for i in &arr{
            ds.insert(i,*i);
        }
        let mut test_arr:Vec<u32>=Vec::new();
        for (_key,data) in ds.iter(){
            test_arr.push(*data.item.b().unwrap());
        }
        arr.sort();
        test_arr.sort();
        for i in 0..test_arr.len(){
            //println!("arr[{}]: {} test_arr[{}]: {}]",i,arr[i],i,test_arr[i]);
            assert!(arr[i]==test_arr[i]);
        }
    }
    #[test]
    #[allow(unused_must_use)]
    fn test_right_join(){
        let mut dsr=new_datastructure::<u32,u32,Label>();
        dsr.insert(&0,0);
        dsr.insert(&1,1);
        dsr.insert(&2,2);
        let mut dsl=new_datastructure::<u32,u32,Label>();
        dsl.insert(&0,0);
        dsl.insert(&1,1);
        dsl.insert(&2,2);
        println!("inserted");
        println!("right ds: {}",dsr);
        println!("left ds: {}",dsl);
        let mut join = right_join(&dsr,&dsl).ok().unwrap();
        println!("did first join");
        let mut vec_out:Vec<u32>=Vec::new();
        for i in join.iter(){
            vec_out.push(*i.1.item.b().unwrap());
        }
        vec_out.sort();
        for i in 0..vec_out.len(){
            assert!(vec_out[i]==i as u32);
        }
        //Testing with extra item in left
        dsl.insert(&7,7);
        join = right_join(&dsr,&dsl).ok().unwrap();
        vec_out.clear();
        for i in join.iter(){
            vec_out.push(*i.1.item.b().unwrap());
        }
        vec_out.sort();
        for i in 0..vec_out.len(){
            assert!(vec_out[i]==i as u32);
        }

        //testing with extra item in right
        dsr.insert(&3,3);
        dsr.insert(&4,4);
        join = right_join(&dsr,&dsl).ok().unwrap();
        vec_out.clear();
        for i in join.iter(){
            vec_out.push(*i.1.item.b().unwrap());
        }
        vec_out.sort();
        for i in 0..vec_out.len(){
            assert!(vec_out[i]==i as u32);
        }
    }
    #[test]
    #[allow(unused_must_use)]
    fn test_eq(){
        let mut dsr=new_datastructure::<u32,u32,Label>();
        dsr.insert(&0,0);
        dsr.insert(&1,1);
        dsr.insert(&2,2);
        let mut dsl=new_datastructure::<u32,u32,Label>();
        dsl.insert(&0,0);
        dsl.insert(&1,1);
        dsl.insert(&2,2);
        assert!(dsr==dsl);
        dsl.insert(&3,3);
        assert!(dsr != dsl);


        
    }
    #[test]
    #[allow(unused_must_use)]
    fn test_serialize(){
        let mut dsr=new_datastructure::<u32,u32,Label>();
        dsr.insert(&0,0);
        dsr.insert(&1,1);
        dsr.insert(&2,2);
        let str_ds = dsr.to_string();
        let dsl:DataStructure<u32,u32,Label> = from_string(&str_ds.ok().unwrap()).ok().unwrap();
        assert!(dsr==dsl);


    }
    #[test]
    #[allow(unused_must_use)]
    fn test_links(){
        let mut dsr=new_datastructure::<u32,u32,Label>();
        dsr.insert(&0,0);
        dsr.insert(&1,1);
        dsr.insert(&2,2);
        dsr.insert_link(&4,&vec![0,1],0);
        let foo:std::vec::Vec<u32> = vec![0,1];
        assert!(*dsr.get_links(&4).ok().unwrap()==(foo));           

    }
    #[test]
    #[allow(unused_must_use)]
    fn test_iter_link(){
        let mut ds = new_datastructure::<u32,u32,Label>();
        ds.insert(&10,5);
        ds.insert_link(&9,&vec![10],0);
        let iter = ds.iter_links(&9).ok().unwrap();
        for (_i,j) in iter{
            assert!(*j==5);
        }
    }
    #[test]
    #[allow(unused_must_use)]
    fn test_iter_data(){
        let mut ds = new_datastructure::<u32,u32,Label>();
        ds.insert(&10,5);
        for (_key,data) in ds.iter_data(){
            assert!(*data==5);
        }
        return ();
            
    }
    #[test]
    #[allow(unused_must_use)]
    fn test_set_data(){
        let mut ds = new_datastructure::<u32,u32,Label>();
        ds.insert(&10,5);
        ds.set_data(&10,&10);
        for (_key,data) in ds.iter_data(){
            assert!(*data==10);
        }
        return ();
            
    }


}
