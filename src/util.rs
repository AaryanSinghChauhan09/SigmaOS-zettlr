//! zettlr model — markdown editor for academics.

#[derive(Debug, Clone)]
pub struct Note {
    pub id: String,
    pub title: String,
    pub content: String,
    pub tags: Vec<String>,
    pub references: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct Model {
    pub notes: Vec<Note>,
    pub next_id: u64,
}

impl Default for Model {
    fn default() -> Self { Self { notes: vec![], next_id: 1 } }
}

impl Model {
    pub fn new() -> Self { Self::default() }

    pub fn create_note(&mut self, title: &str, content: &str, tags: &[String]) -> String {
        let id = format!("{:x}", self.next_id);
        self.next_id += 1;
        self.notes.push(Note { id: id.clone(), title: title.into(), content: content.into(), tags: tags.to_vec(), references: vec![] });
        id
    }

    pub fn add_reference(&mut self, note_id: &str, reference: &str) -> bool {
        if let Some(n) = self.notes.iter_mut().find(|n| n.id == note_id) {
            n.references.push(reference.into());
            true
        } else { false }
    }

    pub fn list_notes(&self) -> &[Note] { &self.notes }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn note_reference() {
        let mut z = Model::new();
        let id = z.create_note("Paper", "Content", &["academic".into()]);
        assert!(z.add_reference(&id, "Smith 2020"));
    }
}
