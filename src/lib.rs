#[derive(Eq, PartialEq, Debug)]
pub struct File<'a> {
    content: &'a str
}

#[derive(Eq, PartialEq, Debug)]
pub struct Diff<'a> {
    content: &'a str,
    file_points: Box<[File<'a>]>,
}

impl<'a> Diff<'a> {
    pub fn parse(content: &str) -> Diff {
        let mut upper_file_boundaries = Vec::new();
        let mut idx = 0;
        for line in content.lines() {
            if line.starts_with("+++ ") {
                upper_file_boundaries.push(idx);
            }
            idx += line.len() + 1;
        }

        let mut lower_file_boundaries = upper_file_boundaries[1..].to_vec();
        lower_file_boundaries.push(idx);

        let files = upper_file_boundaries.drain(..)
                                         .zip(lower_file_boundaries)
                                         .map(|(upper, lower)| {
                                             File {
                                                 content: &content[upper..lower],
                                             }
                                         })
                                         .collect::<Vec<_>>();

        Diff {
            content: content,
            file_points: files.into_boxed_slice(),
        }
    }


    pub fn files(&self) -> &[File] { &self.file_points }
}

impl<'a> File<'a> {
  pub fn name(&self) -> &str {
     &self.content.lines().nth(0).unwrap()[4..]
  }
}

#[test]
fn it_parses_a_git_diff_patch() {
    let patch = r###"diff --git a/src/lib.rs b/src/lib.rs
index a93251b..5315830 100644
--- a/src/lib.rs
+++ b/src/lib.rs
@@ -1,3 +1,3 @@
 #[test]
-fn it_works() {
+fn it_parses_a_git_patch() {
 }
diff --git a/Cargo.toml b/Cargo.toml
index 071b9ee..5dd607e 100644
--- a/Cargo.toml
+++ b/Cargo.toml
@@ -4,3 +4,6 @@ version = "0.1.0"
 authors = ["Bruno Tavares <connect+github@bltavares.com>"]
 
 [dependencies]
+
+
+
"###;

    let diff = Diff::parse(patch);

    assert_eq!(diff.files().len(), 2);
    assert_eq!(diff.files()[0].name(), "b/src/lib.rs");
}
