use super::super::super::{Company};

#[cfg(test)]
#[path="test_add.rs"]
mod test_add;

impl Company {
    pub fn add_employee(
        &mut self,
        employee_name: &String,
        employee_dept: &String
    ) -> Result<&mut Company, &'static str> {
        
        // check if employee exists
        let department_employees = self.employee_list.get_mut(employee_dept);
        
    
        match department_employees {
            Some(x) => match x.contains(&employee_name) {
                true => {
                    Err("The employee {} already exists for the {} department")
                },
                false => {
                    x.push(employee_name.to_owned());
                    x.sort();
                    Ok(self)
                }
            },
            None => {
                self.employee_list.insert(employee_dept.to_owned(), vec![employee_name.to_owned()]);
                Ok(self)
            }
        }
    }
}