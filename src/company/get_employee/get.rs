use std::collections::HashMap;
use super::super::super::{Company};

#[cfg(test)]
#[path="test_get.rs"]
mod test_get;

impl Company {
    pub fn get_employees(
        &mut self,
        department: String
    ) -> Result<HashMap<String, Vec<String>>, &'static str> {
    
        if self.employee_list.is_empty() {
            return Err("No employees have been added to the company yet.")
        }
    
    
        match department.as_str() {
            "all" => Ok(
                self.employee_list.to_owned()
            ),
            _ => match self.employee_list.contains_key(&department) {
                true => Ok(
                    get_dept_employees(self, department)
                ),
                false => Err("Department doesnt exist")
            }
        }
    }

}


fn get_dept_employees(
    company: &mut Company,
    department: String
) -> HashMap<String, Vec<String>> {
    
    let dept_employees: Vec<String> = company.employee_list
        .iter()
        .filter_map(|(k,v)| if *k == department {Some(v.to_owned())} else {None})
        .flatten()
        .collect();


    let vec_dept = vec![(department, dept_employees)];

    HashMap::from_iter(vec_dept)
}