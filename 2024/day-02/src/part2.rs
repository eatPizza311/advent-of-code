#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String> {
    todo!("part2");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        todo!("build test");
        let input = "";
        assert_eq!("", process(input)?);
        Ok(())
    }
}
