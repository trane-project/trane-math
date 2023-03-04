fn main() {}

#[cfg(test)]
mod test {
    use anyhow::Result;
    use trane::{course_library::CourseLibrary, Trane};

    #[test]
    fn verify_courses() -> Result<()> {
        // Copy courses directory to a temp directory.
        let temp_dir = tempfile::tempdir()?;
        fs_extra::dir::copy(
            "courses",
            temp_dir.path(),
            &fs_extra::dir::CopyOptions::new(),
        )?;

        let trane = Trane::new(&temp_dir.path(), &temp_dir.path())?;
        assert!(trane.get_all_exercise_ids()?.len() > 0);
        Ok(())
    }
}
