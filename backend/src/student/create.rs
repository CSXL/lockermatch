use crate::http::Error;
use chrono::{DateTime, Datelike, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A validated student identifier for California High School.
///
/// The student ID must be exactly 6 digits (e.g., "123456").
/// This is enforced at creation time and guarantees all StudentId instances are valid.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StudentId(String);

impl StudentId {
  /// Creates a new StudentId from a string.
  ///
  /// # Format Requirements
  /// - Must be exactly 6 characters long
  /// - Must contain only digits (0-9)
  ///
  /// # Examples
  /// ```
  /// use backend::student::StudentId;
  ///
  /// let valid_id = StudentId::new("123456".to_string());
  /// assert!(valid_id.is_ok());
  ///
  /// let invalid_id = StudentId::new("12345".to_string()); // Returns Error
  /// assert!(invalid_id.is_err());
  /// ```
  ///
  /// # Errors
  /// Returns `Error::UnprocessableEntity` if the ID doesn't meet format requirements.
  pub fn new(id: String) -> Result<Self, Error> {
    if id.len() == 6 && id.chars().all(|c| c.is_digit(10)) {
      Ok(StudentId(id))
    } else {
      Err(Error::UnprocessableEntity {
        errors: HashMap::from([("id".into(), vec!["must be 6 digits".into()])]),
      })
    }
  }

  /// Returns the student ID as a String.
  ///
  /// # Examples
  /// ```
  /// use backend::student::StudentId;
  ///
  /// let id = StudentId::new("123456".to_string()).unwrap();
  /// assert_eq!(id.to_string(), "123456");
  /// ```
  pub fn to_string(&self) -> String {
    self.0.clone()
  }

  /// Creates a StudentId from a string. Alias for `new()`.
  ///
  /// # Errors
  /// Returns `Error::UnprocessableEntity` if the ID doesn't meet format requirements.
  pub fn from_string(id: String) -> Result<Self, Error> {
    return Self::new(id);
  }
}

/// Represents a high school student for locker assignment purposes.
///
/// This struct contains all the information needed to assign lockers to students
/// at California High School, including accommodation requirements.
///
/// # Field Specifications
///
/// ## Required Fields
/// - `id`: 6-digit student identifier (validated)
/// - `first_name`: Student's first name (1-50 characters, trimmed)
/// - `last_name`: Student's last name (1-50 characters, trimmed)
/// - `email`: Valid email address (5-254 characters, normalized to lowercase)
/// - `grade`: High school grade level (9=Freshman, 10=Sophomore, 11=Junior, 12=Senior)
/// - `graduation_year`: Expected graduation year (current year to current year + 10)
///
/// ## Optional Fields
/// - `special_accommodations`: Accessibility needs for locker assignment (max 500 characters)
///
/// ## Timestamps
/// - `created_at`: UTC timestamp when the student record was created
/// - `updated_at`: UTC timestamp when the student record was last modified
///
/// # Examples
/// ```
/// use backend::student::Student;
///
/// let student = Student::new(
///   "123456".to_string(),
///   "John".to_string(),
///   "Doe".to_string(),
///   "john.doe@csxlabs.edu".to_string(),
///   11, // Junior
///   2026,
///   Some("Lower locker for wheelchair access".to_string()),
/// ).unwrap();
///
/// assert_eq!(student.grade_level(), "Junior");
/// assert_eq!(student.full_name(), "John Doe");
/// ```
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Student {
  pub id: StudentId,
  pub first_name: String,
  pub last_name: String,
  pub email: String,
  pub grade: u8,                              // 9-12 for high school grades
  pub graduation_year: u16,                   // e.g., 2025, 2026, etc.
  pub special_accommodations: Option<String>, // Any special needs for locker assignment
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>,
}

impl Student {
  /// Creates a new Student with validation.
  ///
  /// All fields are validated according to the rules specified in the struct documentation.
  /// Names are automatically trimmed and emails are normalized to lowercase.
  ///
  /// # Arguments
  /// * `id` - 6-digit student identifier (must be all digits)
  /// * `first_name` - Student's first name (1-50 characters after trimming)
  /// * `last_name` - Student's last name (1-50 characters after trimming)
  /// * `email` - Valid email address (5-254 characters)
  /// * `grade` - Grade level (9-12 for Freshman through Senior)
  /// * `graduation_year` - Expected graduation year (current year to current year + 10)
  /// * `special_accommodations` - Optional accessibility requirements (max 500 characters)
  ///
  /// # Returns
  /// * `Ok(Student)` - Successfully created and validated student
  /// * `Err(Error::UnprocessableEntity)` - Validation failures with detailed error messages
  ///
  /// # Examples
  /// ```
  /// use backend::student::Student;
  ///
  /// // Valid student
  /// let student = Student::new(
  ///   "123456".to_string(),
  ///   "Jane".to_string(),
  ///   "Smith".to_string(),
  ///   "jane.smith@csxlabs.edu".to_string(),
  ///   9, // Freshman
  ///   2028,
  ///   None,
  /// ).unwrap();
  ///
  /// // Student with accommodations
  /// let student_with_access = Student::new(
  ///   "654321".to_string(),
  ///   "Alex".to_string(),
  ///   "Johnson".to_string(),
  ///   "alex.johnson@csxlabs.edu".to_string(),
  ///   12, // Senior
  ///   2025,
  ///   Some("Bottom row locker for mobility aid access".to_string()),
  /// ).unwrap();
  ///
  /// assert_eq!(student.grade_level(), "Freshman");
  /// assert_eq!(student_with_access.grade_level(), "Senior");
  /// ```
  pub fn new(
    id: String,
    first_name: String,
    last_name: String,
    email: String,
    grade: u8,
    graduation_year: u16,
    special_accommodations: Option<String>,
  ) -> Result<Self, Error> {
    let mut errors = HashMap::new();

    // Validate student ID
    let student_id = match StudentId::new(id) {
      Ok(id) => Some(id),
      Err(Error::UnprocessableEntity { errors: id_errors }) => {
        errors.extend(id_errors);
        None
      }
      Err(e) => return Err(e),
    };

    // Validate first name
    if first_name.trim().is_empty() {
      errors
        .entry("first_name".into())
        .or_insert_with(Vec::new)
        .push("cannot be empty".into());
    } else if first_name.len() > 50 {
      errors
        .entry("first_name".into())
        .or_insert_with(Vec::new)
        .push("cannot be longer than 50 characters".into());
    }

    // Validate last name
    if last_name.trim().is_empty() {
      errors
        .entry("last_name".into())
        .or_insert_with(Vec::new)
        .push("cannot be empty".into());
    } else if last_name.len() > 50 {
      errors
        .entry("last_name".into())
        .or_insert_with(Vec::new)
        .push("cannot be longer than 50 characters".into());
    }

    // Validate email
    if !Self::is_valid_email(&email) {
      errors
        .entry("email".into())
        .or_insert_with(Vec::new)
        .push("must be a valid email address".into());
    }

    // Validate grade
    if !(9..=12).contains(&grade) {
      errors
        .entry("grade".into())
        .or_insert_with(Vec::new)
        .push("must be between 9 and 12".into());
    }

    // Validate graduation year (reasonable range)
    let current_year = chrono::Utc::now().year() as u16;
    if graduation_year < current_year || graduation_year > current_year + 10 {
      errors
        .entry("graduation_year".into())
        .or_insert_with(Vec::new)
        .push("must be within a reasonable range".into());
    }

    // Validate special accommodations if provided
    if let Some(ref accommodations_str) = special_accommodations {
      if accommodations_str.len() > 500 {
        errors
          .entry("special_accommodations".into())
          .or_insert_with(Vec::new)
          .push("cannot be longer than 500 characters".into());
      }
    }

    // If there are validation errors, return them
    if !errors.is_empty() {
      return Err(Error::UnprocessableEntity { errors });
    }

    let now = Utc::now();
    Ok(Student {
      id: student_id.unwrap(), // Safe to unwrap because we validated above
      first_name: first_name.trim().to_string(),
      last_name: last_name.trim().to_string(),
      email: email.trim().to_lowercase(),
      grade,
      graduation_year,
      special_accommodations,
      created_at: now,
      updated_at: now,
    })
  }

  pub fn full_name(&self) -> String {
    format!("{} {}", self.first_name, self.last_name)
  }

  pub fn grade_level(&self) -> String {
    match self.grade {
      9 => "Freshman".to_string(),
      10 => "Sophomore".to_string(),
      11 => "Junior".to_string(),
      12 => "Senior".to_string(),
      _ => format!("Grade {}", self.grade),
    }
  }

  pub fn update_email(&mut self, new_email: String) -> Result<(), Error> {
    if !Self::is_valid_email(&new_email) {
      return Err(Error::unprocessable_entity([(
        "email",
        "must be a valid email address",
      )]));
    }
    self.email = new_email.trim().to_lowercase();
    self.updated_at = Utc::now();
    Ok(())
  }

  pub fn update_grade(&mut self, new_grade: u8) -> Result<(), Error> {
    if !(9..=12).contains(&new_grade) {
      return Err(Error::unprocessable_entity([(
        "grade",
        "must be between 9 and 12",
      )]));
    }
    self.grade = new_grade;
    self.updated_at = Utc::now();
    Ok(())
  }

  pub fn update_graduation_year(&mut self, new_graduation_year: u16) -> Result<(), Error> {
    let current_year = chrono::Utc::now().year() as u16;
    if new_graduation_year < current_year || new_graduation_year > current_year + 10 {
      return Err(Error::unprocessable_entity([(
        "graduation_year",
        "must be within a reasonable range",
      )]));
    }
    self.graduation_year = new_graduation_year;
    self.updated_at = Utc::now();
    Ok(())
  }

  pub fn update_special_accommodations(
    &mut self,
    new_accommodations: Option<String>,
  ) -> Result<(), Error> {
    if let Some(ref accommodations_str) = new_accommodations {
      if accommodations_str.len() > 500 {
        return Err(Error::unprocessable_entity([(
          "special_accommodations",
          "cannot be longer than 500 characters",
        )]));
      }
    }
    self.special_accommodations = new_accommodations;
    self.updated_at = Utc::now();
    Ok(())
  }

  // Simple email validation - you might want to use a proper email validation crate
  fn is_valid_email(email: &str) -> bool {
    email.contains('@') && email.len() >= 5 && email.len() <= 254
  }
}

// Tests
#[cfg(test)]
mod tests {
  use super::*;
  use crate::init_logging;
  use log::debug;

  fn setup() {
    let _ = init_logging(); // Ignore error if already initialized
  }

  #[test]
  fn test_student_id_new() {
    setup();
    let id = StudentId::new("123456".to_string());
    assert_eq!(id.as_ref().unwrap().to_string(), "123456");
    debug!("id: {:?}", id);
  }

  #[test]
  fn test_student_id_new_invalid() {
    setup();
    let id = StudentId::new("12345".to_string());
    assert!(id.as_ref().is_err());
    debug!("error: {:?}", id.as_ref().err().unwrap());
  }

  #[test]
  fn test_student_new_valid() {
    setup();
    let student = Student::new(
      "123456".to_string(),
      "John".to_string(),
      "Doe".to_string(),
      "john.doe@csxlabs.edu".to_string(),
      11, // Junior
      2026,
      None,
    );

    assert!(student.is_ok());
    let student = student.unwrap();
    assert_eq!(student.id.to_string(), "123456");
    assert_eq!(student.first_name, "John");
    assert_eq!(student.last_name, "Doe");
    assert_eq!(student.email, "john.doe@csxlabs.edu");
    assert_eq!(student.grade, 11);
    assert_eq!(student.graduation_year, 2026);
    assert_eq!(student.special_accommodations, None);
    assert_eq!(student.full_name(), "John Doe");
    assert_eq!(student.grade_level(), "Junior");
    debug!("student: {:?}", student);
  }

  #[test]
  fn test_student_new_with_accommodations() {
    setup();
    let student = Student::new(
      "654321".to_string(),
      "Jane".to_string(),
      "Smith".to_string(),
      "jane.smith@csxlabs.edu".to_string(),
      9, // Freshman
      2028,
      Some("Lower locker for wheelchair accessibility".to_string()),
    );

    assert!(student.is_ok());
    let student = student.unwrap();
    assert_eq!(student.grade_level(), "Freshman");
    assert_eq!(
      student.special_accommodations,
      Some("Lower locker for wheelchair accessibility".to_string())
    );
    debug!("student with accommodations: {:?}", student);
  }

  #[test]
  fn test_student_new_invalid_grade() {
    setup();
    let student = Student::new(
      "123456".to_string(),
      "John".to_string(),
      "Doe".to_string(),
      "john.doe@csxlabs.edu".to_string(),
      8, // Invalid grade (should be 9-12)
      2025,
      None,
    );

    assert!(student.is_err());
    if let Err(Error::UnprocessableEntity { errors }) = student {
      assert!(errors.contains_key("grade"));
      debug!("grade validation errors: {:?}", errors);
    } else {
      panic!("Expected UnprocessableEntity error");
    }
  }

  #[test]
  fn test_student_update_grade() {
    setup();
    let mut student = Student::new(
      "123456".to_string(),
      "John".to_string(),
      "Doe".to_string(),
      "john.doe@csxlabs.edu".to_string(),
      10, // Sophomore
      2027,
      None,
    )
    .unwrap();

    let result = student.update_grade(11);
    assert!(result.is_ok());
    assert_eq!(student.grade, 11);
    assert_eq!(student.grade_level(), "Junior");
  }

  #[test]
  fn test_student_update_special_accommodations() {
    setup();
    let mut student = Student::new(
      "123456".to_string(),
      "John".to_string(),
      "Doe".to_string(),
      "john.doe@csxlabs.edu".to_string(),
      10,
      2027,
      None,
    )
    .unwrap();

    let result =
      student.update_special_accommodations(Some("Bottom row locker needed".to_string()));
    assert!(result.is_ok());
    assert_eq!(
      student.special_accommodations,
      Some("Bottom row locker needed".to_string())
    );

    // Test removing accommodations
    let result = student.update_special_accommodations(None);
    assert!(result.is_ok());
    assert_eq!(student.special_accommodations, None);
  }
}
