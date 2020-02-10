use crate::token::DataType;
use std::collections::HashMap;
#[derive(Clone,Debug)]
pub struct VarData{
    value: Option<DataType>,
    constant: bool,
    delete: bool
}
#[derive(Clone,Debug)]
pub struct Scope{
    pub mem: HashMap<String, VarData>
}
impl Scope{
    pub fn new() -> Self{
        Scope{mem: HashMap::new()}
    }
    pub fn add_var(&mut self,name: String){
        match self.mem.insert(name,VarData{
            value: None,
            constant: false,
            delete: false
        }){
            Some(VarData{value: _, constant: true, delete: _}) => panic!("Cant assign to a constant"),
            _ => true
        };
    }
    pub fn set_var(&mut self,name: String,expr: DataType){
        if let Some(ref mut record) = self.mem.get_mut(&name){
            record.value = Some(expr);
        }
    }
    pub fn get_var(self, name: String) -> DataType{
        match self.mem.get(&name){
            Some(x) => x
                .value
                .as_ref()
                .expect("Couldnt get ref")
                .clone(),
            None => {
                println!("{:#?}",self.mem);
                DataType::Error(format!("{} hasnt been initialized or declared",name))
            }
        }
    }
}
