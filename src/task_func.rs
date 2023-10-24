pub mod task_func {
    #[derive(Debug)]
    pub struct task{
        id: i32,
        desc:String,
        status:String
    }
    impl task {
        pub fn new(id:i32,desc:String) -> Self {
            Self{
                id,
                desc,
                status:"incomplete".to_string()
            }
        }
    }
    pub fn print_task(list: &Vec<task>){
        for i in list{
            println!("{:?}",i);
        }
    }
    pub fn complete(list: &mut Vec<task>, id: i32) {
        for (index, task) in list.iter_mut().enumerate() {
            if task.id == id {
                task.status = "complete".to_string();
                break; 
            }
        }
    }

    pub fn delete(list: &mut Vec<task>, id: i32) {
        let mut item_index = None;
        for (index, task) in list.iter().enumerate() {
            if task.id == id {
                item_index = Some(index);
                list.remove(index);
                break; 
            }
        }
    }
}