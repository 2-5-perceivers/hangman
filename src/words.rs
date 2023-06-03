use std::collections::HashMap;

use raylib::get_random_value;

pub struct Words {
  pub words: Vec<&'static str>,
  pub processed_word: Option<ProcessedWord>,
}

pub struct ProcessedWord {
  pub secret: String,
  pub visible: String,
  pub id: HashMap<char, char>,
}

impl Words {
  pub fn new() -> Self {
    Self {
      words: vec![
        "pisica",
        "minge",
        "casa",
        "carte",
        "telefon",
        "masina",
        "copac",
        "animale",
        "flori",
        "apa",
        "bicicleta",
        "masa",
        "fotbal",
        "padure",
        "ferestre",
        "scoala",
        "copii",
        "strada",
        "nori",
        "plaja",
        "munte",
        "pasari",
        "cafea",
        "paine",
        "jurnal",
        "rucsac",
        "prieteni",
        "jocuri",
        "rau",
        "oglinda",
        "trandafiri",
        "vant",
        "lumanare",
        "soare",
        "paianjen",
        "iarba",
        "stele",
        "pietre",
        "toamna",
        "papuci",
        "perna",
        "vis",
        "ganduri",
        "umbrela",
        "balon",
        "penar",
        "fluturi",
        "cafea",
        "cheie",
        "baiat",
        "clasa",
        "fetita",
        "vara",
        "iarna",
        "primavara",
      ],
      processed_word: None,
    }
  }

  pub fn process_word(&mut self, index: usize) {
    let mut secret_word: String = self.words[index].into();
    let mut public_word: String = secret_word.clone();

    let mut map: HashMap<char, char> = HashMap::new();

    let index1 =
      get_random_value::<i32>(0, (secret_word.len() - 1) as i32) as usize;
    let mut index2 =
      get_random_value::<i32>(0, (secret_word.len() - 1) as i32) as usize;

    while index2 == index1
      || secret_word.chars().nth(index1).unwrap()
        == secret_word.chars().nth(index2).unwrap()
    {
      index2 =
        get_random_value::<i32>(0, (secret_word.len() - 1) as i32) as usize;
    }

    let c1 = secret_word.chars().nth(index1).unwrap();
    let c2 = secret_word.chars().nth(index2).unwrap();

    map.insert(c1, '1');
    map.insert(c2, '2');

    secret_word = secret_word.replace(c1, "1");
    secret_word = secret_word.replace(c2, "2");

    public_word = public_word.replace(c1, "_");
    public_word = public_word.replace(c2, "_");

    self.processed_word = Some(ProcessedWord {
      secret: secret_word,
      visible: public_word,
      id: map,
    });

    self.words.remove(index);
  }
}

impl ProcessedWord {
  fn replace_char(str: &mut String, pos: usize, replacement: &char) {
    if pos >= str.len() {
      return; // Return early if the position is out of bounds
    }

    let mut chars: Vec<char> = str.chars().collect();

    chars[pos] = *replacement;

    *str = chars.into_iter().collect();
  }

  pub fn try_char(&mut self, l: &char) -> LetterTryResult {
    if self.id.contains_key(l) {
      if self.secret.len() != self.visible.len() {
        panic!();
      }

      let secret_iter = self.secret.char_indices();

      for (i, c) in secret_iter {
        if c == self.id[l] {
          ProcessedWord::replace_char(&mut self.visible, i, l);
        }
      }

      self.id.remove(l);

      if self.id.is_empty() {
        return LetterTryResult::FINISHED;
      }

      LetterTryResult::GOOD
    } else {
      LetterTryResult::NUP
    }
  }
}

pub enum LetterTryResult {
  NUP,
  GOOD,
  FINISHED,
}
