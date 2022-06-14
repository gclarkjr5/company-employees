use super::super::common::{Company};
use tokio::io::{Error, ErrorKind};

#[cfg(test)]
#[path="test_add.rs"]
mod test_add;

type ErrorGen = Box<dyn std::error::Error + Send + Sync>;

impl Company {

    /// adds an employee+department combo for the company if it doesn't already exist

    pub async fn add_employee(
        &mut self,
        employee_name: &String,
        employee_dept: &String
    ) -> Result<&Company, ErrorGen> {
        
        // check if employee exists
        let department_employees = self.employee_list.get_mut(employee_dept);
        
        match department_employees {
            Some(x) => match x.contains(&employee_name) {
                true => {
                    let msg = format!("The employee {} already exists for the {} department", employee_name, employee_dept);
                    let error_string = Error::new(ErrorKind::Other, msg);
                    Err(Box::new(error_string))
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