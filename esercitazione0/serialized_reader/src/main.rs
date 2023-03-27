use std::io::Read;
use std::fs::File;


#[derive(Clone,Copy)]
struct ValueStruct{
    _type:i32,
    val:f32,
    timestamp:i64
}
#[derive(Clone,Copy)]
struct MValueStruct{
    _type:i32,
    val:[f32;10],
    timestamp:i64
}
#[derive(Clone,Copy)]
struct MessageStruct{
    _type:i32,
    message:[u8;20]// stringa null terminated lung max 20
}

enum ContentData{
    Val(ValueStruct),
    Mvals(MValueStruct),
    Messages(MessageStruct)
}

struct ImportData{
    _type:i32,
    u:ContentData
}

impl ImportData{

    pub fn from_file(f:&mut File)->Vec<ImportData>{
        let mut buffer=Vec::new();
        match f.read_to_end(&mut buffer){
            Ok(_)=>{},
            Err(e)=>panic!("Error reading file:{}",e)
        }

        let mut i=0;
        let mut struct_buf=Vec::<u8>::new();
        let mut data_array=Vec::<ImportData>::new();

        for val in buffer{
            if (i==0) || (i==64) {
                i=0;
                struct_buf=Vec::<u8>::new();
                struct_buf.push(val);
            }else{ 
                struct_buf.push(val);
                if i==63{//convert to struct

                        let mut type_array:[u8;4]=[0,0,0,0];
                        for j in 0..4{
                            type_array[j]=struct_buf[j];
                        }
                        match type_array{
                            [1,0,0,0]=>{

                                let mut val_array:[u8;4]=[0,0,0,0];
                                for j in 12..16{
                                    val_array[j-12]=struct_buf[j];
                                }

                                let mut timestamp_array:[u8;8]=[0,0,0,0,0,0,0,0];
                                for j in 16..24{
                                    timestamp_array[j-16]=struct_buf[j];
                                }

                                let value_data=ValueStruct{
                                    _type:1,
                                    val:f32::from_le_bytes(val_array),
                                    timestamp:i64::from_le_bytes(timestamp_array)
                                };
                                
                                let import_record=ImportData{
                                    _type:1,
                                    u:ContentData::Val(value_data)
                                };
                                data_array.push(import_record);
                            },
                            [2,0,0,0]=>{
                                
                                let mut val_array:[u8;40]=[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0];
                                for j in 12..52{
                                    val_array[j-12]=struct_buf[j];
                                }
                                let mut f32_val_array:[f32;10]=[0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0];
                                for j in 0..10{
                                    let mut t_f32_array:[u8;4]=[0,0,0,0];
                                    for k in 0..4{
                                        t_f32_array[k]=val_array[(j*4)+k];
                                    }
                                    f32_val_array[j]=f32::from_le_bytes(t_f32_array);
                                }

                                let mut timestamp_array=[0,0,0,0,0,0,0,0];
                                for j in 56..64{
                                    timestamp_array[j-56]=struct_buf[j];
                                }
                                
                                let mvalue_data=MValueStruct {
                                    _type: 2, 
                                    val: f32_val_array,
                                    timestamp: i64::from_le_bytes(timestamp_array)
                                };

                                let import_record=ImportData{
                                    _type:2,
                                    u:ContentData::Mvals(mvalue_data)
                                };
                                data_array.push(import_record);
                            },
                            [3,0,0,0]=>{
                                let mut message_array=[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0];
                                for j in 12..32{
                                    message_array[j-12]=struct_buf[j];
                                }
                                
                                let message_data=MessageStruct {
                                    _type: 3, 
                                    message: message_array
                                };

                                let import_record=ImportData{
                                    _type:3,
                                    u:ContentData::Messages(message_data)
                                };
                                data_array.push(import_record);
                            }
                            _=>{
                                println!("not matched");
                            }
                        }
                        {
                            println!("type must be 1/2/3, find:{}",data_array.last().unwrap()._type);
                            match data_array.last().unwrap().u{
                                ContentData::Val(value_struct)=>{
                                    println!("type must be 1, find:{}",value_struct._type);
                                    println!("val must be 1.0, find:{}",value_struct.val);
                                    println!("timestamp must be 50000, find:{}",value_struct.timestamp);
                                },
                                ContentData::Mvals(mvalue_struct)=>{
                                    println!("type must be 2, find:{}",mvalue_struct._type);
                                    println!("val must be 1.0x10, find:{:?}",mvalue_struct.val);
                                    println!("timestamp must be 30000, find:{}",mvalue_struct.timestamp);
                                },
                                ContentData::Messages(message_struct)=>{
                                    println!("type must be 3, find:{}",message_struct._type);
                                    println!("val must be config, find:{:?}",message_struct.message);
                                }
                            }
                        }
                    }
                }
                i+=1;
            }
            return data_array;
        }
    }



fn main() {
    let mut f=match File::open("../data"){
        Ok(f) => f,
        Err(e) => panic!("Error opening file: {}",e)
    };
    let mut _data:Vec<ImportData>=ImportData::from_file(&mut f);
}
