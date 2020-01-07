use std::collections::BTreeMap;
use std::fmt;
use rand::prelude::*;
use std::str::FromStr;
use std::num::ParseIntError;
use serde::{Serialize, Deserialize};
mod opt_pair;
use opt_pair::{OptStruct,new_optstruct_a,new_optstruct_b};
/// # Gulkana
/// Gulkana is a lightweight key based database for string files.
/// The main struct is DataStructure
#[derive(PartialEq,Eq,Copy, Clone,Serialize,Deserialize)]
pub struct MetaData{

}
impl FromStr for MetaData{
    type Err = ParseIntError;
     fn from_str(s: &str) -> Result<Self, Self::Err> {
        return Ok(MetaData{});
     }
}

#[derive(PartialEq,Eq,Clone,Serialize,Deserialize)]
pub struct Link<Key:std::clone::Clone>{
    children: Vec<Key>,
}
fn new_metadata()->MetaData{
    return MetaData{};
}
#[derive(PartialEq,Eq,Clone,Serialize,Deserialize)]
pub struct Node<Key:std::cmp::PartialEq+std::clone::Clone,Item:std::clone::Clone>
{
    metadata:MetaData,
    item:OptStruct<Link<Key>,Item>

}
fn new_node<K:std::cmp::PartialEq+std::clone::Clone,
   I:std::clone::Clone>(input:I)->Node<K,I>
    where
        K:std::clone::Clone,
        I:std::clone::Clone,
    {
    let foo =  Node{
        metadata:new_metadata(),
        item:new_optstruct_b(input),
    };
    return foo;
}
fn new_node_link<K:std::cmp::PartialEq+std::clone::Clone,
   I:std::clone::Clone>(input:&std::vec::Vec<K>)->Node<K,I>
    where
        K:std::clone::Clone,
        I:std::clone::Clone,
    {
    let foo =  Node{
        metadata:new_metadata(),
        item:new_optstruct_a(
            Link{ 
                children:input.clone(),
            }),
    };
    return foo;
}
/// Struct usd to store data
#[derive(PartialEq,Eq,Deserialize,Serialize)]
pub struct DataStructure<KeyType:std::cmp::Ord+std::clone::Clone,
    ItemData:std::clone::Clone>{
    tree:BTreeMap<KeyType,Node<KeyType,ItemData>>,
    
}
impl<KeyType:std::cmp::Ord+std::clone::Clone,
    ItemData:std::clone::Clone > DataStructure<KeyType,ItemData>{
    /// Inserts data into datastructure
    /// ```
    /// let mut ds = gulkana::new_datastructure::<u32,u32>();
    /// ds.insert(&10,5);
    /// ```
    pub fn insert(&mut self,key:&KeyType,data:ItemData)->Result<String,String>
    {
        return self.insert_node(key,new_node(data)); 
    }
    pub fn insertLink(&mut self,key:&KeyType,children:&std::vec::Vec<KeyType>)->Result<String,String>{
        return self.insert_node(key,new_node_link(children));
        
    }
    pub fn overwriteLink(&mut self,key:&KeyType,children:&std::vec::Vec<KeyType>)->Result<String,String>{
        return self.overwriteNode(key,new_node_link(children));
    }
    fn insert_node(&mut self,key:&KeyType,data:Node<KeyType,ItemData>)->Result<String,String>
        {
        if self.tree.contains_key(key)==false{
            self.tree.insert(key.clone(),data);
            return Ok("".to_string());
        }else{
            return Err("key already present".to_string());
        }
    

    }
    fn overwriteNode(&mut self,key:&KeyType,
        data:Node<KeyType,ItemData>)->Result<String,String>{
            self.tree.insert(key.clone(),data);
            return Ok("".to_string());

    }
    pub fn iter(&self)->
        std::collections::btree_map::Iter<'_, KeyType, Node<KeyType,ItemData>>{
        self.tree.iter()
    }
    /// gets key from database
    /// ```
    ///
    /// let mut ds = gulkana::new_datastructure::<u32,u32>();
    /// ds.insert(&10,5);
    /// let data = ds.get(10);
    /// assert!(data.unwrap()==5); 
    /// ```
    pub fn get(&self,key:KeyType)->Option<ItemData>
        where
            KeyType : std::cmp::Ord,
    {
        let temp = self.tree.get(&key);
        if temp.is_none(){

            return None;
        }else{
            return temp.unwrap().item.B();
        }
    }
    fn getNode(&self,key:&KeyType)->Option<&Node<KeyType,ItemData>>{
        return self.tree.get(key);
    }
    pub fn getLinks(&self,key:&KeyType)->Option<Vec<KeyType>>{
        let data = self.getNode(key);
        if data.is_some(){
        
            let vec_temp = data.unwrap().item.A();
        
            if vec_temp.is_some(){
                return Some(vec_temp.unwrap().children);
            }else{
                return None;
            }
        }else{
            return None;
        }
        return None;
    }
    pub fn contains(&self,key:&KeyType)->bool{
        return self.getNode(key).is_some();
    }
    pub fn appendLinks(&mut self,key:&KeyType,
        key_append:&KeyType)->Result<String,String>{
        let data = self.getNode(key);
        if data.is_some(){
            let mut link_vec_opt = data.unwrap().item.A();
            if link_vec_opt.is_some(){
                let mut link_vec = link_vec_opt.unwrap().children;
                if !link_vec.contains(key_append){
                    link_vec.push(key_append.clone());
                    return self.overwriteLink(key,&link_vec);
                }else{
                    return Ok("".to_string());
                }
            }else{
                return Err("key not a link".to_string());
            }
        }else{
            return Err("key not found".to_string());
        }

    }
    pub fn rightJoin(&self,right: &DataStructure<KeyType,ItemData>)->DataStructure<KeyType,ItemData>
    {
        return right_join(self,right);
    }
}
impl<K: std::cmp::Ord+std::fmt::Display+std::clone::Clone,
    I:std::clone::Clone> fmt::Display for DataStructure<K,I>{
    fn fmt(&self, f: &mut fmt::Formatter)-> fmt::Result 
    {
        write!(f,"\n");
        for row in self.iter(){
            write!(f,"\tkey: {}\n",row.0);
        }
        write!(f,"")

    }
}
pub fn right_join<K:std::cmp::Ord+std::clone::Clone,ItemData>(left:&DataStructure<K,ItemData>,
        right:&DataStructure<K,ItemData>)->DataStructure<K,ItemData>
    where
        ItemData:std::clone::Clone,
    {

    let mut left_iter = left.iter().peekable();
    let mut right_iter = right.iter().peekable();
    let mut db = new_datastructure::<K,ItemData>();


    loop{
        let left_opt = left_iter.peek();
        let right_opt = right_iter.peek();
        if left_opt.is_none(){
            return db;            
        }else{
            if right_opt.is_none(){
                db.insert_node(left_opt.unwrap().0,left_opt.unwrap().1.clone());
                left_iter.next();
            }else{
                let left_data = left_opt.unwrap();
                let right_data = right_opt.unwrap();
                let left_key = left_data.0;
                let right_key=right_data.0;
                //if keys are the same
                if left_key==right_key{
                    db.insert_node(left_key,left_data.1.clone());
                    left_iter.next();
                    right_iter.next();
                }else{
                    if left_key>right_key{
                        right_iter.next();
                    }else{
                        db.insert_node(left_key,left_data.1.clone());
                        left_iter.next();
                    }

                }
            }
        }
    }

}
pub fn new_datastructure<K:std::cmp::PartialEq+std::clone::Clone,DataType:std::clone::Clone>()->DataStructure<K,DataType>
    where
        K:std::cmp::Ord,
    {
    return DataStructure{
        tree:BTreeMap::new(),
    }
}


#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_insert(){
        let mut arr:Vec<u32> =Vec::new();
        arr.push(2);
        arr.push(3);
        arr.push(4);
        for i in 1..10000{
            arr.push(random());
        }

        let mut ds = new_datastructure::<u32,u32>();
        for i in &arr{
            
            ds.insert(i,*i);
        }
        let mut test_arr:Vec<u32>=Vec::new();
        for (key,data) in ds.iter(){
            test_arr.push(data.item.B().unwrap());
        }
        arr.sort();
        test_arr.sort();
        for i in 0..test_arr.len(){
            assert!(arr[i]==test_arr[i]);
        }
    }
    #[test]
    fn test_right_join(){
        let mut dsr=new_datastructure::<u32,u32>();
        dsr.insert(&0,0);
        dsr.insert(&1,1);
        dsr.insert(&2,2);
        let mut dsl=new_datastructure::<u32,u32>();
        dsl.insert(&0,0);
        dsl.insert(&1,1);
        dsl.insert(&2,2);
        println!("inserted");
        println!("right ds: {}",dsr);
        println!("left ds: {}",dsl);
        let mut join = right_join(&dsr,&dsl);
        println!("did first join");
        let mut vec_out:Vec<u32>=Vec::new();
        for i in join.iter(){
            vec_out.push(i.1.item.B().unwrap());
        }
        vec_out.sort();
        for i in 0..vec_out.len(){
            assert!(vec_out[i]==i as u32);
        }
        //Testing with extra item in left
        dsl.insert(&7,7);
        join = right_join(&dsr,&dsl);
        vec_out.clear();
        for i in join.iter(){
            vec_out.push(i.1.item.B().unwrap());
        }
        vec_out.sort();
        for i in 0..vec_out.len(){
            assert!(vec_out[i]==i as u32);
        }

        //testing with extra item in right
        dsr.insert(&3,3);
        dsr.insert(&4,4);
        join = right_join(&dsr,&dsl);
        vec_out.clear();
        for i in join.iter(){
            vec_out.push(i.1.item.B().unwrap());
        }
        vec_out.sort();
        for i in 0..vec_out.len(){
            assert!(vec_out[i]==i as u32);
        }
    }
    #[test]
    fn test_eq(){
        let mut dsr=new_datastructure::<u32,u32>();
        dsr.insert(&0,0);
        dsr.insert(&1,1);
        dsr.insert(&2,2);
        let mut dsl=new_datastructure::<u32,u32>();
        dsl.insert(&0,0);
        dsl.insert(&1,1);
        dsl.insert(&2,2);
        assert!(dsr==dsl);
        dsl.insert(&3,3);
        assert!(dsr != dsl);


        
    }
    #[test]
    fn test_serialize(){
        let mut dsr=new_datastructure::<u32,u32>();
        dsr.insert(&0,0);
        dsr.insert(&1,1);
        dsr.insert(&2,2);
        let str_ds = serde_json::to_string(&dsr).unwrap();
        let mut dsl_t:DataStructure<u32,u32> = serde_json::from_str(&str_ds).unwrap();


    }
    #[test]
    fn test_links(){
        let mut dsr=new_datastructure::<u32,u32>();
        dsr.insert(&0,0);
        dsr.insert(&1,1);
        dsr.insert(&2,2);
        dsr.insertLink(&4,&vec![0,1]);
        let foo:std::vec::Vec<u32> = vec![0,1];
        assert!(dsr.getLinks(&4)==Some(foo));           

    }

}
