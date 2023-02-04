fn main() {}

#[cfg(test)]
mod test {
    use anyhow::Result;
    use std::{fs, path::Path};
    use trane::{course_library::CourseLibrary, Trane};

    #[test]
    fn verify_courses() -> Result<()> {
        let courses_path = Path::new("courses");
        let trane = Trane::new(&std::env::current_dir()?, Path::new("courses"))?;
        assert!(trane.get_all_exercise_ids()?.len() > 0);
        fs::remove_dir_all(courses_path.join(".trane"))?;
        Ok(())
    }
}
