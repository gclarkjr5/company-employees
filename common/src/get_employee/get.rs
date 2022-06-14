use super::super::common::{Company};
use tokio::io::{Error, ErrorKind};


#[cfg(test)]
#[path="test_get.rs"]
mod test_get;

type ErrorGen = Box<dyn std::error::Error + Send + Sync>;

impl Company {

    /// retrieves employees either for each department in the company, or for only a particular
    /// department

    pub async fn get_employees(
        &self,
        all_bool: &bool,
        department: &Option<String>
    ) -> Result<Company, ErrorGen> {

        if self.employee_list.is_empty() {
            let msg = "No employees have been added to the company yet.".to_string();
            let error_string = Error::new(ErrorKind::Other, msg);
            return Err(Box::new(error_string))
        }

        if *all_bool {

            let mut company = Company::new().await?;

            self.employee_list
                .iter()
                .for_each(|(d, employees)| {
                    let mut e = employees.to_owned();
                    e.sort();
                    company.employee_list.insert(d.to_owned(), e);
                });

            return Ok(
                company
            )

        } else {

            let dept = department.to_owned().expect("no value provided for department");

            match self.employee_list.contains_key(&dept) {
                true => {

                    let mut filtered_company = Company::new().await?;

                    self.employee_list
                        .iter()
                        .filter(|(d, _)| **d == dept)
                        .for_each(|(d, employees)| {
                            let mut e = employees.to_owned();
                            e.sort();
                            filtered_company.employee_list.insert(d.to_owned(), e);
                        });

                    return Ok(
                        filtered_company
                    )
                },
                false => {
                    let msg = format!("The {dept} department doesn't exist");
                    let error_string = Error::new(ErrorKind::Other, msg);
                    return Err(Box::new(error_string))
                }
            }
        }
    }
}