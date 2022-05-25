use super::super::common::{Company};
use itertools::Itertools;

#[cfg(test)]
#[path="test_get.rs"]
mod test_get;

impl Company {

    /// retrieves employees either for each department in the company, or for only a particular
    /// department
    /// 
    /// Example
    /// 
    /// ```
    /// use company_employees::common::Company;
    /// 
    /// let mut company = Company::new();
    /// 
    /// 
    /// company.employee_list.insert("sales".to_string(), vec!["employee".to_string()]);
    /// 
    /// let dept_employees = company.get_employees(&false, &Some("sales".to_string())).unwrap(); 
    /// 
    /// assert_eq!(
    ///     dept_employees.employee_list.get_key_value(&"sales".to_string()),
    ///     Some((&"sales".to_string() ,&vec!["employee".to_string()]))
    /// )
    /// ```
    pub fn get_employees(
        &mut self,
        all_bool: &bool,
        department: &Option<String>
    ) -> Result<Company, String> {

        if self.employee_list.is_empty() {
            let msg = "No employees have been added to the company yet.".to_string();

            return Err(msg)
        }

        if *all_bool {

            let mut sorted_company = Company::new();

            self.employee_list
                .iter_mut()
                .sorted()
                .for_each(|(d, employees)| {
                    employees.sort();
                    sorted_company.employee_list.insert(d.to_owned(), employees.to_owned());
                });

            return Ok(
                sorted_company
            )

        } else {

            let dept = department.to_owned().unwrap();

            match self.employee_list.contains_key(&dept) {
                true => {

                    let mut filtered_company = Company::new();

                    self.employee_list
                        .iter_mut()
                        .filter(|(d, _)| **d == dept)
                        .for_each(|(d, employees)| {
                            employees.sort();
                            filtered_company.employee_list.insert(d.to_owned(), employees.to_owned());
                        });

                    return Ok(
                        filtered_company
                    )
                },
                false => {
                    let msg = format!("The {dept} department doesn't exist");
                    return Err(msg)
                }
            }
        }
    }
}