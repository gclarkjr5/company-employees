use std::collections::HashMap;
use super::super::common::{Company};

#[cfg(test)]
#[path="test_get.rs"]
mod test_get;

impl Company {

    /// retrieves employees either for each department in the company, or for only a particular
    /// department
    pub fn get_employees(
        &mut self,
        department: String
    ) -> Result<HashMap<String, Vec<String>>, &'static str> {
    
        if self.employee_list.is_empty() {
            return Err("No employees have been added to the company yet.")
        }
    
    
        match department.as_str() {
            "all" => {

                for (_k, v) in self.employee_list.iter_mut() {
                    v.sort();
                }

                Ok(self.employee_list.to_owned())
            },
            _ => match self.employee_list.contains_key(&department) {
                true => Ok(
                    get_dept_employees(self, department)
                ),
                false => Err("Department doesnt exist")
            }
        }
    }

}

/// helper function to get employees of a particular department
fn get_dept_employees(
    company: &mut Company,
    department: String
) -> HashMap<String, Vec<String>> {
    
    let mut dept_employees: Vec<String> = company.employee_list
        .iter()
        .filter_map(|(k,v)| if *k == department {Some(v.to_owned())} else {None})
        .flatten()
        .collect();

    dept_employees.sort();


    let vec_dept = vec![(department, dept_employees)];

    HashMap::from_iter(vec_dept)
}