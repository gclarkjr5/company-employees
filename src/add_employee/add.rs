use super::super::common::{Company};

#[cfg(test)]
#[path="test_add.rs"]
mod test_add;

impl Company {

    /// adds an employee+department combo for the company if it doesn't already exist
    /// 
    /// # Example
    /// 
    /// ```
    /// use company_employees::common::Company;
    /// 
    /// let mut company = Company::new();
    /// 
    /// let name = "employee".to_string();
    /// let department = "sales".to_string();
    /// 
    /// company.add_employee(&name, &department);
    /// 
    /// assert_eq!(
    ///     company.employee_list.get_key_value(&department),
    ///     Some((&department, &vec![name]))
    /// )
    /// ```
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