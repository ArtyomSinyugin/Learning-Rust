pub enum Mass {
    Module(f32),
    Fuel(f32),
}

impl Mass {

    pub fn unwrap (&self) -> &f32 {
        match self {
            Mass::Module(f32)=> f32,
            Mass::Fuel(f32)=> f32,
        }
    }

    pub fn new_module (mass: Option<String>)-> Mass {
        Mass::Module(mass.unwrap().trim().parse::<f32>().unwrap())
    } 

    pub fn fuel_to_go_1 (self) -> Mass {        
        let delta = *&self.unwrap() % 3.0;
        Mass::Fuel((self.unwrap() - delta) / 3.0 - 2.0)       
    }

    pub fn fuel_to_go_2 (self) -> Mass {
        let mut vec: Vec<f32> = Vec::new();
        let mut i = *self.unwrap();
                
           // print!("Дано: {} ", self.unwrap()); 
           while i >= 6.0 {

            let delta = i % 3.0;
            let delta = (i - delta) / 3.0 - 2.0;  
            i = delta;
            vec.push(delta);    
            }   

            // println!("{:?}", vec);
            Mass::Fuel(vec.iter().sum())
    }
}