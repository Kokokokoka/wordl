use miniz_oxide::inflate::decompress_to_vec;
use std::{
    collections::{hash_map::Entry, HashMap},
    env, fs,
    io::stdin,
    process,
};
const EXT: &str = "wordlc";
const DICT: &str = "./dict";
const HELP: &str =
    "in game: use !h or !hint to get hint\n !quit or !q to quit\n !help or !l or ! or !legend to \
                    get this message and info\n
you have to guess the correct word if you want to win this game\n m: means exact match\n o: means \
                    offset, you can't put this letter here anymore\n x: means wrong letter, you \
                    can't use words which contain this letter";

#[derive(PartialEq)]
enum Lords {
    Wrong,
    Right,
    Offset,
    Secret,
}
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Please, tell me what to do, either game or dict is supported\n jk there is (help|-h|-help) too ");
        println!("Starting thy game");
        interface()
    }
    match args[1].as_str() {
        "help" | "-h" | "-help" => println!("{HELP}"),
        "game" => interface(),
        _ => process::exit(1),
    }
}

fn interface() {
    let rd = fs::read_dir(DICT).unwrap();
    let mut lang_vec: Vec<String> = Vec::new();
    for path in rd {
        let pth = path.unwrap().path();
        let str_path = pth.file_name().unwrap().to_string_lossy();
        lang_vec.push(str_path.to_string());
        print!("{} ", str_path);
    }
    // lang
    lang_loop(&lang_vec);
}

fn check_correct(
    guess: &HashMap<char, Vec<usize>>,
    defender: &HashMap<char, Vec<usize>>,
    lord: Lords,
) -> bool {
    for (k, v) in defender {
        if guess.get(k).is_some() {
            if lord == Lords::Wrong {
                println!("This word doesn't contain letter {k}.");
                return false;
            } else if lord == Lords::Right {
                for num in v {
                    if !guess.get(k).unwrap().contains(num) {
                        println!("Letter number {} must be {k}", num + 1);
                        return false;
                    }
                }
            } else if lord == Lords::Offset {
                for num in v {
                    if guess.get(k).unwrap().contains(num) {
                        println!("Letter number {} can't be {k}", num + 1);
                        return false;
                    }
                }
            }
        } else {
            if lord == Lords::Right {
                println!("Letter number {} must be {k}", v[0] + 1); //v.iter().fold(String::new(), |acc, &num| acc + &(num +1).to_string() + ", "));
                return false;
            } else if lord == Lords::Offset {
                println!("There is  at least {} {}", v.len(), k);
                return false;
            }
        }
    }
    /*
    if !guess.keys().any(|k| defender.contains_key(k)) {
      match lord {
          Lords::Wrong => {
            for k in
          },
          Lords::Right => (),
          Lords::Offset => todo!(),
          Lords::Secret => todo!(),
      }
      return false;
    } */
    true
}

fn replace_nth_char(s: &str, idx: usize, newchar: char) -> String {
    s.chars()
        .enumerate()
        .map(|(i, c)| if i == idx { newchar } else { c })
        .collect()
}

fn string_to_wordls(word: &str) -> HashMap<char, Vec<usize>> {
    let mut worldls: HashMap<char, Vec<usize>> = HashMap::new();
    for (i, c1) in word.chars().enumerate() {
        match worldls.entry(c1) {
            Entry::Vacant(e) => {
                e.insert(vec![i]);
            }
            Entry::Occupied(mut e) => {
                if !e.get().contains(&i) {
                    e.get_mut().push(i);
                }
            }
        }
    }
    worldls
}

fn stdin_str() -> String {
    let mut inp: String = String::new();
    stdin().read_line(&mut inp).expect("BAD LINE");
    inp
}

fn lang_loop(lang_vec: &Vec<String>) {
    loop {
        println!("\nChoose lang (or enter !rand or !quit)");
        let lang: String = match stdin_str().trim().parse() {
            Ok(str) => str,
            Err(_) => continue,
        };
        //dbg!(lang.clone());
        let res = match lang.as_str() {
            "!rand" | "!r" => {
                let rand_lng: usize = fastrand::usize(0..lang_vec.len());
                &lang_vec[rand_lng]
            }
            "!quit" | "!q" => process::exit(0),
            "!hint" | "!h" => {
                println!("start thy game first");
                continue;
            }
            "!legend" | "!l" | "!help" => {
                println!("{HELP}");
                continue;
            }
            var => {
                if lang_vec.contains(&var.to_string()) {
                    var
                } else {
                    println!("Wrong lang");
                    continue;
                }
            }
        };
        println!("lang: {}", res.clone());
        let rd = fs::read_dir(format!("{DICT}/{}", res)).unwrap();
        let mut num_vec: Vec<String> = Vec::new();
        for path in rd {
            let pth = path.unwrap().path();
            if let Some(extension) = pth.extension() {
                if extension == EXT {
                    let str_path = pth.file_stem().unwrap().to_string_lossy();
                    num_vec.push(str_path.to_string());
                    print!("{} ", str_path);
                }
            }
        }
        // get length
        length_loop(&num_vec, &res);
    }
}

fn length_loop(num_vec: &Vec<String>, res: &str) {
    loop {
        println!("\nChoose length (or enter !rand or !quit)");
        let len: String = match stdin_str().trim().parse() {
            Ok(str) => str,
            Err(_) => continue,
        };
        //dbg!(len.clone());
        let ln = match len.as_str() {
            "!rand" | "!r" => {
                let rand_len: usize = fastrand::usize(0..num_vec.len());
                num_vec[rand_len].clone()
            }
            "!quit" | "!q" => process::exit(0),
            "!hint" | "!h" => {
                println!("start thy game first");
                continue;
            }
            "!legend" | "!l" | "!help" => {
                println!("{HELP}");
                continue;
            }
            var => {
                if num_vec.contains(&var.to_string()) {
                    var.to_string()
                } else {
                    println!("Wrong len or not a command");
                    continue;
                }
            }
        };
        println!("length: {}", &ln);
        let fp = format!("{DICT}/{res}/{ln}.{EXT}");

        // game
        game_loop(&fp);
    }
}

fn game_loop(fp: &str) {
    /*   let file = match File::open(&fp) {
        Err(why) => panic!("couldn't open {}: {}", fp, why),
        Ok(file) => file,
    }; */
    let file = fs::read(fp).expect("Unable to read file");
    let words_vec = decompress_to_vec(&file).expect("failed to decompress");
    let s = String::from_utf8_lossy(&words_vec);
    let words: Vec<&str> = s.lines().collect();
    //println!("result: {}", s);
    /*     let reader = BufReader::new(file);

    let words: Vec<String> = reader
        .lines()
        .map(|line| line.unwrap().parse::<String>().unwrap())
        .collect(); */
    //let words = ["aa"];
    let secret_number: usize = fastrand::usize(0..words.len());
    let secret_word = &words[secret_number];
    //let lordl: HashMap<char, Lordl> = HashMap::new();
    //let mut mordl: HashMap<char, Vec<usize>> = HashMap::new();
    let secret_lord = string_to_wordls(secret_word);
    let mut wrong_lord: HashMap<char, Vec<usize>> = HashMap::new();
    let mut match_lord: HashMap<char, Vec<usize>> = HashMap::new();
    let mut offset_lord: HashMap<char, Vec<usize>> = HashMap::new();
    /*
    dbg!(&secret_lord);
    dbg!(secret_number);
    dbg!(secret_word);
    */
    //let mut wrong_chars: String = String::new();
    let mut hint = "_".repeat(secret_word.chars().count());
    let mut wrong: String = "".to_string();
    loop {
        /*
        dbg!(&wrong_lord);
        dbg!(&match_lord);
        dbg!(&offset_lord);
        */
        //println!("hint: {hint}");
        //dbg!(&wrong_chars);
        let guess: String = match stdin_str().trim().parse() {
            Ok(str) => str,
            Err(_) => continue,
        };
        match guess.as_str() {
            "!quit" | "!q" => process::exit(0),
            "!d" => println!("{secret_word}"),
            "!hint" | "!h" => println!("known: {hint}\ncan't use: {wrong}"),
            "!legend" | "!l" | "!help" | "!" => {
                println!("{HELP}\n known: {hint}\n can't use: {wrong}")
            }
            //"!d" => println!("{secret_word}"),
            guess_word => {
                if words.contains(&guess_word) {
                    //right answer
                    if secret_word == &guess {
                        println!(
                            "You've guessed it, another one? Y/y/Yes/yes or anything else to stop"
                        );
                        let answer: String = match stdin_str().trim().parse() {
                            Ok(str) => str,
                            Err(_) => continue,
                        };

                        // another round
                        match answer.as_str() {
                            "Y" | "yes" | "Yes" | "y" => {
                                break;
                            }
                            _ => {
                                println!("Guess not, see you soon");
                                process::exit(0)
                            }
                        }
                    }
                    let guess_lord = string_to_wordls(guess_word);
                    if check_correct(&guess_lord, &match_lord, Lords::Right)
                        && check_correct(&guess_lord, &offset_lord, Lords::Offset)
                        && check_correct(&guess_lord, &wrong_lord, Lords::Wrong)
                        && check_correct(&guess_lord, &secret_lord, Lords::Secret)
                    {
                        for (k, guess_vec) in guess_lord {
                            // if we've guessed a letter
                            if secret_lord.contains_key(&k) {
                                // check position and count
                                // if cnt == 1 => trivial. off if pos differs, matches otherwise
                                let secret_vec = secret_lord.get(&k).unwrap();
                                // this is cool and all but an overkill for 1 elem
                                // if v.iter().all(|&x| vec_iter.any(|&item| item == x)) {
                                for pos in secret_vec {
                                    if guess_vec.contains(&pos) {
                                        // we have a strong match! time to populate the match_lord
                                        match match_lord.entry(k) {
                                            Entry::Vacant(e) => {
                                                e.insert(vec![*pos]);
                                            }
                                            Entry::Occupied(mut e) => {
                                                if !e.get().contains(&*pos) {
                                                    e.get_mut().push(*pos);
                                                }
                                            }
                                        }
                                        //match_lord.entry(k).or_insert(vec![pos]);
                                    }
                                }
                                let mut guess_filtered = guess_vec.clone();
                                let mut secret_filtered = secret_vec.clone();
                                if match_lord.contains_key(&k) {
                                    guess_filtered.retain(|element| {
                                        !match_lord.get(&k).unwrap().contains(element)
                                    });
                                    secret_filtered.retain(|element| {
                                        !match_lord.get(&k).unwrap().contains(element)
                                    });
                                }
                                //dbg!(&guess_filtered, &guess_vec, &secret_filtered);
                                for (pos, i) in secret_filtered.iter().enumerate() {
                                    if guess_filtered.len() <= pos {
                                        continue;
                                    }
                                    //dbg!(pos, i, &guess_filtered, &secret_filtered, &secret_lord);
                                    match offset_lord.entry(k) {
                                        Entry::Vacant(e) => {
                                            e.insert(vec![guess_filtered[pos]]);
                                        }
                                        Entry::Occupied(mut e) => {
                                            if !e.get().contains(i) {
                                                e.get_mut().push(guess_filtered[pos]);
                                            }
                                        }
                                    }
                                }
                                //assert!(v.iter().all(|&x| vec_iter.any(|&item| item == x)));
                                //secret_lord.get(&k).contains(&v);
                            } else {
                                // no such letter, we need to populate wrong letters
                                // in order to stop one from using words with those letters
                                // no need to check pos
                                wrong_lord.insert(k, vec![1]);
                            }
                        }
                        for (i, char) in guess_word.chars().enumerate() {
                            //dbg!(i, char);
                            if match_lord.contains_key(&char)
                                && match_lord.get(&char).unwrap().contains(&i)
                            {
                                if match_lord.get(&char).unwrap().contains(&i) {
                                    hint = replace_nth_char(&hint, i, char);
                                    print!("m");
                                }
                            } else if offset_lord.contains_key(&char) {
                                print!("o");
                            } else {
                                print!("x");
                            }
                        }
                        wrong = wrong_lord
                            .keys()
                            .map(|s| s.to_string())
                            .collect::<Vec<_>>()
                            .join(", ");
                        println!();
                    }
                    /*
                    if secret_word.chars().any(|c| has_chars(&c, &guess_word)) {
                      for ((i, guess_char), secret_char) in
                      guess_word.chars().enumerate().zip(secret_word.chars())
                      {
                        if guess_char == secret_char {
                          hint = replace_nth_char(&hint, i, guess_char);
                          print!("m");
                        } else if secret_word.contains(guess_char) {
                          if mordl.contains_key(&guess_char) {}
                          match mordl.entry(guess_char) {
                            Entry::Vacant(e) => {
                              e.insert(vec![i]);
                            }
                            Entry::Occupied(mut e) => {
                              e.get_mut().push(i);
                            }
                          }
                          print!("o")
                        } else {
                          wrong_chars = format!("{wrong_chars}{guess_char}");
                          wrong_chars = undpe_chars(&wrong_chars);
                          print!("x")
                        }
                      }
                      print!("\n");
                    } else {
                      wrong_chars = format!("{wrong_chars}{guess_word}");
                      wrong_chars = undpe_chars(&wrong_chars);
                      println!("{} no matches", "x".repeat(secret_word.len()))
                    } */
                } else {
                    println!("No such word or command");
                    continue;
                }
            }
        };
    }
}
/* fn _new_len_dict(dir: Result<DirEntry, Error>) {
  let mut file_path = dir.unwrap().path();
  file_path.push("index.dic_new");
  let file_display = file_path.display();
  let de = File::open(&file_path).unwrap(); //.expect(panic!("no dictionary for lang {}", file_display));
  let buf = BufReader::new(de);
  let sv = buf.lines().map(|l| l.expect("couldn't parse line"));
  println!("{file_display}");
  let lang_dict = _words_by_len(sv);
  for (k, v) in lang_dict {
    file_path.pop();
    file_path.push(format!("{}.wordl", k.to_string()));
    let file_display = file_path.display();
    let mut file = match File::create(&file_path) {
      Err(why) => panic!("couldn't create {}: {}", file_display, why),
      Ok(file) => file
    };
    for i in &v {
      match file.write_all(i.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", file_display, why),
        Ok(_) => ()
      }
    }
  }
} */

/* fn _words_by_len(sv: impl Iterator<Item = String>) -> HashMap<usize, Vec<String>> {
  let mut dict = HashMap::new();
  for i in sv {
    let mut ch = i.chars();
    let cnt = ch.clone().count();
    if cnt <= 1 {
      continue;
    }
    if ch.any(|c| !c.is_alphabetic() || c.is_uppercase()) {
      println!("{:?}", ch);
      continue;
    }
    let ni = format!("{i}\n");
    match dict.entry(cnt) {
      Entry::Vacant(e) => {
        e.insert(vec![ni]);
      }
      Entry::Occupied(mut e) => {
        e.get_mut().push(ni);
      }
    }
  }
  dict
} */

/* fn _dirs() {
  let rd = fs::read_dir(DICT).unwrap();
  for dir in rd {
    _new_len_dict(dir)
  }
} */
// word -> (letters + position)
// eg:
// once
// more
// xmxo -- no miss no ok
// oboe
// oxxo -- no second o
// obey -- can't use this word as it contains previous wrong chars + no guessed chars
// so, there is a restriction on input: len, word in hashset, char not in prev wrong, char + pos in prev right, char - [miss pos] in prev miss
// words -- hashset, random 0..len, word: hm? char: [count], [pos], wrong: enum {miss, wrong, right hm?}
// 1. in word => o or m
// 2. cnt > 1? {special mb} {if pos_w == pos_g => o} cnt -1
// 3. pos_w  != pos_g => m, cnt -1
