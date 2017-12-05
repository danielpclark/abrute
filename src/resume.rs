use std::fmt::Display;
use std::fs;
use std::path::Path;
use std::fs::{File,OpenOptions};
use std::fmt;
use std::io::{Read,Write,BufReader};
use std::convert::TryFrom;
use ::result::Error::MalformedResumeKey;
extern crate array_tool;
use self::array_tool::vec::Shift;
extern crate digits;
use self::digits::prelude::*;
use std::cmp::Ordering;

struct ResumeKeyDB {
  rkeys: Vec<ResumeKey>,
}

impl ResumeKeyDB {
  fn first(&self) -> Option<&ResumeKey> {
    self.rkeys.iter().next()
  }

  fn get(f: String) -> ResumeKeyDB {
    let file = File::open(f);
    assert!(file.is_ok());
    let mut buf_reader = BufReader::new(file.ok().unwrap());
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents).ok();

    let mut db: Vec<ResumeKey> = vec![];
    for item in contents.
        split("\n\n").
        map(|s| s.to_string()).
        filter(|s| !s.is_empty()).
        collect::<Vec<String>>() {
      let rk = ResumeKey::try_from(item);
      if let Ok(key) = rk {
        db.push(key);
      }
    }
    ResumeKeyDB { rkeys: db }
  }
}

#[derive(Debug,PartialEq)]
struct ResumeKey {
  characters: String,
  adjacent: Option<u8>,
  start: Digits,
  target: String,
}

impl ResumeKey {
  fn latest(self, db: ResumeKeyDB) -> ResumeKey {
    db.
      rkeys.
      into_iter().
      filter(|rk| rk.characters == self.characters).
      filter(|rk| rk.target == self.target).
      filter(|rk| rk.start >= self.start).
      max_by(|_,_| Ordering::Greater).
      unwrap_or(self)
  }
}

#[test]
fn can_pick_latest_resume_value() {
  let b = BaseCustom::<char>::new("asdf".chars().collect());
  let aa = Digits::new(b.clone(), "aaaa".to_string());
  let ab = Digits::new(b.clone(), "ssss".to_string());
  let ac = Digits::new(b.clone(), "dddd".to_string());
  let ad = Digits::new(b.clone(), "ffff".to_string());

  let ra = ResumeKey { characters: "asdf".to_string(), adjacent: None, start: aa, target: "thing".to_string() };
  let rb = ResumeKey { characters: "asdf".to_string(), adjacent: None, start: ab, target: "thing".to_string() };
  let rc = ResumeKey { characters: "asdf".to_string(), adjacent: None, start: ac, target: "thing".to_string() };
  let rd = ResumeKey { characters: "asdf".to_string(), adjacent: None, start: ad, target: "thing".to_string() };

  let db = ResumeKeyDB { rkeys: vec![ra,rd,rc] };

  assert_eq!(rb.latest(db).start.to_s(),"ffff");
}

impl TryFrom<String> for ResumeKey {
  type Error = ::result::Error;
  fn try_from(s: String) -> Result<ResumeKey, Self::Error> {
    let mut values: Vec<String> = s.split("\n").map(|s| s.to_string()).filter(|s| !s.is_empty()).collect();
    let c = values.shift().ok_or_else(|| return MalformedResumeKey)?;
    let a = values.shift().ok_or_else(|| return MalformedResumeKey).unwrap();
    let a = match a.parse::<u8>() {
      Ok(v) => Some(v),
      _ => None,
    };
    Ok(ResumeKey {
      characters: c.clone(),
      adjacent: a,
      start: Digits::new(
        BaseCustom::<char>::new(c.chars().collect()),
        values.shift().ok_or_else(|| return MalformedResumeKey)?
      ),
      target: values.shift().ok_or_else(|| return MalformedResumeKey)?,
    })
  }
}

impl Display for ResumeKey {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let a = if self.adjacent.is_some() {
      format!("{}", self.adjacent.unwrap())
    } else {
      "None".to_string()
    };
    write!(f,
           "{}\n{}\n{}\n{}\n\n",
           self.characters,
           a,
           self.start.to_s(),
           self.target
    )
  }
}

#[test]
fn input_and_output_for_resume_works() {
  let r = ResumeKey {
    characters: String::from("asdf"),
    adjacent: Some(1),
    start: Digits::new({BaseCustom::<char>::new("asdf".chars().collect())}, String::from("aaaa")),
    target: String::from("thing.aes")
  };

  let mut file = File::create(".example.res").ok().unwrap();
  file.write_all(&format!("{}", r).as_bytes()).ok();

  let keys = ResumeKeyDB::get(".example.res".to_string());
  let k = keys.first();
  assert_eq!(Some(&r), k); 

  let _a = fs::remove_file(".example.res");
  assert!(!Path::new(".example.res").exists(), "`.example.res` cleanup failed!");
}

#[derive(Debug)]
struct ResumeFile;

impl ResumeFile {
  pub(crate) fn save(&self, rk: &ResumeKey) {
    let file = OpenOptions::new().append(true).create(true).open(".abrute");
    assert!(file.is_ok(), "Failed to create or open file `.abrute` for resuming work!");

    if let Ok(mut f) = file {
      f.write_all(&format!("{}\n", rk).as_bytes()).ok();
      let _a = f.sync_data();
    }

    self.backup()
  }

  fn backup(&self) {
    let file = ".abrute";
    let backup = ".abrute.bak";
    fs::copy(file, backup).
      expect("Failed to backup `.abrute` to `.abrute.bak`!");
  }

  pub(crate) fn load(&self) -> ResumeKeyDB {
    let file = ".abrute";
    if Path::new(file).exists() {
      let db = ResumeKeyDB::get(file.to_owned());
      if !db.rkeys.is_empty() { return db; }
    }

    let backup = ".abrute.bak";
    if Path::new(backup).exists() {
      let db = ResumeKeyDB::get(file.to_owned());
      if !db.rkeys.is_empty() { return db; }
    }

    return ResumeKeyDB { rkeys: vec![] };
  }

  pub(crate) fn purge(&self) {
    let _a = fs::remove_file(".abrute");
    let _b = fs::remove_file(".abrute.bak");
  }
}

#[test]
fn it_backup_system_works() {
  let db_backup = ResumeFile {};

  let bc = BaseCustom::<char>::new("asdf".chars().collect());
  let aa = Digits::new(bc.clone(), "aaaa".to_string());
  let ab = Digits::new(bc.clone(), "ssss".to_string());

  let ra = ResumeKey { characters: "asdf".to_string(), adjacent: None, start: aa, target: "thing".to_string() };
  let rb = ResumeKey { characters: "asdf".to_string(), adjacent: None, start: ab, target: "thing".to_string() };

  db_backup.save(&ra);
  db_backup.save(&rb);

  assert!(Path::new(".abrute").exists(), "`.abrute` not found!");
  assert!(Path::new(".abrute.bak").exists(), "`.abrute.bak` not found!");

  let loaded_db = db_backup.load();

  assert!(loaded_db.rkeys.contains(&ra), "`.abrute` backup did not contain first key backup!");
  assert!(loaded_db.rkeys.contains(&rb), "`.abrute` backup did not contain second key backup!");

  db_backup.purge();

  assert!(!Path::new(".abrute").exists(), "`.abrute` cleanup failed!");
  assert!(!Path::new(".abrute.bak").exists(), "`.abrute.bak` cleanup failed!");
}
